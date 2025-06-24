<script>
    import { onMount } from 'svelte';
    import {
        currentSessionId,
        currentSessionConfig,
        sessionHistory,
        messages,
        statusText,
        isConnected,
        currentStreamingMessage,
        streamingBuffer
    } from './stores.js';
    import { initializeTauri } from './lib/tauri.js';

    import SessionForm from './components/SessionForm.svelte';
    import SessionList from './components/SessionList.svelte';
    import SessionHeader from './components/SessionHeader.svelte';
    import ChatArea from './components/ChatArea.svelte';
    import MessageInput from './components/MessageInput.svelte';

    let sessionId = null;
    let sessionConfig = null;

    currentSessionId.subscribe(value => {
        sessionId = value;
        if (value) {
            enableChat();
        }
    });

    currentSessionConfig.subscribe(value => {
        sessionConfig = value;
    });

    async function enableChat() {
        if (!sessionId) return;

        statusText.set('🟡 Starting...');

        try {
            await initializeTauri();

            if (!window.__TAURI__ || !window.__TAURI__.core || !window.__TAURI__.core.invoke) {
                throw new Error('Tauri core.invoke not available');
            }

            const invoke = window.__TAURI__.core.invoke;

            // Start the session process
            await invoke('start_session_process', {
                sessionId: sessionId
            });

            statusText.set('🟢 Connected');
            isConnected.set(true);
            messages.update(msgs => [...msgs, {
                type: 'system',
                content: '🚀 Session process started. You can now chat with Octomind!'
            }]);

        } catch (error) {
            console.error('Failed to start session process:', error);
            messages.update(msgs => [...msgs, {
                type: 'error',
                content: `❌ Failed to start session process: ${error}`
            }]);
            statusText.set('🔴 Error');
            isConnected.set(false);
        }
    }

    async function setupSessionEventListeners() {
        try {
            await initializeTauri();
            const { listen } = window.__TAURI__.event;

            // Listen for session output (stdout/stderr)
            await listen('session_output', (event) => {
                const { session_id, type, content } = event.payload;

                // Only handle output for the current session
                if (session_id === sessionId) {
                    addStreamingOutput(content, type);
                }
            });

            // Listen for session ended events
            await listen('session_ended', (event) => {
                const { session_id } = event.payload;

                if (session_id === sessionId) {
                    messages.update(msgs => [...msgs, {
                        type: 'system',
                        content: '🔴 Session process ended'
                    }]);
                    statusText.set('🔴 Disconnected');
                    isConnected.set(false);
                }
            });

            console.log('Session event listeners setup complete');
        } catch (error) {
            console.error('Failed to setup session event listeners:', error);
        }
    }

    function addStreamingOutput(content, type) {
        // Extract cost information if present
        const costMatch = content.match(/cost:\s*\$(\d+\.?\d*)/i);
        if (costMatch) {
            const cost = parseFloat(costMatch[1]);
            const sessionName = sessionConfig?.name || sessionConfig?.resume;
            if (sessionName && cost > 0) {
                updateSessionCost(sessionName, cost);
                console.log(`Updated session ${sessionName} cost: $${cost}`);
            }
        }

        // If this is the first output for a new response, create a new streaming message
        if (!$currentStreamingMessage) {
            currentStreamingMessage.set(true);
            streamingBuffer.set('');
        }

        // Add content to buffer
        streamingBuffer.update(buffer => buffer + content + '\n');

        // If this looks like end of response (empty line or session prompt), finalize
        if (content.trim() === '' || content.includes('> ') || content.includes('Session saved')) {
            finalizeStreamingMessage();
        }
    }

    function finalizeStreamingMessage() {
        if ($currentStreamingMessage) {
            // Add the final streaming message to the messages list
            messages.update(msgs => [...msgs, {
                type: 'assistant',
                content: $streamingBuffer
            }]);

            currentStreamingMessage.set(null);
            streamingBuffer.set('');
        }
    }

    function updateSessionCost(sessionName, cost) {
        sessionHistory.update(history => {
            const session = history.find(s => s.name === sessionName);
            if (session) {
                session.totalCost = cost;
                session.lastUsed = new Date().toISOString();
            }
            return history;
        });
    }

    onMount(async () => {
        console.log('=== Initializing App ===');

        // Initialize welcome message
        messages.set([{
            type: 'system',
            content: '🚀 Welcome to <strong>Octomind UI</strong>! Create a new session or resume an existing one to start chatting with your AI development assistant.'
        }]);

        // Setup event listeners for session streaming
        await setupSessionEventListeners();

        console.log('App initialized successfully');
    });
</script>

<div class="container">
    <div class="sidebar">
        <h2>🤖 Octomind Sessions</h2>
        <SessionForm />
        <SessionList />
    </div>

    <div class="main-content">
        <SessionHeader />
        <ChatArea />
        <MessageInput />
    </div>
</div>
