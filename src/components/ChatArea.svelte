<script>
    import { messages, currentStreamingMessage, streamingBuffer } from '../stores.js';
    import { formatMessage } from '../lib/tauri.js';
    import { afterUpdate } from 'svelte';

    let chatArea;
    let messageList = [];
    let streamingMsg = null;
    let streamingContent = '';

    messages.subscribe(value => {
        messageList = value;
    });

    currentStreamingMessage.subscribe(value => {
        streamingMsg = value;
    });

    streamingBuffer.subscribe(value => {
        streamingContent = value;
    });

    // Auto-scroll to bottom when new messages arrive
    afterUpdate(() => {
        if (chatArea) {
            chatArea.scrollTop = chatArea.scrollHeight;
        }
    });
</script>

<div class="chat-area" bind:this={chatArea}>
    {#if messageList.length === 0}
        <div class="message system">
            🚀 Welcome to <strong>Octomind UI</strong>! Create a new session or resume an existing one to start chatting with your AI development assistant.
        </div>
    {:else}
        {#each messageList as message}
            <div class="message {message.type}">
                {@html formatMessage(message.content)}
            </div>
        {/each}
    {/if}

    {#if streamingMsg}
        <div class="message assistant streaming">
            {@html streamingContent}
        </div>
    {/if}
</div>
