import { error } from '@sveltejs/kit';

import { Guild, Event, User } from '$lib/server/database.js';

export async function load({ locals, params }) {
	if (isNaN(params.guild)) {
		throw error(400, 'Invalid guild id.');
	}

	if (isNaN(params.event)) {
		throw error(400, 'Invalid event id.');
	}

	const guildId = BigInt(params.guild);
	const eventId = BigInt(params.event);

	if (locals.guilds) {
		// Get event info
		const event = await Event.findByPk(eventId, {
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
		}).then((event) => JSON.parse(JSON.stringify(event)));

		if (!event) {
			throw error(403, 'You are not allowed to access this event.');
		}

		return { db: { event } };
	} else {
		throw error(401, 'Could not check whether you are a member of this server.');
	}
}
