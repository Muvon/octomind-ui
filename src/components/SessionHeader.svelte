<script>
    import { currentSessionConfig, sessionHistory, connectionState, sessionCost, sessionTokens, resetSession, addMessage } from '../stores.js';
    import { disconnect } from '../lib/websocket.js';

    async function closeSession() {
        disconnect();
        resetSession();
        addMessage('status', 'Session closed. Create a new session to continue.');
    }

    // Reactive values using $ prefix
    $: sessionName = $currentSessionConfig ?
        ($currentSessionConfig.name || 'New Session') :
        'No Session Active';

    $: sessionInfo = $currentSessionConfig ?
        `${$currentSessionConfig.role} • ${$currentSessionConfig.directory}` :
        'Select or create a session to start';

    $: costDisplay = $sessionCost > 0 ? `$${$sessionCost.toFixed(5)}` : '';
    $: tokensDisplay = $sessionTokens > 0 ? `${$sessionTokens.toLocaleString()} tokens` : '';
</script>

<div class="session-header">
    <div class="session-header-info">
        <h3>{sessionName}</h3>
        <small>{sessionInfo}</small>
    </div>
    <div class="session-header-actions">
        {#if $sessionCost > 0}
            <div class="cost-badge">
                {costDisplay}
                {#if $sessionTokens > 0}
                    <span style="opacity: 0.7; margin-left: 8px;">• {tokensDisplay}</span>
                {/if}
            </div>
        {/if}
        <button class="btn btn-secondary" on:click={closeSession}>
            ❌ Close Session
        </button>
    </div>
</div>
