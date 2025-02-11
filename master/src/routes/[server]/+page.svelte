<script>
    export let data;
    let showNewProjectModal = false;
    let newProject = {
        name: '',
        git_repo: '',
        install_cmd: '',
        build_cmd: '',
        run_cmd: '',
        healthcheck_endpoint: '',
        healthcheck_timeout: 30
    };

    async function createProject() {
        try {
            const res = await fetch(`/api/servers/${data.server.id}?path=/projects`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(newProject)
            });
            
            if (!res.ok) throw new Error('Failed to create project');
            showNewProjectModal = false;
            window.location.reload();
        } catch (e) {
            console.error('Error creating project:', e);
        }
    }
</script>

<div class="container mx-auto max-w-5xl">
    <header class="mb-12">
        <div class="flex items-center gap-4 mb-4">
            <a href="/" class="text-[#cdd6f4] hover:text-[#89b4fa] transition-colors">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                </svg>
            </a>
            <h1 class="text-[#cdd6f4] text-3xl font-bold">{data.server.name}</h1>
        </div>
        <p class="text-[#a6adc8]">{data.server.url}</p>
    </header>

    {#if data.error}
        <div class="p-4 bg-red-500/20 border border-red-500/30 rounded-lg mb-8">
            <p class="text-red-400">{data.error}</p>
        </div>
    {/if}

    <div class="flex justify-between items-center mb-8">
        <h2 class="text-2xl font-semibold text-[#cdd6f4]">Projects</h2>
        <button
            on:click={() => showNewProjectModal = true}
            class="px-4 py-2 bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7] 
                   text-[#1e1e2e] rounded-md font-medium transition-all duration-200">
            New Project
        </button>
    </div>

    <div class="grid grid-cols-1 gap-4">
        {#each data.projects as project}
            <div class="p-6 bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 
                      rounded-lg border border-[#6e6c7e]/20 hover:border-[#6e6c7e]/40 transition-all duration-200">
                <div class="flex items-center justify-between">
                    <div>
                        <h3 class="text-[#cdd6f4] text-xl font-medium">{project.name}</h3>
                        <p class="text-[#a6adc8] text-sm mt-1">{project.git_repo}</p>
                    </div>
                    <a href="/{data.server.id}/projects/{project.id}"
                       class="p-2 hover:bg-[#1e1e2e]/50 rounded-md transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#cdd6f4]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                    </a>
                </div>
            </div>
        {/each}

        {#if data.projects.length === 0 && !data.error}
            <div class="text-center py-12">
                <p class="text-[#a6adc8]">No projects found. Create your first project to get started.</p>
            </div>
        {/if}
    </div>
</div>

{#if showNewProjectModal}
    <div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <div class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-lg">
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">New Project</h2>
                <form on:submit|preventDefault={createProject} class="space-y-4">
                    <div>
                        <label for="name" class="block text-sm text-[#cdd6f4] mb-2">Project Name</label>
                        <input type="text" bind:value={newProject.name} required
                               class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                      text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div>
                        <label for="git_repo" class="block text-sm text-[#cdd6f4] mb-2">Git Repository URL</label>
                        <input type="text" bind:value={newProject.git_repo} required
                               class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                      text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Install Command</label>
                            <input type="text" bind:value={newProject.install_cmd} placeholder="bun install"
                                   class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                          text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2">Build Command</label>
                            <input type="text" bind:value={newProject.build_cmd} placeholder="bun run build"
                                   class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                          text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                        </div>
                    </div>
                    <div>
                        <label class="block text-sm text-[#cdd6f4] mb-2">Start Command</label>
                        <input type="text" bind:value={newProject.run_cmd} required placeholder="bun start"
                               class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                      text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div class="flex justify-end gap-3 pt-4">
                        <button type="button" on:click={() => showNewProjectModal = false}
                            class="px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#313244]/30 rounded-md transition-colors">
                            Cancel
                        </button>
                        <button type="submit"
                            class="px-4 py-2 text-sm bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7] 
                                   text-[#1e1e2e] rounded-md font-medium transition-colors">
                            Create Project
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
{/if}
