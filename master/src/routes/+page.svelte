<script>
    import OsIcon from '$lib/components/OsIcon.svelte';
    export let data;
    let showModal = false;
    let showDeleteModal = false;
    let showEditModal = false;
    let selectedServer = {
        id: null,
        name: '',
        url: ''
    };

    function toggleModal() {
        showModal = !showModal;
    }

    function openDeleteModal(server) {
        selectedServer = server;
        showDeleteModal = true;
    }

    function openEditModal(server) {
        selectedServer = { ...server };
        showEditModal = true;
    }

    function handleServerClick(e, serverId) {
        if (e.target.closest('button')) return;
        window.location.href = `/${serverId}`;
    }

    function needsUpdate(serverVersion, latestVersion) {
        if (!serverVersion || !latestVersion) return false;
        return serverVersion !== latestVersion;
    }
</script>

<div class="container mx-auto max-w-5xl">
    <header class="mb-12">
        <h1 class="text-[#cdd6f4] text-3xl font-bold">Servers</h1>
        <p class="text-[#a6adc8] mt-2">Connect and manage your server deployments.</p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each data.servers as server}
            <div class="group p-6 bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 hover:from-[#313244]/40 hover:to-[#313244]/20 
                      rounded-lg border border-[#6e6c7e]/20 transition-all duration-200 cursor-pointer"
                 on:click={(e) => handleServerClick(e, server.id)}>
                <div class="flex items-start justify-between">
                    <div class="flex gap-4">
                        <div class="pt-1">
                            <OsIcon os={server.info?.os || 'unknown'} size={32} />
                        </div>
                        <div>
                            <h3 class="text-[#cdd6f4] text-lg font-medium">{server.name}</h3>
                            <p class="text-[#a6adc8] text-sm mt-1">{server.url}</p>
                            {#if server.info}
                                <div class="flex items-center gap-2 mt-2 text-[#a6adc8] text-sm">
                                    <OsIcon os={server.info.os} distro={server.info.distro} size={32} />
                                    <div class="flex flex-col">
                                        <span>{server.info.distro || server.info.os}</span>
                                        <span class="text-[#6c7086] text-xs">{server.info.arch}</span>
                                    </div>
                                    <span class="text-[#89b4fa] ml-auto">v{server.info.version}</span>
                                </div>
                            {/if}
                        </div>
                    </div>
                    <div class="opacity-0 group-hover:opacity-100 transition-opacity flex gap-2">
                        {#if needsUpdate(server.info?.version, data.latestVersion)}
                            <form method="POST" action="?/updateNodeVersion">
                                <input type="hidden" name="id" value={server.id}>
                                <button type="submit" class="p-2 hover:bg-[#89b4fa]/20 rounded-md" title="Update to {data.latestVersion}">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#89b4fa]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                                    </svg>
                                </button>
                            </form>
                        {/if}
                        <button class="p-2 hover:bg-[#1e1e2e]/50 rounded-md" on:click={() => openEditModal(server)}>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#cdd6f4]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                        </button>
                        <button class="p-2 hover:bg-red-500/20 rounded-md" on:click={() => openDeleteModal(server)}>
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        {/each}

        <button
            on:click={toggleModal}
            class="p-6 bg-gradient-to-br from-[#313244]/20 to-transparent hover:from-[#313244]/30 hover:to-[#313244]/10 
                   rounded-lg border border-dashed border-[#6e6c7e]/40 flex items-center justify-center gap-2 
                   transition-all duration-200 relative group">
            <div class="absolute inset-0 bg-gradient-to-r from-[#89b4fa]/10 to-[#cba6f7]/10 opacity-0 
                      group-hover:opacity-100 transition-opacity rounded-lg"></div>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-[#cdd6f4]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span class="text-[#cdd6f4] font-medium">Add New Server</span>
        </button>
    </div>
</div>

{#if showModal}
    <div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <div class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-md">
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">Add New Server</h2>
                <form method="POST" action="?/addServer" class="space-y-4">
                    <div>
                        <label for="name" class="block text-sm text-[#cdd6f4] mb-2">Server Name</label>
                        <input type="text" name="name" id="name" required
                            class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                   text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div>
                        <label for="url" class="block text-sm text-[#cdd6f4] mb-2">Server URL</label>
                        <input type="url" name="url" id="url" required
                            class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                   text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div class="flex justify-end gap-3 pt-4">
                        <button type="button" on:click={toggleModal}
                            class="px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#313244]/30 rounded-md transition-colors">
                            Cancel
                        </button>
                        <button type="submit"
                            class="px-4 py-2 text-sm bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7] 
                                   text-[#1e1e2e] rounded-md font-medium transition-colors">
                            Add Server
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
{/if}

{#if showDeleteModal}
    <div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <div class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-md">
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">Delete Server</h2>
                <p class="text-[#a6adc8]">Are you sure you want to delete {selectedServer?.name}?</p>
                <form method="POST" action="?/deleteServer" class="flex justify-end gap-3 pt-4">
                    <input type="hidden" name="id" value={selectedServer?.id}>
                    <button type="button" on:click={() => showDeleteModal = false}
                        class="px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#313244]/30 rounded-md transition-colors">
                        Cancel
                    </button>
                    <button type="submit"
                        class="px-4 py-2 text-sm bg-red-500 hover:bg-red-600 text-white rounded-md font-medium transition-colors">
                        Delete
                    </button>
                </form>
            </div>
        </div>
    </div>
{/if}

{#if showEditModal}
    <div class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <div class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-md">
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">Edit Server</h2>
                <form method="POST" action="?/updateServer" class="space-y-4">
                    <input type="hidden" name="id" value={selectedServer.id}>
                    <div>
                        <label for="edit-name" class="block text-sm text-[#cdd6f4] mb-2">Server Name</label>
                        <input type="text" name="name" id="edit-name" required
                            bind:value={selectedServer.name}
                            class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                   text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div>
                        <label for="edit-url" class="block text-sm text-[#cdd6f4] mb-2">Server URL</label>
                        <input type="url" name="url" id="edit-url" required
                            bind:value={selectedServer.url}
                            class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30 
                                   text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent">
                    </div>
                    <div class="flex justify-end gap-3 pt-4">
                        <button type="button" on:click={() => showEditModal = false}
                            class="px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#313244]/30 rounded-md transition-colors">
                            Cancel
                        </button>
                        <button type="submit"
                            class="px-4 py-2 text-sm bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7] 
                                   text-[#1e1e2e] rounded-md font-medium transition-colors">
                            Save Changes
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
{/if}
