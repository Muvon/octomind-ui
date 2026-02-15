import { writable, derived } from 'svelte/store';

// Connection state
export const connectionState = writable('disconnected'); // 'disconnected' | 'connecting' | 'connected'
export const serverSessionId = writable(null);

// Session management
export const currentSessionConfig = writable(null);
export const sessionHistory = writable([]);

// Chat state
export const messages = writable([]);
export const isThinking = writable(false);
export const thinkingContent = writable('');
export const thinkingTokens = writable(0);

// Cost tracking
export const sessionCost = writable(0);
export const sessionTokens = writable(0);

// Form state
export const sessionForm = writable({
    sessionName: '',
    directory: '',
    role: 'developer',
    model: '',
    temperature: 0.7,
    maxTokens: ''
});

// Derived: is connected and ready
export const isReady = derived(
    [connectionState, serverSessionId],
    ([$connectionState, $serverSessionId]) => 
        $connectionState === 'connected' && $serverSessionId !== null
);

// Initialize session history from localStorage
if (typeof localStorage !== 'undefined') {
    const stored = localStorage.getItem('octomind_sessions');
    if (stored) {
        try {
            sessionHistory.set(JSON.parse(stored));
        } catch (e) {
            console.error('Failed to parse session history:', e);
        }
    }
}

// Auto-save session history
sessionHistory.subscribe(value => {
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('octomind_sessions', JSON.stringify(value));
    }
});

// Helper: Add message to chat
export function addMessage(type, content, meta = {}) {
    messages.update(msgs => [...msgs, {
        id: crypto.randomUUID(),
        type,
        content,
        timestamp: Date.now(),
        ...meta
    }]);
}

// Helper: Clear messages
export function clearMessages() {
    messages.set([]);
}

// Helper: Update session cost
export function updateCost(cost, tokens) {
    sessionCost.set(cost);
    sessionTokens.set(tokens);
    
    // Also update session history
    currentSessionConfig.subscribe(config => {
        if (config?.name) {
            sessionHistory.update(history => {
                const session = history.find(s => s.name === config.name);
                if (session) {
                    session.totalCost = cost;
                    session.totalTokens = tokens;
                    session.lastUsed = new Date().toISOString();
                }
                return history;
            });
        }
    })();
}

// Helper: Reset session state
export function resetSession() {
    currentSessionConfig.set(null);
    serverSessionId.set(null);
    clearMessages();
    sessionCost.set(0);
    sessionTokens.set(0);
    isThinking.set(false);
    thinkingContent.set('');
    thinkingTokens.set(0);
}
