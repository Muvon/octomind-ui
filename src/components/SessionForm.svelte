<script>
    import { sessionForm, sessionHistory, currentSessionConfig, addMessage, connectionState } from '../stores.js';
    import { selectDirectory, generateSessionName } from '../lib/tauri.js';
    import { createEventDispatcher, onMount } from 'svelte';

    const dispatch = createEventDispatcher();

    // Local form state (synced with store)
    let form = {
        sessionName: '',
        directory: '',
        role: 'developer',
        model: '',
        temperature: 0.7,
        maxTokens: ''
    };

    // Sync from store on mount
    onMount(async () => {
        const stored = $sessionForm;
        form = { ...form, ...stored };
        
        // Set initial directory if not set
        if (!form.directory) {
            form.directory = await getCurrentDirectory();
        }
    });

    // Sync to store when form changes
    function updateForm(key, value) {
        form[key] = value;
        sessionForm.set({ ...form });
    }

    $: sessionNamePreview = (!form.sessionName?.trim() && form.directory?.trim())
        ? `Preview: ${generateSessionName(form.directory.trim())}`
        : '';

    async function handleSelectDirectory() {
        try {
            const selected = await selectDirectory();
            if (selected) {
                updateForm('directory', selected);
                addMessage('status', `📁 Directory selected: ${selected}`);
            }
        } catch (error) {
            addMessage('error', `Failed to select directory: ${error.message}`);
        }
    }

    async function createSession() {
        if (!form.directory) {
            alert('Please select or enter a working directory');
            return;
        }

        const sessionName = generateSessionName(form.directory, form.sessionName);

        // Store session config
        currentSessionConfig.set({
            name: sessionName,
            directory: form.directory,
            role: form.role,
            model: form.model || null,
            temperature: form.temperature,
            maxTokens: form.maxTokens || null
        });

        // Add to session history
        const sessionData = {
            name: sessionName,
            directory: form.directory,
            role: form.role,
            model: form.model,
            temperature: form.temperature,
            max_tokens: form.maxTokens,
            lastUsed: new Date().toISOString(),
            totalCost: 0,
            totalTokens: 0
        };

        sessionHistory.update(history => {
            const existingIndex = history.findIndex(s => s.name === sessionName);
            if (existingIndex >= 0) {
                history[existingIndex] = { ...history[existingIndex], ...sessionData };
            } else {
                history.unshift(sessionData);
            }
            return history.slice(0, 50);
        });

        addMessage('status', `✅ Session created: ${sessionName}`);
        dispatch('connect');
    }

    async function getCurrentDirectory() {
        try {
            if (window.__TAURI__?.core) {
                const dirs = await window.__TAURI__.core.invoke('list_directories');
                return dirs[0] || '/home/box/work/muvon/octomind';
            }
            return '/home/box/work/muvon/octomind';
        } catch {
            return '/home/box/work/muvon/octomind';
        }
    }
</script>

<div class="session-form">
    <h3>✨ New Session</h3>

    <div class="form-group">
        <label for="session-name">Session Name (optional)</label>
        <input
            type="text"
            id="session-name"
            placeholder="Auto-generated if empty"
            value={form.sessionName}
            on:input={(e) => updateForm('sessionName', e.target.value)}
        >
        {#if sessionNamePreview}
            <small class="text-muted" style="margin-top: 4px; display: block; font-size: 11px;">
                {sessionNamePreview}
            </small>
        {/if}
    </div>

    <div class="form-group">
        <label for="directory">Working Directory</label>
        <div class="directory-input-group">
            <input
                type="text"
                id="directory"
                placeholder="/path/to/project"
                value={form.directory}
                on:input={(e) => updateForm('directory', e.target.value)}
            >
            <button class="btn-icon" on:click={handleSelectDirectory} title="Browse">
                📁
            </button>
        </div>
    </div>

    <div class="form-group">
        <label for="role">Role</label>
        <select id="role" value={form.role} on:change={(e) => updateForm('role', e.target.value)}>
            <option value="developer">Developer</option>
            <option value="assistant">Assistant</option>
        </select>
    </div>

    <div class="form-group">
        <label for="model">Model (optional)</label>
        <input
            type="text"
            id="model"
            placeholder="e.g., openrouter:anthropic/claude-3.5-sonnet"
            value={form.model}
            on:input={(e) => updateForm('model', e.target.value)}
        >
    </div>

    <div style="display: flex; gap: 12px;">
        <div class="form-group" style="flex: 1;">
            <label for="temperature">Temperature</label>
            <input
                type="number"
                id="temperature"
                min="0"
                max="2"
                step="0.1"
                value={form.temperature}
                on:input={(e) => updateForm('temperature', parseFloat(e.target.value) || 0.7)}
            >
        </div>
        <div class="form-group" style="flex: 1;">
            <label for="max-tokens">Max Tokens</label>
            <input
                type="number"
                id="max-tokens"
                placeholder="Default"
                value={form.maxTokens}
                on:input={(e) => updateForm('maxTokens', e.target.value)}
            >
        </div>
    </div>

    <button class="btn" on:click={createSession} style="width: 100%;">
        🚀 Create Session
    </button>
</div>
