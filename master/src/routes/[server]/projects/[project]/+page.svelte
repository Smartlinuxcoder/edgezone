<script>
    import { onMount, onDestroy } from 'svelte';
    export let data;
    let showEditModal = false;
    let editedProject = { ...data.project };
    let selectedDeployment = null;
    let deploymentLogs = '';
    let showLogsModal = false;
    let deployments = data.deployments;
    let pollingInterval;
    let isDeploying = false;
    let deployError = null;

    async function pollDeployments() {
        try {
            const res = await fetch(`/api/servers/${data.server.id}?path=/projects/${data.project.id}/deployments`);
            if (res.ok) {
                deployments = await res.json();
            }
        } catch (e) {
            console.error('Error polling deployments:', e);
        }
    }

    async function viewDeploymentLogs(deployment) {
        try {
            const res = await fetch(`/api/servers/${data.server.id}?path=/projects/${data.project.id}/deployments/${deployment.id}`);
            if (!res.ok) throw new Error('Failed to fetch logs');
            const logData = await res.json();
            deploymentLogs = logData.logs;
            selectedDeployment = deployment;
            showLogsModal = true;
        } catch (e) {
            console.error('Error fetching logs:', e);
        }
    }

    async function startDeployment() {
        isDeploying = true;
        deployError = null;
        
        try {
            const res = await fetch(
                `/api/servers/${data.server.id}/projects/${data.project.id}/deploy`,
                { method: 'POST' }
            );
            
            if (!res.ok) throw new Error('Failed to start deployment');
            
            const newDeployment = await res.json();
            deployments = [newDeployment, ...deployments];
            
            // Start polling more frequently during deployment
            const pollDeployment = async () => {
                if (newDeployment.status === 0) { // If still building
                    const statusRes = await fetch(
                        `/api/servers/${data.server.id}/projects/${data.project.id}/deploy`
                    );
                    if (statusRes.ok) {
                        const updated = await statusRes.json();
                        deployments = deployments.map(d => 
                            d.id === updated.id ? updated : d
                        );
                        if (updated.status === 0) {
                            setTimeout(pollDeployment, 2000);
                        }
                    }
                }
            };
            
            pollDeployment();
        } catch (e) {
            console.error('Deployment error:', e);
            deployError = 'Failed to start deployment';
        } finally {
            isDeploying = false;
        }
    }

    onMount(() => {
        pollingInterval = setInterval(pollDeployments, 5000);
    });

    onDestroy(() => {
        if (pollingInterval) clearInterval(pollingInterval);
    });

    $: statusColor = (status) => {
        switch (status) {
            case 0: return 'text-yellow-400';
            case 1: return 'text-green-400';
            case 2: return 'text-red-400';
            default: return 'text-gray-400';
        }
    };

    $: statusText = (status) => {
        switch (status) {
            case 0: return 'Building';
            case 1: return 'Running';
            case 2: return 'Failed';
            default: return 'Unknown';
        }
    };
</script>

<div class="container mx-auto max-w-5xl">
    <header class="mb-12">
        <div class="flex items-center gap-4 mb-4">
            <a href="/{data.server.id}" class="text-[#cdd6f4] hover:text-[#89b4fa] transition-colors">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                </svg>
            </a>
            <h1 class="text-[#cdd6f4] text-3xl font-bold">{data.project.name}</h1>
            <button
                on:click={() => showEditModal = true}
                class="p-2 hover:bg-[#1e1e2e]/50 rounded-md transition-colors">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#cdd6f4]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
            </button>
        </div>
        <p class="text-[#a6adc8]">{data.project.git_repo}</p>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Project Info -->
        <div class="space-y-6">
            <div class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-6">
                <h2 class="text-xl font-semibold text-[#cdd6f4] mb-4">Configuration</h2>
                <dl class="space-y-3">
                    <div>
                        <dt class="text-[#a6adc8] text-sm">Install Command</dt>
                        <dd class="text-[#cdd6f4] font-mono">{data.project.install_cmd || 'No command'}</dd>
                    </div>
                    <div>
                        <dt class="text-[#a6adc8] text-sm">Build Command</dt>
                        <dd class="text-[#cdd6f4] font-mono">{data.project.build_cmd || 'No command'}</dd>
                    </div>
                    <div>
                        <dt class="text-[#a6adc8] text-sm">Start Command</dt>
                        <dd class="text-[#cdd6f4] font-mono">{data.project.run_cmd}</dd>
                    </div>
                </dl>
            </div>

            <div class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-6">
                <h2 class="text-xl font-semibold text-[#cdd6f4] mb-4">Environment Variables</h2>
                <div class="font-mono text-sm">
                    {#if data.project.env}
                        <pre class="whitespace-pre-wrap text-[#cdd6f4]">{data.project.env}</pre>
                    {:else}
                        <p class="text-[#a6adc8]">No environment variables set</p>
                    {/if}
                </div>
            </div>
        </div>

        <!-- Deployments -->
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">Deployments</h2>
                <div class="flex items-center gap-4">
                    {#if deployError}
                        <p class="text-red-400 text-sm">{deployError}</p>
                    {/if}
                    <button
                        on:click={startDeployment}
                        disabled={isDeploying}
                        class="px-4 py-2 bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7] 
                               text-[#1e1e2e] rounded-md font-medium transition-all duration-200 disabled:opacity-50 
                               disabled:cursor-not-allowed flex items-center gap-2">
                        {#if isDeploying}
                            <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            Deploying...
                        {:else}
                            Deploy
                        {/if}
                    </button>
                </div>
            </div>

            <div class="space-y-4">
                {#each deployments as deployment}
                    <div class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-4">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-[#cdd6f4] font-mono text-sm">{deployment.commit_hash || 'No commit hash'}</p>
                                <p class={`text-sm ${statusColor(deployment.status)}`}>{statusText(deployment.status)}</p>
                            </div>
                            <button
                                on:click={() => viewDeploymentLogs(deployment)}
                                class="p-2 hover:bg-[#1e1e2e]/50 rounded-md transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#cdd6f4]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            </button>
                        </div>
                    </div>
                {/each}

                {#if deployments.length === 0}
                    <div class="text-center py-8">
                        <p class="text-[#a6adc8]">No deployments yet</p>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>

{#if showLogsModal}
    <div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <div class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-4xl">
            <div class="p-6 space-y-4">
                <div class="flex items-center justify-between">
                    <h2 class="text-xl font-semibold text-[#cdd6f4]">Deployment Logs</h2>
                    <button
                        on:click={() => showLogsModal = false}
                        class="p-2 hover:bg-[#313244]/30 rounded-md transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#cdd6f4]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
                <div class="bg-[#11111b] rounded-lg p-4 max-h-[60vh] overflow-y-auto">
                    <pre class="font-mono text-sm text-[#cdd6f4] whitespace-pre-wrap">{deploymentLogs}</pre>
                </div>
            </div>
        </div>
    </div>
{/if}

{#if showEditModal}
    <div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <div class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-2xl">
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">Edit Project</h2>
                <form method="POST" action="?/updateProject" class="space-y-4">
                    <!-- Basic Info -->
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Project Name</label>
                            <input type="text" name="name" required
                                bind:value={editedProject.name}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Git Repository</label>
                            <input type="text" name="git_repo" required
                                bind:value={editedProject.git_repo}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                    </div>

                    <!-- Commands -->
                    <div class="grid grid-cols-3 gap-4">
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Install Command</label>
                            <input type="text" name="install_cmd"
                                bind:value={editedProject.install_cmd}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Build Command</label>
                            <input type="text" name="build_cmd"
                                bind:value={editedProject.build_cmd}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Start Command</label>
                            <input type="text" name="run_cmd" required
                                bind:value={editedProject.run_cmd}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                    </div>
                </form>
            </div>
        </div>
    </div>
{/if}
