import { db } from '$lib/server/db';
import { user } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { error, json } from '@sveltejs/kit';

export async function POST({ params, fetch }) {
    const servers = await db.select().from(user).where(eq(user.id, Number(params.server)));
    if (!servers.length) {
        throw error(404, 'Server not found');
    }

    try {
        const deployRes = await fetch(`/api/servers/${params.server}?path=/projects/${params.project}/deploy`, {
            method: 'POST'
        });
        if (!deployRes.ok) {
            throw error(500, 'Failed to start deployment');
        }

        const deployment = await deployRes.json();

        let attempts = 0;
        const maxAttempts = 3;
        let finalDeployment = deployment;

        while (attempts < maxAttempts && finalDeployment.status === 0) {
            await new Promise(resolve => setTimeout(resolve, 2000));

            const statusRes = await fetch(
                `/api/servers/${params.server}?path=/projects/${params.project}/deployments/${deployment.id}`
            );

            if (statusRes.ok) {
                finalDeployment = await statusRes.json();
            }

            attempts++;
        }

        return json(finalDeployment);
    } catch (e) {
        console.error('Deployment error:', e);
        throw error(500, 'Failed to execute deployment');
    }
}

export async function GET({ params, fetch }) {
    const servers = await db.select().from(user).where(eq(user.id, Number(params.server)));
    if (!servers.length) {
        throw error(404, 'Server not found');
    }

    try {
        // Get latest deployment
        const deploymentsRes = await fetch(
            `/api/servers/${params.server}?path=/projects/${params.project}/deployments`
        );

        if (!deploymentsRes.ok) {
            throw error(500, 'Failed to fetch deployments');
        }

        const deployments = await deploymentsRes.json();
        if (!deployments.length) {
            return json({ status: null });
        }

        const latestDeployment = deployments[0];
        return json(latestDeployment);
    } catch (e) {
        console.error('Error fetching deployment status:', e);
        throw error(500, 'Failed to get deployment status');
    }
}
