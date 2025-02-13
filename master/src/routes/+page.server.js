import { db } from '$lib/server/db';
import { user } from '$lib/server/db/schema';
import { fail } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { versionCache } from '$lib/server/cache';

async function getLatestVersion() {
    const cached = versionCache.get('edgezone-node');
    if (cached) return cached;

    try {
        const response = await fetch('https://crates.io/api/v1/crates/edgezone-node', {
            headers: {
                'User-Agent': 'Edgezone/1.0.0 (smartcoder@smartlinux.xyz) - Self-hosted edge computing platform',
            }
        });

        if (!response.ok) {
            throw new Error('Failed to fetch version');
        }

        const data = await response.json();
        const version = data.crate?.max_version;
        
        if (version) {
            versionCache.set('edgezone-node', version);
            return version;
        }
        return null;
    } catch (error) {
        console.error('Failed to fetch latest version:', error);
        return null;
    }
}

export const load = async () => {
    const servers = await db.select().from(user);
    
    const [serversWithInfo, latestVersion] = await Promise.all([
        Promise.all(
            servers.map(async (server) => {
                try {
                    const response = await fetch(`${server.url}/info`);
                    const info = await response.json();
                    return { ...server, info };
                } catch (error) {
                    return { ...server, info: null };
                }
            })
        ),
        getLatestVersion()
    ]);

    return { servers: serversWithInfo, latestVersion };
};

export const actions = {
	addServer: async ({ request }) => {
		const data = await request.formData();
		const name = data.get('name');
		const url = data.get('url');

		if (!name || !url) {
			return fail(400, { error: 'Name and URL are required' });
		}

		try {
			await db.insert(user).values({ name, url });
			return { success: true };
		} catch (error) {
			return fail(500, { error: 'Failed to add server' });
		}
	},

	deleteServer: async ({ request }) => {
		const data = await request.formData();
		const id = Number(data.get('id'));

		if (!id) {
			return fail(400, { error: 'Server ID is required' });
		}

		try {
			await db.delete(user).where(eq(user.id, id));
			return { success: true };
		} catch (error) {
			return fail(500, { error: 'Failed to delete server' });
		}
	},

	updateServer: async ({ request }) => {
		const data = await request.formData();
		const id = Number(data.get('id'));
		const name = data.get('name');
		const url = data.get('url');

		if (!id || !name || !url) {
			return fail(400, { error: 'All fields are required' });
		}

		try {
			await db.update(user)
				.set({ name, url })
				.where(eq(user.id, id));
			return { success: true };
		} catch (error) {
			return fail(500, { error: 'Failed to update server' });
		}
	},

	updateNodeVersion: async ({ request }) => {
		const data = await request.formData();
		const serverId = Number(data.get('id'));

		if (!serverId) {
			return fail(400, { error: 'Server ID is required' });
		}

		try {
			const servers = await db.select().from(user).where(eq(user.id, serverId));
			const server = servers[0];
			console.log(server);
			if (!server) {
				return fail(404, { error: 'Server not found' });
			}

			const response = await fetch(`${server.url}/update`, {
				method: 'POST'
			});
			console.log(response);
			if (!response.ok) {
				throw new Error('Update failed');
			}

			return { success: true };
		} catch (error) {
			console.error('Failed to update server version:', error);
			return fail(500, { error: 'Failed to update server version' });
		}
	}
};
