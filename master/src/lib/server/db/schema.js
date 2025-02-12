import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const user = sqliteTable('servers', {
	id: integer('id').primaryKey(),
	name: text('name').notNull(),
	url: text('url').notNull(),
});

export const projects = sqliteTable('projects', {
	id: integer('id').primaryKey(),
	server_id: integer('server_id').notNull(),
	name: text('name').notNull(),
	domain: text('domain'),
});
