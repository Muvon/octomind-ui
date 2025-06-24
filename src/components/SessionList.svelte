<script>
    import { sessionHistory, currentSessionConfig, currentSessionId, messages, sessionForm } from '../stores.js';
    import { initializeTauri } from '../lib/tauri.js';

    let sessions = [];
    let currentConfig = null;

    sessionHistory.subscribe(value => {
        sessions = value;
    });

    currentSessionConfig.subscribe(value => {
        currentConfig = value;
    });

    async function resumeSessionFromHistory(sessionName) {
        const sessionData = sessions.find(s => s.name === sessionName);
        if (!sessionData) {
            messages.update(msgs => [...msgs, { type: 'error', content: `Session "${sessionName}" not found in history` }]);
            return;
        }

        // Set form values from history
        sessionForm.update(f => ({
            ...f,
            directory: sessionData.directory,
            role: sessionData.role,
            model: sessionData.model || '',
            temperature: sessionData.temperature || 0.7,
            maxTokens: sessionData.max_tokens || ''
        }));

        try {
            await initializeTauri();

            if (!window.__TAURI__ || !window.__TAURI__.core || !window.__TAURI__.core.invoke) {
                throw new Error('Tauri core.invoke not available');
            }

            const invoke = window.__TAURI__.core.invoke;

            messages.update(msgs => [...msgs, { type: 'system', content: `Resuming session: ${sessionName}...` }]);

            const sessionId = await invoke('resume_session_config', {
                sessionName: sessionName,
                directory: sessionData.directory,
                model: sessionData.model,
                temperature: sessionData.temperature,
                maxTokens: sessionData.max_tokens,
                role: sessionData.role
            });

            console.log('Session resumed with ID:', sessionId);
            currentSessionId.set(sessionId);

            const sessionInfo = await invoke('get_session_info', {
                sessionId: sessionId
            });

            currentSessionConfig.set(sessionInfo);
            messages.update(msgs => [...msgs, { type: 'system', content: `✅ Session resumed: ${sessionName} in ${sessionData.directory}` }]);

            // Update last used time
            sessionHistory.update(history => {
                const existingIndex = history.findIndex(s => s.name === sessionName);
                if (existingIndex >= 0) {
                    history[existingIndex].lastUsed = new Date().toISOString();
                }
                return history;
            });

        } catch (error) {
            console.error('Failed to resume session:', error);
            messages.update(msgs => [...msgs, { type: 'error', content: `❌ Failed to resume session: ${error}` }]);
        }
    }

    function removeSessionFromHistory(sessionName) {
        if (confirm(`Are you sure you want to delete session "${sessionName}"?`)) {
            sessionHistory.update(history => history.filter(s => s.name !== sessionName));

            // If this was the current session, close it
            if (currentConfig &&
                (currentConfig.name === sessionName || currentConfig.resume === sessionName)) {
                closeSession();
            }

            messages.update(msgs => [...msgs, { type: 'system', content: `🗑️ Session "${sessionName}" deleted` }]);
        }
    }

    function clearAllSessions() {
        if (confirm('Are you sure you want to delete ALL sessions? This cannot be undone.')) {
            sessionHistory.set([]);

            // Close current session if any
            if (currentConfig) {
                closeSession();
            }

            messages.update(msgs => [...msgs, { type: 'system', content: '🗑️ All sessions cleared' }]);
        }
    }

    function closeSession() {
        // This will be handled by the parent component
        currentSessionId.set(null);
        currentSessionConfig.set(null);
    }

    function isActiveSession(sessionName) {
        return currentConfig &&
            (currentConfig.name === sessionName || currentConfig.resume === sessionName);
    }
</script>

<div class="session-list">
    <h3>📚 Existing Sessions</h3>

    <div id="sessions-container">
        {#if sessions.length === 0}
            <div class="loading">No sessions found. Create a new session to get started.</div>
        {:else}
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                <span style="font-size: 12px; opacity: 0.8;">{sessions.length} sessions</span>
                <button
                    class="btn btn-secondary"
                    on:click={clearAllSessions}
                    style="font-size: 11px; padding: 4px 8px;"
                >
                    🗑️ Clear All
                </button>
            </div>

            {#each sessions as session}
                {@const isActive = isActiveSession(session.name)}
                {@const lastUsed = new Date(session.lastUsed).toLocaleDateString()}
                {@const cost = session.totalCost > 0 ? `$${session.totalCost.toFixed(5)}` : '$0.00'}

                <div class="session-item {isActive ? 'active' : ''}" style="position: relative;">
                    <div
                        on:click={() => resumeSessionFromHistory(session.name)}
                        style="cursor: pointer; flex: 1;"
                        role="button"
                        tabindex="0"
                        on:keydown={(e) => e.key === 'Enter' && resumeSessionFromHistory(session.name)}
                    >
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <div style="flex: 1; min-width: 0;">
                                <div style="font-weight: 600; margin-bottom: 4px; font-size: 13px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                    {session.name}
                                </div>
                                <div style="font-size: 11px; opacity: 0.8;">
                                    {session.role} • {lastUsed}
                                </div>
                                <div style="font-size: 10px; opacity: 0.6; margin-top: 2px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                    {session.directory}
                                </div>
                            </div>
                            <div style="text-align: right; margin-left: 8px;">
                                <div style="font-size: 12px; color: #8bc34a; font-weight: 600;">
                                    {cost}
                                </div>
                                <div style="font-size: 9px; opacity: 0.6; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 60px;">
                                    {session.model}
                                </div>
                            </div>
                        </div>
                    </div>
                    <button
                        on:click|stopPropagation={() => removeSessionFromHistory(session.name)}
                        class="delete-btn"
                        style="position: absolute; top: 4px; right: 4px; background: rgba(255, 0, 0, 0.2); border: 1px solid rgba(255, 0, 0, 0.3); color: #ff6b6b; border-radius: 4px; width: 20px; height: 20px; font-size: 10px; cursor: pointer; display: flex; align-items: center; justify-content: center;"
                        title="Delete session"
                    >
                        ×
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>
