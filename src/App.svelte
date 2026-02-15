<script>
    import { onMount, onDestroy } from 'svelte';
    import {
        connectionState,
        serverSessionId,
        currentSessionConfig,
        sessionHistory,
        messages,
        isThinking,
        thinkingContent,
        thinkingTokens,
        sessionCost,
        sessionTokens,
        sessionForm,
        addMessage,
        clearMessages,
        resetSession,
        isReady
    } from './stores.js';
    import * as websocket from './lib/websocket.js';
    import { startServer, stopServer, isServerRunning } from './lib/tauri.js';

    import SessionForm from './components/SessionForm.svelte';
    import SessionList from './components/SessionList.svelte';
    import SessionHeader from './components/SessionHeader.svelte';
    import ChatArea from './components/ChatArea.svelte';
    import MessageInput from './components/MessageInput.svelte';

    let serverPort = 8080;
    let serverStatus = 'stopped'; // 'stopped' | 'starting' | 'running'
    let connectionStatus = 'disconnected';

    // Handle WebSocket messages
    function handleWebSocketMessage(data) {
        console.log('WS Message:', data);
        
        switch (data.type) {
            case 'status':
                // Server status message
                if (data.session_id) {
                    serverSessionId.set(data.session_id);
                    // Auto-set session config if not already set (for auto-connect on mount)
                    currentSessionConfig.update(config => {
                        if (!config) {
                            console.log('Auto-setting session config from server session_id');
                            return {
                                name: data.session_id,
                                directory: '/workspace',
                                role: 'developer',
                                model: null,
                                temperature: 0.7,
                                maxTokens: null
                            };
                        }
                        console.log('Session config already set:', config);
                        return config;
                    });
                }
                // Only add status message if it has content
                if (data.content) {
                    addMessage('status', data.content);
                }
                break;
                
            case 'thinking':
                // AI is thinking
                isThinking.set(true);
                thinkingContent.set(data.content);
                if (data.meta?.tokens) {
                    thinkingTokens.set(data.meta.tokens);
                }
                break;
                
            case 'assistant':
                // AI response
                isThinking.set(false);
                addMessage('assistant', data.content);
                break;
                
            case 'cost':
                // Cost update
                if (data.meta) {
                    sessionCost.set(data.meta.session_cost || 0);
                    sessionTokens.set(data.meta.session_tokens || 0);
                }
                break;
                
            case 'error':
                addMessage('error', data.content);
                break;
        }
    }

    async function ensureServerRunning() {
        if (serverStatus === 'running') return true;
        
        // Try to start via Tauri (desktop app mode)
        try {
            await startServer(serverPort);
            serverStatus = 'running';
            return true;
        } catch (error) {
            // Tauri not available - web dev mode
            // Assume server is already running externally
            console.log('Tauri not available, assuming external server on port', serverPort);
            serverStatus = 'running';
            return true;
        }
    }

    async function connectToServer() {
        if (!await ensureServerRunning()) return;
        
        // If already connected, just re-register the handler
        if (websocket.isConnected()) {
            websocket.onMessage(handleWebSocketMessage);
            return;
        }
        
        connectionState.set('connecting');
        
        try {
            await websocket.connect(`ws://127.0.0.1:${serverPort}`, { autoReconnect: true });
            connectionState.set('connected');
            websocket.onMessage(handleWebSocketMessage);
        } catch (error) {
            console.error('Failed to connect:', error);
            connectionState.set('disconnected');
            addMessage('error', `Failed to connect to server: ${error}`);
        }
    }

    async function disconnectFromServer() {
        websocket.disconnect();
        connectionState.set('disconnected');
        serverSessionId.set(null);
    }

    onMount(async () => {
        // Check if server is already running
        try {
            const running = await isServerRunning();
            if (running) {
                serverStatus = 'running';
            }
        } catch (e) {
            // Ignore
        }
        
        // Don't auto-connect - wait for user to create/resume session
        // The connectToServer will be called when SessionForm dispatches 'connect'
    });

    onDestroy(async () => {
        await disconnectFromServer();
    });

    // Reactive: connection status from store
    $: connectionStatus = $connectionState;
</script>

<div class="app-container">
    <div class="sidebar">
        <div class="sidebar-header">
            <h1>🤖 Octomind</h1>
            <div class="connection-status {connectionStatus}">
                <span class="status-dot"></span>
                <span class="status-text">
                    {#if connectionStatus === 'connected'}
                        Connected
                    {:else if connectionStatus === 'connecting'}
                        Connecting...
                    {:else}
                        Disconnected
                    {/if}
                </span>
            </div>
        </div>
        
        <SessionForm on:connect={connectToServer} />
        <SessionList on:connect={connectToServer} />
    </div>

    <div class="main-content">
        <SessionHeader />
        <ChatArea />
        <MessageInput />
    </div>
</div>
