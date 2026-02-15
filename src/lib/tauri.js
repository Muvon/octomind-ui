// Tauri API utilities
export async function initializeTauri() {
    console.log('Initializing Tauri...');
    return new Promise((resolve) => {
        if (window.__TAURI__) {
            console.log('Tauri APIs available');
            resolve();
        } else {
            console.log('Waiting for Tauri APIs...');
            window.addEventListener('DOMContentLoaded', () => {
                if (window.__TAURI__) {
                    console.log('Tauri APIs available after DOMContentLoaded');
                    resolve();
                } else {
                    console.log('Tauri APIs still not available, waiting...');
                    setTimeout(() => {
                        if (window.__TAURI__) {
                            console.log('Tauri APIs available after timeout');
                        } else {
                            console.log('Tauri APIs still not available after timeout');
                        }
                        resolve();
                    }, 500);
                }
            });
        }
    });
}

export async function getCurrentDirectory() {
    try {
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            return process.cwd() || '/home/box/work/muvon/octomind';
        }

        const dirs = await window.__TAURI__.core.invoke('list_directories');
        return dirs[0] || '';
    } catch (error) {
        console.error('Failed to get current directory:', error);
        return '/home/box/work/muvon/octomind';
    }
}

export async function selectDirectory() {
    try {
        console.log('=== Testing Directory Selection ===');

        if (window.__TAURI__ && window.__TAURI__.core) {
            const selected = await window.__TAURI__.core.invoke('select_directory_native');
            if (selected && selected !== null) {
                return selected;
            }
        }

        throw new Error('No directory selection method available');

    } catch (error) {
        console.error('Failed to select directory:', error);
        throw error;
    }
}

export function generateSessionName(directory, customName = null) {
    if (customName && customName.trim()) {
        return customName.trim();
    }

    const basename = directory.split('/').pop() || directory.split('\\\\').pop() || 'unknown';

    const now = new Date();
    const ymd = now.getFullYear().toString() +
               (now.getMonth() + 1).toString().padStart(2, '0') +
               now.getDate().toString().padStart(2, '0');
    const hms = now.getHours().toString().padStart(2, '0') +
               now.getMinutes().toString().padStart(2, '0') +
               now.getSeconds().toString().padStart(2, '0');

    return `ui-${ymd}-${hms}-${basename}`;
}

export function formatMessage(content) {
    if (typeof content === 'string') {
        return content
            .replace(/```(\w*)\n?([\s\S]*?)```/g, '<pre><code class="language-$1">$2</code></pre>')
            .replace(/`([^`]+)`/g, '<code>$1</code>')
            .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
            .replace(/\*([^*]+)\*/g, '<em>$1</em>')
            .replace(/\n/g, '<br>');
    }
    return content;
}

export async function startServer(port = 8080) {
    if (!window.__TAURI__ || !window.__TAURI__.core) {
        throw new Error('Tauri not available');
    }
    return await window.__TAURI__.core.invoke('start_octomind_server', { port });
}

export async function stopServer() {
    if (!window.__TAURI__ || !window.__TAURI__.core) {
        throw new Error('Tauri not available');
    }
    return await window.__TAURI__.core.invoke('stop_octomind_server');
}

export async function isServerRunning() {
    if (!window.__TAURI__ || !window.__TAURI__.core) {
        return false;
    }
    return await window.__TAURI__.core.invoke('is_server_running');
}
