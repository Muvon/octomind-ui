<script>
    import { messages, isThinking, thinkingContent, thinkingTokens, sessionCost, sessionTokens } from '../stores.js';
    import { formatMessage } from '../lib/tauri.js';
    import { afterUpdate } from 'svelte';

    let chatArea;

    // Auto-scroll to bottom when new messages arrive
    afterUpdate(() => {
        if (chatArea) {
            chatArea.scrollTop = chatArea.scrollHeight;
        }
    });
</script>

<div class="chat-area" bind:this={chatArea}>
    {#if $messages.length === 0}
        <div class="empty-state">
            <div class="empty-state-icon">🤖</div>
            <div class="empty-state-text">
                Welcome to <strong>Octomind</strong>! Create a new session to start chatting with your AI development assistant.
            </div>
        </div>
    {:else}
        {#each $messages as message}
            <div class="message {message.type}">
                {#if message.type === 'thinking'}
                    <div class="thinking-header">
                        <div class="thinking-spinner"></div>
                        <span>Thinking...</span>
                        {#if $thinkingTokens > 0}
                            <span class="thinking-tokens">{$thinkingTokens} tokens</span>
                        {/if}
                    </div>
                    <div class="thinking-content">
                        {@html formatMessage(message.content)}
                    </div>
                {:else}
                    {@html formatMessage(message.content)}
                {/if}
            </div>
        {/each}
    {/if}

    {#if $isThinking && $thinkingContent}
        <div class="message thinking">
            <div class="thinking-header">
                <div class="thinking-spinner"></div>
                <span>Thinking...</span>
                {#if $thinkingTokens > 0}
                    <span class="thinking-tokens">{$thinkingTokens} tokens</span>
                {/if}
            </div>
            <div class="thinking-content">
                {@html formatMessage($thinkingContent)}
            </div>
        </div>
    {/if}
</div>
