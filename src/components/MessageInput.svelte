<script>
    import { currentSessionConfig, messages, connectionState, isThinking, addMessage } from '../stores.js';
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

        try {
            // Send via WebSocket
            websocket.send({ type: 'input', content: message });
        } catch (error) {
            console.error('Failed to send message:', error);
            addMessage('error', `Failed to send message: ${error}`);
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
            placeholder="Type your message... (Enter to send, Shift+Enter for new line)"
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
                💬
            {/if}
            Send
        </button>
    </div>
</div>
