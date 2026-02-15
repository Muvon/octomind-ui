<script>
    import { sessionHistory, currentSessionConfig, sessionForm, addMessage, connectionState } from '../stores.js';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    async function resumeSessionFromHistory(sessionName) {
        const sessionData = $sessionHistory.find(s => s.name === sessionName);
        if (!sessionData) {
            addMessage('error', `Session "${sessionName}" not found in history`);
            return;
        }

        // Set form values from history
        sessionForm.set({
            sessionName: '',
            directory: sessionData.directory,
            role: sessionData.role,
            model: sessionData.model || '',
            temperature: sessionData.temperature || 0.7,
            maxTokens: sessionData.max_tokens || ''
        });

        // Set current session config
        currentSessionConfig.set({
            name: sessionName,
            directory: sessionData.directory,
            role: sessionData.role,
            model: sessionData.model || null,
            temperature: sessionData.temperature || 0.7,
            maxTokens: sessionData.max_tokens || null
        });

        // Update last used time
        sessionHistory.update(history => {
            const existingIndex = history.findIndex(s => s.name === sessionName);
            if (existingIndex >= 0) {
                history[existingIndex].lastUsed = new Date().toISOString();
            }
            return history;
        });

        addMessage('status', `✅ Session resumed: ${sessionName}`);
        dispatch('connect');
    }

    function removeSessionFromHistory(sessionName) {
        if (confirm(`Are you sure you want to delete session "${sessionName}"?`)) {
            sessionHistory.update(history => history.filter(s => s.name !== sessionName));

            // If this was the current session, close it
            if ($currentSessionConfig?.name === sessionName) {
                currentSessionConfig.set(null);
            }

            addMessage('status', `🗑️ Session "${sessionName}" deleted`);
        }
    }

    function clearAllSessions() {
        if (confirm('Are you sure you want to delete ALL sessions? This cannot be undone.')) {
            sessionHistory.set([]);
            if ($currentSessionConfig) {
                currentSessionConfig.set(null);
            }
            addMessage('status', '🗑️ All sessions cleared');
        }
    }

    function isActiveSession(sessionName) {
        return $currentSessionConfig?.name === sessionName;
    }
</script>

<div class="session-list">
    <h3>📚 Existing Sessions</h3>

    <div id="sessions-container">
        {#if $sessionHistory.length === 0}
            <div class="loading">No sessions found. Create a new session to get started.</div>
        {:else}
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                <span style="font-size: 12px; opacity: 0.8;">{$sessionHistory.length} sessions</span>
                <button
                    class="btn btn-secondary"
                    on:click={clearAllSessions}
                    style="font-size: 11px; padding: 4px 8px;"
                >
                    🗑️ Clear All
                </button>
            </div>

            {#each $sessionHistory as session}
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
                                <div class="session-item-name">
                                    {session.name}
                                </div>
                                <div class="session-item-meta">
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
                        title="Delete session"
                    >
                        ×
                    </button>
                </div>
            {/each}
        {/if}
    </div>
</div>
