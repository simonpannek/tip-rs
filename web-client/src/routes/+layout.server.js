export function load({ locals }) {
	return {
		user: locals.user,
		guilds: locals.guilds
	};
}
