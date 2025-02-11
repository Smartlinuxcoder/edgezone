import { db } from '$lib/server/db';
import { user } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { error, json } from '@sveltejs/kit';

export async function GET({ params, fetch }) {
    const servers = await db.select().from(user).where(eq(user.id, Number(params.server)));
    if (!servers.length) {
        throw error(404, 'Server not found');
    }

    try {
        const res = await fetch(
            `/api/servers/${params.server}?path=/projects/${params.project}/deployments/${params.deployment}`
        );
        
        if (!res.ok) {
            throw error(500, 'Failed to fetch deployment logs');
        }

        const data = await res.json();
        return json(data);
    } catch (e) {
        console.error('Error fetching deployment:', e);
        throw error(500, 'Failed to fetch deployment data');
    }
}
