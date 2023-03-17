import { error } from '@sveltejs/kit';

import { Guild, Event, User } from '$lib/server/database.js';

export async function load({ locals, params }) {
	if (isNaN(params.guild)) {
		throw error(400, 'Invalid guild id.');
	}

	const guildId = BigInt(params.guild);

	if (locals.guilds) {
		// Get user events
		const events = await Guild.findAll({
			attributes: [],
			where: {
				id: guildId,
				ignore: false
			},
			include: [
				{
					model: Event,
					attributes: ['id', 'name', 'owner_id'],
					include: [
						{
							model: User,
							attributes: [],
							where: {
								id: locals.user.id
							}
						}
					]
				}
			]
		}).then((events) => events[0]['Events'].map((event) => JSON.parse(JSON.stringify(event))));

		if (!events.length && !locals.guilds.has(guildId)) {
			throw error(403, 'You are not allowed to access this server.');
		}

		return { db: { events } };
	} else {
		throw error(401, 'Could not check whether you are a member of this server.');
	}
}
