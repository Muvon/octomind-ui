// WebSocket connection manager for octomind server

let ws = null;
let reconnectTimer = null;
let messageHandler = null;

export function connect(url = 'ws://127.0.0.1:8080', options = {}) {
    return new Promise((resolve, reject) => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            resolve(ws);
            return;
        }

        ws = new WebSocket(url);

        ws.onopen = () => {
            console.log('WebSocket connected to octomind server');
            if (reconnectTimer) {
                clearTimeout(reconnectTimer);
                reconnectTimer = null;
            }
            resolve(ws);
        };

        ws.onmessage = (event) => {
            console.log('WS RAW:', event.data);
            try {
                const data = JSON.parse(event.data);
                if (messageHandler) {
                    messageHandler(data);
                }
            } catch (err) {
                console.error('Failed to parse WebSocket message:', err, 'Raw:', event.data.slice(0, 200));
                // Don't call handler on parse error to avoid duplicate messages
            }
        };

        ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            reject(error);
        };

        ws.onclose = () => {
            console.log('WebSocket disconnected');
            ws = null;
            
            // Auto-reconnect after 3 seconds
            if (options.autoReconnect !== false) {
                reconnectTimer = setTimeout(() => {
                    connect(url, options).catch(() => {});
                }, 3000);
            }
        };
    });
}

export function disconnect() {
    if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
    }
    if (ws) {
        ws.close();
        ws = null;
    }
    messageHandler = null;
}

export function send(message, sessionId = null) {
    if (!ws || ws.readyState !== WebSocket.OPEN) {
        throw new Error('WebSocket not connected');
    }
    
    const payload = typeof message === 'string' 
        ? { type: 'message', content: message }
        : message;
    
    // Add session_id if provided
    if (sessionId) {
        payload.session_id = sessionId;
    }
    
    ws.send(JSON.stringify(payload));
}

export function sendSession(sessionId) {
    if (!ws || ws.readyState !== WebSocket.OPEN) {
        throw new Error('WebSocket not connected');
    }
    ws.send(JSON.stringify({ type: 'session', session_id: sessionId }));
}

export function sendCommand(command, args = [], sessionId = null) {
    if (!ws || ws.readyState !== WebSocket.OPEN) {
        throw new Error('WebSocket not connected');
    }
    const payload = { type: 'command', command, args };
    if (sessionId) {
        payload.session_id = sessionId;
    }
    ws.send(JSON.stringify(payload));
}

export function onMessage(handler) {
    // Replace handler instead of adding to prevent duplicates
    messageHandler = handler;
}

export function isConnected() {
    return ws && ws.readyState === WebSocket.OPEN;
}

export function getConnectionState() {
    if (!ws) return 'disconnected';
    switch (ws.readyState) {
        case WebSocket.CONNECTING: return 'connecting';
        case WebSocket.OPEN: return 'connected';
        case WebSocket.CLOSING: return 'closing';
        case WebSocket.CLOSED: return 'disconnected';
        default: return 'unknown';
    }
}