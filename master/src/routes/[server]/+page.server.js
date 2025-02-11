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
        const projectsRes = await fetch(`/api/servers/${server.id}?path=/projects`);
        if (!projectsRes.ok) {
            throw new Error('Failed to fetch projects');
        }
        const projects = await projectsRes.json();
        
        return {
            server,
            projects
        };
    } catch (e) {
        console.error('Error fetching projects:', e);
        return {
            server,
            projects: [],
            error: 'Failed to connect to server'
        };
    }
};
