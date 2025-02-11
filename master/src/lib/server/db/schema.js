import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const user = sqliteTable('servers', {
	id: integer('id').primaryKey(),
	name: text('name').notNull(),
	url: text('url').notNull(),
});
