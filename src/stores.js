import { writable } from 'svelte/store';

// Session management stores
export const currentSessionId = writable(null);
export const currentSessionConfig = writable(null);
export const sessionHistory = writable([]);

// Chat state
export const messages = writable([]);
export const isConnected = writable(false);
export const statusText = writable('🔴 Disconnected');

// Form state
export const sessionForm = writable({
    sessionName: '',
    directory: '',
    role: 'developer',
    model: '',
    temperature: 0.7,
    maxTokens: ''
});

// Streaming state
export const currentStreamingMessage = writable(null);
export const streamingBuffer = writable('');

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

// Auto-save session history to localStorage
sessionHistory.subscribe(value => {
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('octomind_sessions', JSON.stringify(value));
    }
});
