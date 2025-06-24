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
        if (!window.__TAURI__ || !window.__TAURI__.tauri) {
            return '/Users/dk/Work/dev/muvon/octomind'; // Fallback
        }

        const dirs = await window.__TAURI__.tauri.invoke('list_directories');
        return dirs[0] || '';
    } catch (error) {
        console.error('Failed to get current directory:', error);
        return '/Users/dk/Work/dev/muvon/octomind'; // Fallback
    }
}

export async function selectDirectory() {
    try {
        console.log('=== Testing Directory Selection ===');

        // First try the Tauri dialog API
        if (window.__TAURI__ && window.__TAURI__.dialog && window.__TAURI__.dialog.open) {
            console.log('Trying Tauri dialog API...');

            const selected = await window.__TAURI__.dialog.open({
                directory: true,
                multiple: false,
                title: 'Select Working Directory'
            });

            if (selected && selected !== null) {
                return selected;
            }
        }

        // Fallback to native command
        console.log('Trying native directory selection...');
        if (window.__TAURI__ && window.__TAURI__.tauri) {
            const selected = await window.__TAURI__.tauri.invoke('select_directory_native');

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

    // Extract basename from directory path
    const basename = directory.split('/').pop() || directory.split('\\\\').pop() || 'unknown';

    // Format: ui-YYYYMMDD-HHMMSS-basename
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
        // Enhanced markdown-like formatting for octomind responses
        return content
            // Handle code blocks first (multiline)
            .replace(/```([^`]+)```/g, '<pre><code>$1</code></pre>')
            // Handle inline code
            .replace(/`([^`]+)`/g, '<code>$1</code>')
            // Handle bold text
            .replace(/\\*\\*([^*]+)\\*\\*/g, '<strong>$1</strong>')
            // Handle italic text
            .replace(/\\*([^*]+)\\*/g, '<em>$1</em>')
            // Handle line breaks
            .replace(/\\n/g, '<br>');
    }
    return content;
}
