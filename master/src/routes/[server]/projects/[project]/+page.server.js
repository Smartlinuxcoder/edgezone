import { db } from '$lib/server/db';
import { user, projects } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { error } from '@sveltejs/kit';

export const load = async ({ fetch, params }) => {
    const servers = await db.select().from(user).where(eq(user.id, Number(params.server)));
    if (!servers.length) {
        throw error(404, 'Server not found');
    }
    
    const server = servers[0];
    
    try {
        const project = await fetch(`/api/servers/${server.id}?path=/projects/${params.project}`).then(r => r.json());
        
        return {
            server,
            project
        };
    } catch (e) {
        console.error('Error fetching project:', e);
        throw error(500, 'Failed to fetch project data');
    }
};

export const actions = {
    updateProject: async ({ fetch, params, request }) => {
        const data = await request.formData();
        const domain = data.get('domain');

        try {
            if (domain !== undefined) {
                await db.update(projects)
                    .set({ domain })
                    .where(eq(projects.id, Number(params.project)));
            }
            
            const project = {
                name: data.get('name'),
                git_repo: data.get('git_repo'),
                install_cmd: data.get('install_cmd') || null,
                build_cmd: data.get('build_cmd') || null,
                run_cmd: data.get('run_cmd'),
                env: data.get('env') || null,
                healthcheck_endpoint: data.get('healthcheck_endpoint') || null,
                healthcheck_timeout: Number(data.get('healthcheck_timeout')) || 30
            };

            const res = await fetch(`/api/servers/${params.server}?path=/projects/${params.project}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(project)
            });
            
            if (!res.ok) throw new Error('Failed to update project');
                
            return { success: true };
        } catch (e) {
            console.error('Failed to update project:', e);
            return { error: 'Failed to update project' };
        }
    },

    deploy: async ({ fetch, params }) => {
        try {
            const res = await fetch(`/api/servers/${params.server}?path=/projects/${params.project}/deploy`, {
                method: 'POST'
            });

            if (!res.ok) throw new Error('Failed to start deployment');
            
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
