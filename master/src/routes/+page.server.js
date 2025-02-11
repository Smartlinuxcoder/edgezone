import { db } from '$lib/server/db';
import { user } from '$lib/server/db/schema';
import { fail } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';

export const load = async () => {
	const servers = await db.select().from(user);
	return { servers };
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
	}
};
