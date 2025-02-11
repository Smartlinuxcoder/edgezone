import { db } from '$lib/server/db';
import { user } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { error, json } from '@sveltejs/kit';

async function getServer(serverId) {
    const servers = await db.select().from(user).where(eq(user.id, Number(serverId)));
    if (!servers.length) {
        throw error(404, 'Server not found');
    }
    return servers[0];
}

export async function GET({ params, url }) {
    const server = await getServer(params.server);
    const path = url.searchParams.get('path') || '';

    
    try {
        const response = await fetch(`${server.url}${path}`);
        const data = await response.json();
        return json(data);
    } catch (e) {
        throw error(500, 'Failed to fetch from server');
    }
}

export async function POST({ params, url, request }) {
    const server = await getServer(params.server);
    const path = url.searchParams.get('path') || '';

    try {
        const body = await request.json().catch(() => ({}));
        const response = await fetch(`${server.url}${path}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: JSON.stringify(body)
        });

        if (!response.ok) {
            const errorText = await response.text();
            console.error('Server error response:', errorText);
            throw error(response.status, 'Server request failed');
        }

        const text = await response.text();
        if (!text) {
            return json({});
        }

        try {
            const data = JSON.parse(text);
            return json(data);
        } catch (e) {
            console.error('JSON parse error:', text);
            throw error(500, 'Invalid JSON response from server');
        }

    } catch (e) {
        console.error('Deployment error:', {
            message: e.message,
            stack: e.stack,
            url: `${server.url}${path}`
        });
        throw error(500, e.message || 'Failed to send or process data from server');
    }
}

export async function PUT({ params, url, request }) {
    const server = await getServer(params.server);
    const path = url.searchParams.get('path') || '';
    
    try {
        const body = await request.json();
        const response = await fetch(`${server.url}${path}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        });
        const data = await response.json();
        return json(data);
    } catch (e) {
        throw error(500, 'Failed to update data on server');
    }
}

export async function DELETE({ params, url }) {
    const server = await getServer(params.server);
    const path = url.searchParams.get('path') || '';
    
    try {
        const response = await fetch(`${server.url}${path}`, {
            method: 'DELETE'
        });
        const data = await response.json();
        return json(data);
    } catch (e) {
        throw error(500, 'Failed to delete from server');
    }
}
