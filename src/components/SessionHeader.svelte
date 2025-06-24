<script>
    import { currentSessionConfig, currentSessionId, sessionHistory, statusText, isConnected, messages } from '../stores.js';
    import { initializeTauri } from '../lib/tauri.js';

    let sessionConfig = null;
    let sessionId = null;
    let sessions = [];
    let status = '🔴 Disconnected';

    currentSessionConfig.subscribe(value => {
        sessionConfig = value;
    });

    currentSessionId.subscribe(value => {
        sessionId = value;
    });

    sessionHistory.subscribe(value => {
        sessions = value;
    });

    statusText.subscribe(value => {
        status = value;
    });

    async function closeSession() {
        if (sessionId) {
            try {
                await initializeTauri();
                const { invoke } = window.__TAURI__.tauri;

                // Stop the session process
                await invoke('stop_session_process', {
                    sessionId: sessionId
                });
            } catch (error) {
                console.error('Failed to stop session process:', error);
            }
        }

        currentSessionId.set(null);
        currentSessionConfig.set(null);
        isConnected.set(false);
        statusText.set('🔴 Disconnected');
        messages.set([{
            type: 'system',
            content: 'Welcome to Octomind UI! Create a new session or resume an existing one to start chatting.'
        }]);
    }

    // Reactive values
    $: sessionName = sessionConfig ?
        (sessionConfig.name || sessionConfig.resume || 'New Session') :
        'No Session Active';

    $: sessionInfo = sessionConfig ?
        (() => {
            const sessionData = sessions.find(s => s.name === sessionName);
            const cost = sessionData?.totalCost > 0 ? ` • $${sessionData.totalCost.toFixed(5)}` : '';
            return `${sessionConfig.role} • ${sessionConfig.directory}${cost}`;
        })() :
        'Select or create a session to start';
</script>

<div class="session-header">
    <div>
        <h3>{sessionName}</h3>
        <small>{sessionInfo}</small>
    </div>
    <div>
        <span class="status-indicator">{status}</span>
        <button class="btn btn-secondary" on:click={closeSession}>
            ❌ Close Session
        </button>
    </div>
</div>
