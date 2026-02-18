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
    <h3>Sessions</h3>

    <div id="sessions-container">
        {#if $sessionHistory.length === 0}
            <div class="loading">No sessions yet</div>
        {:else}
            {#each $sessionHistory as session}
                {@const isActive = isActiveSession(session.name)}
                
                <div class="session-item {isActive ? 'active' : ''}">
                    <div
                        on:click={() => resumeSessionFromHistory(session.name)}
                        style="cursor: pointer;"
                        role="button"
                        tabindex="0"
                        on:keydown={(e) => e.key === 'Enter' && resumeSessionFromHistory(session.name)}
                    >
                        <div class="session-item-name">{session.name}</div>
                        <div class="session-item-meta">{session.directory}</div>
                    </div>
                    <button
                        on:click|stopPropagation={() => removeSessionFromHistory(session.name)}
                        class="delete-btn"
                        title="Delete"
                    >×</button>
                </div>
            {/each}
        {/if}
    </div>
</div>
