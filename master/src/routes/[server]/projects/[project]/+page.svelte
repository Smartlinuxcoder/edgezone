<script>
    import { onMount, onDestroy } from "svelte";
    export let data;
    let showEditModal = false;
    let editedProject = { ...data.project };
    let selectedDeployment = null;
    let deploymentLogs = "";
    let showLogsModal = false;
    let deployments = [];
    let pollingInterval;
    let isDeploying = false;
    let deployError = null;
    let showEnvModal = false;
    let envVars = data.project.env ? parseEnvString(data.project.env) : [];


    let activeTab = "config";

    function switchTab(tab) {
        activeTab = tab;
    }

    function parseEnvString(envStr) {
        if (!envStr) return [];
        return envStr
            .split("\n")
            .filter((line) => line.trim() && !line.startsWith("#"))
            .map((line) => {
                const [key, ...values] = line.split("=");
                return { key: key.trim(), value: values.join("=").trim() };
            });
    }

    function stringifyEnvVars(vars) {
        return vars.map((v) => `${v.key}=${v.value}`).join("\n");
    }

    function addEnvVar() {
        envVars = [...envVars, { key: "", value: "" }];
    }

    function removeEnvVar(index) {
        envVars = envVars.filter((_, i) => i !== index);
    }

    async function saveEnvVars() {
        editedProject.env = stringifyEnvVars(envVars);
        const formData = new FormData();
        Object.entries(editedProject).forEach(([key, value]) => {
            formData.append(key, value ?? "");
        });

        await fetch(`?/updateProject`, {
            method: "POST",
            body: formData,
        });
        showEnvModal = false;
    }

    async function pollDeployments() {
        try {
            const res = await fetch(
                `/api/servers/${data.server.id}?path=/projects/${data.project.id}/deployments`,
            );
            if (res.ok) {
                const newDeployments = await res.json();
                // Sort deployments by ID in descending order (newest first)
                deployments = newDeployments.sort((a, b) => b.id - a.id);
            }
        } catch (e) {
            console.error("Error polling deployments:", e);
        }
    }

    async function viewDeploymentLogs(deployment) {
        try {
            const res = await fetch(
                `/api/servers/${data.server.id}?path=/projects/${data.project.id}/deployments/${deployment.id}`,
            );
            if (!res.ok) throw new Error("Failed to fetch logs");
            const logData = await res.json();
            deploymentLogs = logData.logs;
            selectedDeployment = deployment;
            showLogsModal = true;
        } catch (e) {
            console.error("Error fetching logs:", e);
        }
    }

    async function startDeployment() {
        isDeploying = true;
        deployError = null;

        try {
            const res = await fetch(
                `/api/servers/${data.server.id}/projects/${data.project.id}/deploy`,
                { method: "POST" },
            );

            if (!res.ok) throw new Error("Failed to start deployment");

            const newDeployment = await res.json();
            deployments = [newDeployment, ...deployments].sort(
                (a, b) => b.id - a.id,
            );

            const pollDeployment = async () => {
                if (newDeployment.status === 0) {
                    const statusRes = await fetch(
                        `/api/servers/${data.server.id}/projects/${data.project.id}/deploy`,
                    );
                    if (statusRes.ok) {
                        const updated = await statusRes.json();
                        deployments = deployments.map((d) =>
                            d.id === updated.id ? updated : d,
                        );
                        if (updated.status === 0) {
                            setTimeout(pollDeployment, 2000);
                        }
                    }
                }
            };

            pollDeployment();
        } catch (e) {
            console.error("Deployment error:", e);
            deployError = "Failed to start deployment";
        } finally {
            isDeploying = false;
        }
    }

    async function saveProjectSettings(event) {
        event.preventDefault();
        const formData = new FormData();
        formData.append('domain', editedProject.domain);

        const response = await fetch('?/updateProject', {
            method: 'POST',
            body: formData
        });

        if (response.ok) {
            showEditModal = false;
            // Update local data
            data.project.domain = editedProject.domain;
        }
    }

    onMount(() => {
        pollDeployments(); // Initial fetch
        pollingInterval = setInterval(pollDeployments, 5000);
    });

    onDestroy(() => {
        if (pollingInterval) clearInterval(pollingInterval);
    });

    $: {
        if (data.deployments) {
            deployments = [...data.deployments].sort((a, b) => b.id - a.id);
        }
    }

    $: statusColor = (status) => {
        switch (status) {
            case 0:
                return "text-yellow-400"; // PENDING
            case 1:
                return "text-blue-400"; // INSTALLING
            case 2:
                return "text-purple-400"; // BUILDING
            case 3:
                return "text-green-400"; // RUNNING
            case 4:
                return "text-red-400"; // FAILED
            case 5:
                return "text-gray-400"; // STOPPED
            default:
                return "text-gray-400";
        }
    };

    $: statusText = (status) => {
        switch (status) {
            case 0:
                return "Pending";
            case 1:
                return "Installing";
            case 2:
                return "Building";
            case 3:
                return "Running";
            case 4:
                return "Failed";
            case 5:
                return "Stopped";
            default:
                return "Unknown";
        }
    };
</script>

<div class="container mx-auto max-w-7xl">
    <header class="mb-8">
        <div class="flex items-center gap-4 mb-4">
            <a
                href="/{data.server.id}"
                class="text-[#cdd6f4] hover:text-[#89b4fa] transition-colors"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M10 19l-7-7m0 0l7-7m-7 7h18"
                    />
                </svg>
            </a>
            <div class="flex-1">
                <h1
                    class="text-[#cdd6f4] text-3xl font-bold flex items-center gap-3"
                >
                    {data.project.name}
<!--                     <span
                        class={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${statusColor(data.project.status)}`}
                    >
                        {statusText(data.project.status)}
                    </span> -->
                </h1>
                <p class="text-[#a6adc8] mt-1 flex items-center gap-2">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                        />
                    </svg>
                    {data.project.git_repo}
                </p>
            </div>
            <div class="flex items-center gap-3">
                <button
                    on:click={() =>
                        window.open(`https://${data.project.domain}`, "_blank")}
                    class="px-4 py-2 bg-[#313244]/30 hover:bg-[#313244]/50 text-[#cdd6f4] rounded-md transition-colors flex items-center gap-2"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                        />
                    </svg>
                    Visit
                </button>
                <button
                    on:click={startDeployment}
                    disabled={isDeploying}
                    class="px-4 py-2 bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7]
                           text-[#1e1e2e] rounded-md font-medium transition-all duration-200 disabled:opacity-50
                           disabled:cursor-not-allowed flex items-center gap-2"
                >
                    {#if isDeploying}
                        <svg
                            class="animate-spin h-4 w-4"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <circle
                                class="opacity-25"
                                cx="12"
                                cy="12"
                                r="10"
                                stroke="currentColor"
                                stroke-width="4"
                            ></circle>
                            <path
                                class="opacity-75"
                                fill="currentColor"
                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                            ></path>
                        </svg>
                        Deploying...
                    {:else}
                        Deploy
                    {/if}
                </button>
            </div>
        </div>
    </header>

    <!-- Tab Navigation -->
    <nav class="flex border-b border-[#6e6c7e]/20 mb-8">
        <button
            class="px-6 py-3 font-medium text-sm {activeTab === 'config'
                ? 'text-[#89b4fa] border-b-2 border-[#89b4fa]'
                : 'text-[#a6adc8] hover:text-[#cdd6f4]'}"
            on:click={() => switchTab("config")}
        >
            Configuration
        </button>
        <button
            class="px-6 py-3 font-medium text-sm {activeTab === 'deployments'
                ? 'text-[#89b4fa] border-b-2 border-[#89b4fa]'
                : 'text-[#a6adc8] hover:text-[#cdd6f4]'}"
            on:click={() => switchTab("deployments")}
        >
            Deployments
        </button>
    </nav>

    {#if activeTab === "config"}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <div class="space-y-6">
                <div
                    class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-6"
                >
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-xl font-semibold text-[#cdd6f4]">
                            Project Settings
                        </h2>
                        <button
                            on:click={() => (showEditModal = true)}
                            class="p-2 hover:bg-[#1e1e2e]/50 rounded-md transition-colors"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5 text-[#cdd6f4]"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                                />
                            </svg>
                        </button>
                    </div>
                    <dl class="space-y-3">
                        <div>
                            <dt class="text-[#a6adc8] text-sm">Domain</dt>
                            <dd class="text-[#cdd6f4]">
                                {data.project.domain || "Not set"}
                            </dd>
                        </div>
                        <div>
                            <dt class="text-[#a6adc8] text-sm">
                                Git Repository
                            </dt>
                            <dd class="text-[#cdd6f4] font-mono text-sm">
                                {data.project.git_repo}
                            </dd>
                        </div>
                    </dl>
                </div>

                <div
                    class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-6"
                >
                    <h2 class="text-xl font-semibold text-[#cdd6f4] mb-4">
                        Build & Run Commands
                    </h2>
                    <dl class="space-y-3">
                        <div>
                            <dt class="text-[#a6adc8] text-sm">
                                Install Command
                            </dt>
                            <dd class="text-[#cdd6f4] font-mono">
                                {data.project.install_cmd || "No command"}
                            </dd>
                        </div>
                        <div>
                            <dt class="text-[#a6adc8] text-sm">
                                Build Command
                            </dt>
                            <dd class="text-[#cdd6f4] font-mono">
                                {data.project.build_cmd || "No command"}
                            </dd>
                        </div>
                        <div>
                            <dt class="text-[#a6adc8] text-sm">
                                Start Command
                            </dt>
                            <dd class="text-[#cdd6f4] font-mono">
                                {data.project.run_cmd}
                            </dd>
                        </div>
                    </dl>
                </div>
            </div>

            <!-- Environment Variables -->
            <div
                class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-6"
            >
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-xl font-semibold text-[#cdd6f4]">
                        Environment Variables
                    </h2>
                    <button
                        on:click={() => (showEnvModal = true)}
                        class="p-2 hover:bg-[#1e1e2e]/50 rounded-md transition-colors"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5 text-[#cdd6f4]"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                            />
                        </svg>
                    </button>
                </div>
                {#if data.project.env}
                    <div class="font-mono text-sm space-y-2">
                        {#each parseEnvString(data.project.env) as envVar}
                            <div
                                class="flex items-center justify-between py-2 px-3 bg-[#1e1e2e]/30 rounded"
                            >
                                <span class="text-[#cdd6f4]">{envVar.key}</span>
                                <span class="text-[#6e6c7e]">•••••••••</span>
                            </div>
                        {/each}
                    </div>
                {:else}
                    <p class="text-[#a6adc8]">No environment variables set</p>
                {/if}
            </div>
        </div>
    {:else}
        <!-- Deployments Tab -->
        <div class="space-y-4">
            <div class="flex justify-between items-center">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">
                    Deployments
                </h2>
                <div class="flex items-center gap-4">
                    {#if deployError}
                        <p class="text-red-400 text-sm">{deployError}</p>
                    {/if}
                    <button
                        on:click={startDeployment}
                        disabled={isDeploying}
                        class="px-4 py-2 bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec] hover:to-[#f5c2e7]
                               text-[#1e1e2e] rounded-md font-medium transition-all duration-200 disabled:opacity-50
                               disabled:cursor-not-allowed flex items-center gap-2"
                    >
                        {#if isDeploying}
                            <svg
                                class="animate-spin h-4 w-4"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <circle
                                    class="opacity-25"
                                    cx="12"
                                    cy="12"
                                    r="10"
                                    stroke="currentColor"
                                    stroke-width="4"
                                ></circle>
                                <path
                                    class="opacity-75"
                                    fill="currentColor"
                                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                ></path>
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
                    <div
                        class="bg-gradient-to-br from-[#313244]/30 to-[#313244]/10 rounded-lg border border-[#6e6c7e]/20 p-6"
                    >
                        <div class="flex items-start justify-between mb-4">
                            <div>
                                <div class="flex items-center gap-2 mb-1">
                                    <p
                                        class={`text-sm font-medium ${statusColor(deployment.status)}`}
                                    >
                                        {statusText(deployment.status)}
                                    </p>
                                    <span class="text-[#6e6c7e] text-sm">•</span
                                    >
                                    <p class="text-[#a6adc8] text-sm">
                                        {new Date(
                                            deployment.created_at,
                                        ).toLocaleString()}
                                    </p>
                                </div>
                                <p class="text-[#cdd6f4] font-mono text-sm">
                                    {deployment.commit_hash || "No commit hash"}
                                </p>
                                {#if deployment.commit_message}
                                    <p class="text-[#a6adc8] text-sm mt-1">
                                        {deployment.commit_message}
                                    </p>
                                {/if}
                            </div>
                            <button
                                on:click={() => viewDeploymentLogs(deployment)}
                                class="p-2 hover:bg-[#1e1e2e]/50 rounded-md transition-colors"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-5 w-5 text-[#cdd6f4]"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M4 6h16M4 12h16M4 18h16"
                                    />
                                </svg>
                            </button>
                        </div>
                        {#if deployment.status === 4}
                            <div
                                class="mt-4 p-4 bg-red-500/10 border border-red-500/20 rounded-md"
                            >
                                <p class="text-red-400 text-sm">
                                    {deployment.error || "Deployment failed"}
                                </p>
                            </div>
                        {/if}
                    </div>
                {/each}

                {#if deployments.length === 0}
                    <div class="text-center py-8">
                        <p class="text-[#a6adc8]">No deployments yet</p>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

{#if showLogsModal}
    <div
        class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    >
        <div
            class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-4xl"
        >
            <div class="p-6 space-y-4">
                <div class="flex items-center justify-between">
                    <h2 class="text-xl font-semibold text-[#cdd6f4]">
                        Deployment Logs
                    </h2>
                    <button
                        on:click={() => (showLogsModal = false)}
                        class="p-2 hover:bg-[#313244]/30 rounded-md transition-colors"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5 text-[#cdd6f4]"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                </div>
                <div
                    class="bg-[#11111b] rounded-lg p-4 max-h-[60vh] overflow-y-auto"
                >
                    <pre
                        class="font-mono text-sm text-[#cdd6f4] whitespace-pre-wrap">{deploymentLogs}</pre>
                </div>
            </div>
        </div>
    </div>
{/if}

{#if showEditModal}
    <div
        class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    >
        <div
            class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-2xl"
        >
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">
                    Edit Project
                </h2>
                <form on:submit|preventDefault={saveProjectSettings} class="space-y-4">
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2"
                                >Project Name</label
                            >
                            <input
                                type="text"
                                name="name"
                                required
                                bind:value={editedProject.name}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent"
                            />
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2"
                                >Domain</label
                            >
                            <input
                                type="text"
                                name="domain"
                                bind:value={editedProject.domain}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent"
                            />
                        </div>
                    </div>

                    <!-- Git and Commands -->
                    <div>
                        <label class="block text-sm text-[#cdd6f4] mb-2"
                            >Git Repository</label
                        >
                        <input
                            type="text"
                            name="git_repo"
                            required
                            bind:value={editedProject.git_repo}
                            class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                   text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent"
                        />
                    </div>

                    <div class="grid grid-cols-3 gap-4">
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2"
                                >Install Command</label
                            >
                            <input
                                type="text"
                                name="install_cmd"
                                bind:value={editedProject.install_cmd}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent"
                            />
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2"
                                >Build Command</label
                            >
                            <input
                                type="text"
                                name="build_cmd"
                                bind:value={editedProject.build_cmd}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent"
                            />
                        </div>
                        <div>
                            <label class="block text-sm text-[#cdd6f4] mb-2"
                                >Start Command</label
                            >
                            <input
                                type="text"
                                name="run_cmd"
                                required
                                bind:value={editedProject.run_cmd}
                                class="w-full px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent"
                            />
                        </div>
                    </div>

                    <div class="flex justify-end gap-3 pt-4">
                        <button
                            type="button"
                            on:click={() => (showEditModal = false)}
                            class="px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#313244]/30 rounded-md transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            class="px-4 py-2 text-sm bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec]
                                   hover:to-[#f5c2e7] text-[#1e1e2e] rounded-md font-medium transition-colors"
                        >
                            Save Changes
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
{/if}

{#if showEnvModal}
    <div
        class="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    >
        <div
            class="bg-gradient-to-b from-[#1e1e2e] to-[#181825] border border-[#6e6c7e]/20 rounded-lg shadow-xl w-full max-w-2xl"
        >
            <div class="p-6 space-y-4">
                <h2 class="text-xl font-semibold text-[#cdd6f4]">
                    Environment Variables
                </h2>
                <div class="space-y-4">
                    {#each envVars as envVar, i}
                        <div class="flex items-center gap-4">
                            <input
                                type="text"
                                placeholder="KEY"
                                bind:value={envVar.key}
                                class="flex-1 px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] font-mono text-sm"
                            />
                            <input
                                type="text"
                                placeholder="VALUE"
                                bind:value={envVar.value}
                                class="flex-1 px-3 py-2 rounded-md bg-[#313244]/30 border border-[#6e6c7e]/30
                                       text-[#cdd6f4] font-mono text-sm"
                            />
                            <button
                                on:click={() => removeEnvVar(i)}
                                class="p-2 hover:bg-red-500/20 rounded-md"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-5 w-5 text-red-400"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </button>
                        </div>
                    {/each}
                    <button
                        on:click={addEnvVar}
                        class="w-full py-2 px-4 border border-dashed border-[#6e6c7e]/40 rounded-md text-[#a6adc8]
                               hover:border-[#89b4fa]/40 hover:text-[#89b4fa] transition-colors"
                    >
                        Add Variable
                    </button>
                </div>
                <div class="flex justify-end gap-3 pt-4">
                    <button
                        on:click={() => (showEnvModal = false)}
                        class="px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#313244]/30 rounded-md transition-colors"
                    >
                        Cancel
                    </button>
                    <button
                        on:click={saveEnvVars}
                        class="px-4 py-2 text-sm bg-gradient-to-r from-[#89b4fa] to-[#cba6f7] hover:from-[#74c7ec]
                               hover:to-[#f5c2e7] text-[#1e1e2e] rounded-md font-medium transition-colors"
                    >
                        Save Changes
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
