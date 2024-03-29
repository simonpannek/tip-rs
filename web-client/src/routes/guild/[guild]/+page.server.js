import { error } from '@sveltejs/kit';

import { Guild, Event, User } from '$lib/server/database.js';

export async function load({ locals, params }) {
	if (isNaN(params.guild)) {
		throw error(400, 'Invalid guild id.');
	}

	const guildId = BigInt(params.guild);

	if (locals.guilds) {
		// Get user events
		const events = await Event.findAll({
			attributes: ['id', 'name', 'description'],
			include: [
				{
					model: Guild,
					attributes: [],
					where: {
						id: guildId,
						ignore: false
					}
				},
				{
					model: User,
					attributes: [],
					where: {
						id: locals.user.id
					}
				},
				{
					model: User,
					as: 'owner',
					attributes: ['id', 'name', 'avatar']
				}
			]
		}).then((events) => events.map((event) => JSON.parse(JSON.stringify(event))));

		if (!events.length && !locals.guilds.has(guildId)) {
			throw error(403, 'You are not allowed to access this server.');
		}

		return { db: { events } };
	} else {
		throw error(401, 'Could not check whether you are a member of this server.');
	}
}
