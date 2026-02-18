<script>
    import { currentSessionConfig, messages, connectionState, isThinking, isSending, addMessage } from '../stores.js';
    import * as websocket from '../lib/websocket.js';

    let messageText = '';

    // Reactive: input is disabled when no session, not connected, or thinking
    $: inputDisabled = !$currentSessionConfig || $connectionState !== 'connected' || $isThinking;

    async function sendMessage() {
        if (!$currentSessionConfig) {
            alert('No active session');
            return;
        }

        const message = messageText.trim();
        if (!message) return;

        // Add user message to chat
        addMessage('user', message);
        messageText = '';

        // Show sending animation
        isSending.set(true);

        try {
            // Check if it's a command (starts with /)
            if (message.startsWith('/')) {
                // Parse command: /command arg1 arg2 -> command='command', args=['arg1', 'arg2']
                const parts = message.slice(1).trim().split(/\s+/);
                const command = parts[0];
                const args = parts.slice(1);
                websocket.sendCommand(command, args, $currentSessionConfig.name);
            } else {
                // Regular message as message type
                websocket.send({ type: 'message', content: message }, $currentSessionConfig.name);
            }
        } catch (error) {
            console.error('Failed to send message:', error);
            addMessage('error', `Failed to send message: ${error}`);
            isSending.set(false);
        }
    }

    function handleKeyPress(event) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
            sendMessage();
        }
    }

    function handleKeyDown(event) {
        // Cmd/Ctrl + Enter to send
        if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
            event.preventDefault();
            sendMessage();
        }
    }
</script>

<div class="input-area">
    <div class="input-container">
        <textarea
            class="message-input"
            placeholder="Message..."
            bind:value={messageText}
            on:keypress={handleKeyPress}
            on:keydown={handleKeyDown}
            disabled={inputDisabled}
            rows="1"
        ></textarea>
        <button
            class="send-btn"
            on:click={sendMessage}
            disabled={inputDisabled}
        >
            {#if $isThinking}
                <span class="thinking-spinner" style="width: 14px; height: 14px;"></span>
            {:else}
                Send
            {/if}
        </button>
    </div>
</div>
