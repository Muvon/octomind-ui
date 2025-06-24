<script>
    import { currentSessionId, messages, isConnected } from '../stores.js';
    import { initializeTauri } from '../lib/tauri.js';

    let messageText = '';
    let sessionId = null;
    let connected = false;
    let inputDisabled = true;

    currentSessionId.subscribe(value => {
        sessionId = value;
        inputDisabled = !value;
    });

    isConnected.subscribe(value => {
        connected = value;
        inputDisabled = !value || !sessionId;
    });

    async function sendMessage() {
        if (!sessionId) {
            alert('No active session');
            return;
        }

        const message = messageText.trim();
        if (!message) return;

        // Add user message to chat
        messages.update(msgs => [...msgs, { type: 'user', content: message }]);
        messageText = '';
        inputDisabled = true;

        try {
            await initializeTauri();
            const { invoke } = window.__TAURI__.tauri;

            // Send message to the persistent session process
            await invoke('send_message_to_session', {
                sessionId: sessionId,
                message: message
            });

        } catch (error) {
            console.error('Failed to send message:', error);
            messages.update(msgs => [...msgs, { type: 'error', content: `Error: ${error}` }]);
        } finally {
            inputDisabled = false;
        }
    }

    function handleKeyPress(event) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
            sendMessage();
        }
    }
</script>

<div class="input-area">
    <div class="input-container">
        <input
            type="text"
            class="message-input"
            placeholder="Type your message..."
            bind:value={messageText}
            on:keypress={handleKeyPress}
            disabled={inputDisabled}
        >
        <button
            class="btn"
            on:click={sendMessage}
            disabled={inputDisabled}
        >
            💬 Send
        </button>
    </div>
</div>
