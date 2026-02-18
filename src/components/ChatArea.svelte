<script>
    import { messages, isThinking, thinkingContent, thinkingTokens, isSending, sessionCost, sessionTokens } from '../stores.js';
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
            <div class="empty-state-text">
                Create a session to start
            </div>
        </div>
    {:else}
        {#each $messages as message (message.id)}
            <div class="message {message.type}">
                {#if message.type === 'thinking'}
                    <div class="thinking-header">
                        <div class="thinking-spinner"></div>
                        <span>Thinking</span>
                    </div>
                    <div class="thinking-content">
                        {@html formatMessage(message.content)}
                    </div>
                {:else if message.type === 'assistant'}
                    <div class="message-label">Assistant</div>
                    {@html formatMessage(message.content)}
                {:else if message.type === 'status'}
                    <div class="status-content">
                        {@html formatMessage(message.content)}
                        
                        {#if message.meta?.data}
                            <div class="command-output">
                                {#if message.meta.data.servers}
                                    <div class="command-section">
                                        <h4>MCP Servers ({message.meta.data.servers.length})</h4>
                                        <div class="servers-list">
                                            {#each message.meta.data.servers as server}
                                                <div class="server-item">
                                                    <div class="server-header">
                                                        <span class="server-name">{server.name}</span>
                                                        <span class="server-health {server.health}">{server.health}</span>
                                                    </div>
                                                    <div class="server-meta">
                                                        <span class="server-type">{server.connection_type}</span>
                                                        {#if server.restart_count > 0}
                                                            <span class="server-restarts">{server.restart_count} restarts</span>
                                                        {/if}
                                                    </div>
                                                </div>
                                            {/each}
                                        </div>
                                    </div>
                                {/if}
                                {#if message.meta.data.tools}
                                    <div class="command-section">
                                        <h4>Available Tools ({message.meta.data.total_tools})</h4>
                                        <div class="tools-list">
                                            {#each Object.entries(message.meta.data.tools) as [serverName, tools]}
                                                <div class="tool-group">
                                                    <div class="tool-group-header">{serverName}</div>
                                                    {#each tools as tool}
                                                        <div class="tool-item">
                                                            <span class="tool-name">{tool.name}</span>
                                                            <span class="tool-desc">{tool.description}</span>
                                                        </div>
                                                    {/each}
                                                </div>
                                            {/each}
                                        </div>
                                    </div>
                                {/if}
                                {#if message.meta.data.statistics}
                                    <div class="command-section">
                                        <h4>Cache Statistics</h4>
                                        <div class="info-grid">
                                            <div class="info-item">
                                                <span class="info-label">Cache Next Message</span>
                                                <span class="info-value">{message.meta.data.cache_next_user_message ? 'Yes' : 'No'}</span>
                                            </div>
                                            <div class="info-item">
                                                <span class="info-label">Total Cache Write</span>
                                                <span class="info-value">{message.meta.data.statistics.total_cache_write_tokens.toLocaleString()} tokens</span>
                                            </div>
                                            <div class="info-item">
                                                <span class="info-label">Total Cache Read</span>
                                                <span class="info-value">{message.meta.data.statistics.total_cache_read_tokens.toLocaleString()} tokens</span>
                                            </div>
                                            <div class="info-item">
                                                <span class="info-label">Current Non-Cached</span>
                                                <span class="info-value">{message.meta.data.statistics.current_non_cached_tokens.toLocaleString()} tokens</span>
                                            </div>
                                        </div>
                                    </div>
                                {/if}
                            </div>
                        {/if}
                        
                        {#if message.data}
                            <div class="command-output">
                                {#if message.data.servers}
                                    <div class="command-section">
                                        <h4>MCP Servers ({message.data.servers.length})</h4>
                                        <div class="servers-list">
                                            {#each message.data.servers as server}
                                                <div class="server-item">
                                                    <div class="server-header">
                                                        <span class="server-name">{server.name}</span>
                                                        <span class="server-health {server.health}">{server.health}</span>
                                                    </div>
                                                    <div class="server-meta">
                                                        <span class="server-type">{server.connection_type}</span>
                                                        {#if server.restart_count > 0}
                                                            <span class="server-restarts">{server.restart_count} restarts</span>
                                                        {/if}
                                                    </div>
                                                </div>
                                            {/each}
                                        </div>
                                    </div>
                                {/if}
                                {#if message.data.tools}
                                    <div class="command-section">
                                        <h4>Available Tools ({message.data.total_tools})</h4>
                                        <div class="tools-list">
                                            {#each Object.entries(message.data.tools) as [serverName, tools]}
                                                <div class="tool-group">
                                                    <div class="tool-group-header">{serverName}</div>
                                                    {#each tools as tool}
                                                        <div class="tool-item">
                                                            <span class="tool-name">{tool.name}</span>
                                                            <span class="tool-desc">{tool.description}</span>
                                                        </div>
                                                    {/each}
                                                </div>
                                            {/each}
                                        </div>
                                    </div>
                                {/if}
                            </div>
                        {/if}
                        
                        {#if message.data?.statistics}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>Cache Statistics</h4>
                                    <div class="info-grid">
                                        <div class="info-item">
                                            <span class="info-label">Cache Next Message</span>
                                            <span class="info-value">{message.data.cache_next_user_message ? 'Yes' : 'No'}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Total Cache Write</span>
                                            <span class="info-value">{message.data.statistics.total_cache_write_tokens.toLocaleString()} tokens</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Total Cache Read</span>
                                            <span class="info-value">{message.data.statistics.total_cache_read_tokens.toLocaleString()} tokens</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Current Non-Cached</span>
                                            <span class="info-value">{message.data.statistics.current_non_cached_tokens.toLocaleString()} tokens</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        {/if}
                        
                        
                        {#if message.meta?.commands || message.commands}
                            {@const commands = message.meta?.commands || message.commands}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>Available Commands ({commands.length})</h4>
                                    <div class="commands-list">
                                        {#each commands as cmd}
                                            <div class="command-item">{cmd}</div>
                                        {/each}
                                    </div>
                                </div>
                            </div>
                        {/if}
                        
                        {#if (message.meta?.command_type === 'info' || message.command_type === 'info')}
                            {@const info = message.meta?.command_type === 'info' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>Session Info</h4>
                                    <div class="info-grid">
                                        <div class="info-item">
                                            <span class="info-label">Session</span>
                                            <span class="info-value">{info.session_name}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Model</span>
                                            <span class="info-value">{info.model}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Role</span>
                                            <span class="info-value">{info.role}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Tokens Used</span>
                                            <span class="info-value">{info.tokens_used?.toLocaleString() || 0}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Total Cost</span>
                                            <span class="info-value">${info.total_cost?.toFixed(4) || '0.0000'}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Cache Write</span>
                                            <span class="info-value">{info.tokens_cache_write?.toLocaleString() || 0} tokens</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Cached</span>
                                            <span class="info-value">{info.tokens_cached?.toLocaleString() || 0} tokens</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">Cache Savings</span>
                                            <span class="info-value">${info.cache_savings?.toFixed(4) || '0.0000'}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        {/if}
                        
                        {#if (message.meta?.command_type === 'report' || message.command_type === 'report') && (message.meta?.entries || message.entries)}
                            {@const report = message.meta?.command_type === 'report' ? message.meta : message}
                            {@const entries = report.entries}
                            {@const totals = report.totals}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>Session Report</h4>
                                    <div class="report-entries">
                                        {#each entries as entry}
                                            <div class="report-entry">
                                                <div class="report-header">
                                                    <span class="report-request">{entry.user_request.substring(0, 60)}...</span>
                                                    <span class="report-cost">${entry.cost}</span>
                                                </div>
                                                <div class="report-meta">
                                                    <span>AI: {entry.ai_time}</span>
                                                    <span>Human: {entry.human_time}</span>
                                                    <span>Tools: {entry.tools_used}</span>
                                                </div>
                                            </div>
                                        {/each}
                                    </div>
                                    {#if totals}
                                        <div class="report-totals">
                                            <div class="total-item">
                                                <span class="total-label">Total Cost</span>
                                                <span class="total-value">${totals.total_cost.toFixed(4)}</span>
                                            </div>
                                            <div class="total-item">
                                                <span class="total-label">Total AI Time</span>
                                                <span class="total-value">{(totals.total_ai_time_ms / 1000).toFixed(1)}s</span>
                                            </div>
                                            <div class="total-item">
                                                <span class="total-label">Total Human Time</span>
                                                <span class="total-value">{(totals.total_human_time_ms / 1000 / 60).toFixed(1)}m</span>
                                            </div>
                                            <div class="total-item">
                                                <span class="total-label">Tool Calls</span>
                                                <span class="total-value">{totals.total_tool_calls}</span>
                                            </div>
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        {/if}
                        
                        {#if (message.meta?.command_type === 'plan' || message.command_type === 'plan')}
                            {@const planData = message.meta?.command_type === 'plan' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    {#if planData.has_plan && planData.plan}
                                        <h4>📋 {planData.plan.plan_title}</h4>
                                        <div class="plan-progress">
                                            <span class="plan-progress-text">Task {planData.plan.current_task}/{planData.plan.total_tasks}</span>
                                            <div class="plan-progress-bar">
                                                <div class="plan-progress-fill" style="width: {(planData.plan.current_task / planData.plan.total_tasks * 100)}%"></div>
                                            </div>
                                        </div>
                                        <div class="plan-current-task">
                                            <div class="current-task-label">CURRENT TASK</div>
                                            <div class="current-task-title">{planData.plan.current_task_title}</div>
                                            <div class="current-task-desc">{planData.plan.current_task_description}</div>
                                        </div>
                                        <div class="plan-tasks">
                                            {#each planData.plan.tasks as task, index}
                                                <div class="plan-task {task.status}">
                                                    <div class="task-header">
                                                        <span class="task-number">{index + 1}</span>
                                                        <span class="task-title">{task.title}</span>
                                                        <span class="task-status">
                                                            {#if task.status === 'completed'}✅
                                                            {:else if task.status === 'in_progress'}🔄
                                                            {:else}⏳
                                                            {/if}
                                                        </span>
                                                    </div>
                                                    <div class="task-desc">{task.description}</div>
                                                </div>
                                            {/each}
                                        </div>
                                    {:else}
                                        <h4>📋 No Active Plan</h4>
                                        <div class="plan-no-plan">
                                            {@html formatMessage(planData.display || 'No plan is currently active. Use the plan tool for complex, multi-step tasks.')}
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        {/if}
                        
                        
                        {#if (message.meta?.command_type === 'context' || message.command_type === 'context')}
                            {@const contextData = message.meta?.command_type === 'context' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>📋 Context ({contextData.total_messages || 0} messages)</h4>
                                    {#if contextData.filter}
                                        <div class="context-filter">
                                            <span class="filter-label">Filter:</span>
                                            <span class="filter-value">{contextData.filter}</span>
                                        </div>
                                    {/if}
                                    <div class="context-messages">
                                        {#each contextData.filtered_messages || [] as msg}
                                            <div class="context-message {msg.role}">
                                                <div class="context-message-header">
                                                    <span class="context-role">{msg.role}</span>
                                                    {#if msg.tool_calls && msg.tool_calls.length > 0}
                                                        <span class="context-tools">{msg.tool_calls.length} tools</span>
                                                    {/if}
                                                </div>
                                                <div class="context-message-content">
                                                    {@html formatMessage(msg.content?.substring(0, 500) || '')}
                                                    {#if msg.content && msg.content.length > 500}
                                                        <span class="context-truncated">... ({msg.content.length - 500} more chars)</span>
                                                    {/if}
                                                </div>
                                            </div>
                                        {/each}
                                    </div>
                                </div>
                            </div>
                        {/if}
                        
                        {#if (message.meta?.command_type === 'list' || message.command_type === 'list')}
                            {@const listData = message.meta?.command_type === 'list' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>📋 Available Sessions (Page {listData.page || 1} of {listData.total_pages || 1})</h4>
                                    <div class="list-summary">
                                        <span>Showing {listData.sessions?.length || 0} of {listData.total_sessions || 0} sessions</span>
                                    </div>
                                    {#if listData.sessions && listData.sessions.length > 0}
                                        <div class="sessions-table">
                                            <div class="sessions-header">
                                                <div class="session-cell name">Name</div>
                                                <div class="session-cell created">Created</div>
                                                <div class="session-cell model">Model</div>
                                                <div class="session-cell tokens">Tokens</div>
                                                <div class="session-cell cost">Cost</div>
                                            </div>
                                            {#each listData.sessions as session}
                                                <div class="session-row {session.is_current ? 'current' : ''}">
                                                    <div class="session-cell name">
                                                        <span class="session-name">{session.name}</span>
                                                        {#if session.is_current}
                                                            <span class="current-badge">current</span>
                                                        {/if}
                                                    </div>
                                                    <div class="session-cell created">{session.created}</div>
                                                    <div class="session-cell model">{session.model}</div>
                                                    <div class="session-cell tokens">{session.tokens?.toLocaleString() || 0}</div>
                                                    <div class="session-cell cost">${session.cost?.toFixed(4) || '0.0000'}</div>
                                                </div>
                                            {/each}
                                        </div>
                                        {#if listData.total_pages > 1}
                                            <div class="list-navigation">
                                                <div class="nav-info">
                                                    <span>Navigation:</span>
                                                    <span class="nav-hint">Next: <code>/list {listData.page + 1}</code></span>
                                                    {#if listData.page > 1}
                                                        <span class="nav-hint">Previous: <code>/list {listData.page - 1}</code></span>
                                                    {/if}
                                                    <span class="nav-hint">Go to page: <code>/list &lt;page&gt;</code></span>
                                                </div>
                                            </div>
                                        {/if}
                                    {/if}
                                </div>
                            </div>
                        {/if}
                        
                        
                        {#if (message.meta?.command_type === 'role' || message.command_type === 'role')}
                            {@const roleData = message.meta?.command_type === 'role' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>👤 Role Configuration</h4>
                                    <div class="role-current">
                                        <span class="role-label">Current Role:</span>
                                        <span class="role-value">{roleData.current_role}</span>
                                        {#if roleData.changed}
                                            <span class="role-changed">changed from {roleData.old_role || 'none'}</span>
                                        {/if}
                                    </div>
                                    {#if roleData.available_roles && roleData.available_roles.length > 0}
                                        <div class="role-list">
                                            <div class="role-list-header">Available Roles:</div>
                                            {#each roleData.available_roles as role}
                                                <div class="role-item {role === roleData.current_role ? 'active' : ''}">{role}</div>
                                            {/each}
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        {/if}
                        
                        {#if (message.meta?.command_type === 'model' || message.command_type === 'model')}
                            {@const modelData = message.meta?.command_type === 'model' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>🤖 Model Configuration</h4>
                                    <div class="model-current">
                                        <span class="model-label">Current Model:</span>
                                        <span class="model-value">{modelData.new_model}</span>
                                        {#if modelData.changed}
                                            <span class="model-changed">changed from {modelData.old_model || 'none'}</span>
                                        {/if}
                                    </div>
                                </div>
                            </div>
                        {/if}
                        
                        {#if (message.meta?.command_type === 'workflow' || message.command_type === 'workflow')}
                            {@const workflowData = message.meta?.command_type === 'workflow' ? message.meta : message}
                            <div class="command-output">
                                <div class="command-section">
                                    <h4>⚙️ Workflows</h4>
                                    {#if workflowData.data?.workflows && workflowData.data.workflows.length > 0}
                                        <div class="workflow-list">
                                            {#each workflowData.data.workflows as workflow}
                                                <div class="workflow-item">{workflow}</div>
                                            {/each}
                                        </div>
                                    {/if}
                                    {#if workflowData.data?.message}
                                        <div class="workflow-message">{workflowData.data.message}</div>
                                    {/if}
                                </div>
                            </div>
                        {/if}
                        
                        
                        
                        {#if message.meta || message.command_type || message.model || message.tokens_used || message.total_cost}
                            {@const meta = message.meta || message}
                            <div class="status-meta">
                                {#if meta.command_type}
                                    <span class="status-badge command">{meta.command_type}</span>
                                {/if}
                                {#if meta.model}
                                    <span class="status-badge model">{meta.model}</span>
                                {/if}
                                {#if meta.tokens_used}
                                    <span class="status-badge tokens">{meta.tokens_used.toLocaleString()} tokens</span>
                                {/if}
                                {#if meta.total_cost}
                                    <span class="status-badge cost">${meta.total_cost.toFixed(4)}</span>
                                {/if}
                            </div>
                        {/if}
                    </div>
                {:else}
                    {@html formatMessage(message.content)}
                {/if}
            </div>
        {/each}
    {/if}

    {#if $isSending}
        <div class="message sending">
            <div class="sending-dots">
                <span></span>
                <span></span>
                <span></span>
            </div>
        </div>
    {/if}

    {#if $isThinking && $thinkingContent}
        <div class="message thinking">
            <div class="thinking-header">
                <div class="thinking-spinner"></div>
                <span>Thinking</span>
            </div>
            <div class="thinking-content">
                {@html formatMessage($thinkingContent)}
            </div>
        </div>
    {/if}
</div>
