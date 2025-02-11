import { db } from '$lib/server/db';
import { user } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { error } from '@sveltejs/kit';

export const load = async ({ fetch, params }) => {
    const servers = await db.select().from(user).where(eq(user.id, Number(params.server)));
    if (!servers.length) {
        throw error(404, 'Server not found');
    }
    
    const server = servers[0];
    
    try {
        const [project, deployments] = await Promise.all([
            fetch(`/api/servers/${server.id}?path=/projects/${params.project}`).then(r => r.json()),
            fetch(`/api/servers/${server.id}?path=/projects/${params.project}/deployments`).then(r => r.json())
        ]);
        
        let latestDeployment = null;
        if (deployments.length > 0) {
            latestDeployment = deployments[0];
            try {
                const logsRes = await fetch(
                    `/api/servers/${server.id}?path=/projects/${params.project}/deployments/${latestDeployment.id}`
                );
                if (logsRes.ok) {
                    const logsData = await logsRes.json();
                    latestDeployment.logs = logsData.logs;
                }
            } catch (e) {
                console.error('Error fetching latest deployment logs:', e);
            }
        }
        
        return {
            server,
            project,
            deployments,
            latestDeployment
        };
    } catch (e) {
        console.error('Error fetching project:', e);
        throw error(500, 'Failed to fetch project data');
    }
};

export const actions = {
    updateProject: async ({ fetch, params, request }) => {
        const data = await request.formData();
        const projectData = {
            name: data.get('name'),
            git_repo: data.get('git_repo'),
            install_cmd: data.get('install_cmd') || null,
            build_cmd: data.get('build_cmd') || null,
            run_cmd: data.get('run_cmd'),
            env: data.get('env') || null,
            healthcheck_endpoint: data.get('healthcheck_endpoint') || null,
            healthcheck_timeout: Number(data.get('healthcheck_timeout')) || 30
        };

        try {
            const res = await fetch(`/api/servers/${params.server}?path=/projects/${params.project}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(projectData)
            });

            if (!res.ok) throw new Error('Failed to update project');
            return { success: true };
        } catch (e) {
            return { error: 'Failed to update project' };
        }
    },

    deploy: async ({ fetch, params }) => {
        try {
            const res = await fetch(`/api/servers/${params.server}?path=/projects/${params.project}/deploy`, {
                method: 'POST'
            });

            if (!res.ok) throw new Error('Failed to start deployment');
            
            // Return the new deployment data
            const deployment = await res.json();
            return { 
                success: true,
                deployment 
            };
        } catch (e) {
            return { error: 'Failed to start deployment' };
        }
    }
};
