import { Guild } from '$lib/server/database.js';

export async function load({ locals }) {
	if (locals.guilds) {
		// Get user guilds
		const guilds = await Guild.findAll({
			attributes: ['id'],
			where: {
				id: Array.from(locals.guilds.keys()),
				ignore: false
			}
		}).then((guilds) => guilds.map((guild) => JSON.parse(JSON.stringify(guild))));

		return { db: { guilds } };
	} else {
		return { db: { guilds: [] } };
	}
}
