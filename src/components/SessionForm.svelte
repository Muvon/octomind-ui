<script>
    import { sessionForm, sessionHistory, currentSessionId, currentSessionConfig, messages, statusText } from '../stores.js';
    import { initializeTauri, getCurrentDirectory, selectDirectory, generateSessionName } from '../lib/tauri.js';
    import { onMount } from 'svelte';

    let sessionNamePreview = '';
    let form = {};

    // Subscribe to form store
    sessionForm.subscribe(value => {
        form = value;
    });

    onMount(async () => {
        // Set initial directory
        const currentDir = await getCurrentDirectory();
        sessionForm.update(f => ({ ...f, directory: currentDir }));
        updateSessionNamePreview();
    });

    function updateSessionNamePreview() {
        if (!form.sessionName.trim() && form.directory.trim()) {
            sessionNamePreview = `Preview: ${generateSessionName(form.directory.trim())}`;
        } else {
            sessionNamePreview = '';
        }
    }

    async function handleSelectDirectory() {
        try {
            const selected = await selectDirectory();
            if (selected) {
                sessionForm.update(f => ({ ...f, directory: selected }));
                messages.update(msgs => [...msgs, { type: 'system', content: `📁 Directory selected: ${selected}` }]);
                updateSessionNamePreview();
            } else {
                messages.update(msgs => [...msgs, { type: 'system', content: '📁 No directory selected' }]);
            }
        } catch (error) {
            messages.update(msgs => [...msgs, { type: 'error', content: `Failed to select directory: ${error.message}` }]);
        }
    }

    async function createSession() {
        if (!form.directory) {
            alert('Please select or enter a working directory');
            return;
        }

        // Generate unique session name
        const sessionName = generateSessionName(form.directory, form.sessionName);

        console.log('Creating session with:', {
            sessionName,
            directory: form.directory,
            role: form.role,
            model: form.model,
            temperature: form.temperature,
            maxTokens: form.maxTokens
        });

        try {
            await initializeTauri();

            if (!window.__TAURI__ || !window.__TAURI__.core || !window.__TAURI__.core.invoke) {
                throw new Error('Tauri core.invoke not available');
            }

            const invoke = window.__TAURI__.core.invoke;

            messages.update(msgs => [...msgs, { type: 'system', content: `Creating new session: ${sessionName}...` }]);

            const sessionId = await invoke('create_session_config', {
                name: sessionName,
                directory: form.directory,
                model: form.model || null,
                temperature: form.temperature,
                maxTokens: form.maxTokens ? parseInt(form.maxTokens) : null,
                role: form.role
            });

            console.log('Session created with ID:', sessionId);
            currentSessionId.set(sessionId);

            const sessionInfo = await invoke('get_session_info', {
                sessionId: sessionId
            });

            console.log('Session info:', sessionInfo);
            currentSessionConfig.set(sessionInfo);

            // Add to session history
            const sessionData = {
                name: sessionName,
                directory: form.directory,
                role: form.role,
                model: form.model,
                temperature: form.temperature,
                max_tokens: form.maxTokens,
                lastUsed: new Date().toISOString(),
                totalCost: 0
            };

            sessionHistory.update(history => {
                const existingIndex = history.findIndex(s => s.name === sessionName);
                if (existingIndex >= 0) {
                    history[existingIndex] = { ...history[existingIndex], ...sessionData };
                } else {
                    history.unshift(sessionData);
                }
                return history.slice(0, 50); // Keep only last 50 sessions
            });

            messages.update(msgs => [...msgs, { type: 'system', content: `✅ Session created: ${sessionName} in ${form.directory}` }]);

        } catch (error) {
            console.error('Failed to create session:', error);
            messages.update(msgs => [...msgs, { type: 'error', content: `❌ Failed to create session: ${error}` }]);
        }
    }

    // Reactive updates
    $: {
        updateSessionNamePreview();
    }
</script>

<div class="session-form">
    <h3>✨ New Session</h3>

    <div class="form-group">
        <label for="session-name">Session Name (optional):</label>
        <input
            type="text"
            id="session-name"
            placeholder="Auto: ui-YYYYMMDD-HHMMSS-dirname"
            bind:value={form.sessionName}
            on:input={() => sessionForm.update(f => ({ ...f, sessionName: form.sessionName }))}
        >
        <div id="session-name-preview" style="font-size: 11px; opacity: 0.6; margin-top: 4px;">
            {sessionNamePreview}
        </div>
    </div>

    <div class="form-group">
        <label for="directory">Working Directory:</label>
        <input
            type="text"
            id="directory"
            placeholder="Enter directory path"
            bind:value={form.directory}
            on:input={() => sessionForm.update(f => ({ ...f, directory: form.directory }))}
        >
        <div class="directory-buttons">
            <button class="btn btn-secondary" on:click={handleSelectDirectory}>
                📁 Browse Directory
            </button>
        </div>
    </div>

    <div class="form-group">
        <label for="role">Role:</label>
        <select
            id="role"
            bind:value={form.role}
            on:change={() => sessionForm.update(f => ({ ...f, role: form.role }))}
        >
            <option value="developer">Developer</option>
            <option value="assistant">Assistant</option>
        </select>
    </div>

    <div class="form-group">
        <label for="model">Model (optional):</label>
        <input
            type="text"
            id="model"
            placeholder="e.g., openrouter:anthropic/claude-3.5-sonnet"
            bind:value={form.model}
            on:input={() => sessionForm.update(f => ({ ...f, model: form.model }))}
        >
    </div>

    <div class="form-group">
        <label for="temperature">Temperature:</label>
        <input
            type="number"
            id="temperature"
            min="0"
            max="2"
            step="0.1"
            bind:value={form.temperature}
            on:input={() => sessionForm.update(f => ({ ...f, temperature: parseFloat(form.temperature) || 0.7 }))}
        >
    </div>

    <div class="form-group">
        <label for="max-tokens">Max Tokens (optional):</label>
        <input
            type="number"
            id="max-tokens"
            placeholder="Leave empty for default"
            bind:value={form.maxTokens}
            on:input={() => sessionForm.update(f => ({ ...f, maxTokens: form.maxTokens }))}
        >
    </div>

    <button class="btn" on:click={createSession}>🚀 Create Session</button>
</div>
