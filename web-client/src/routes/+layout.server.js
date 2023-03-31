export const trailingSlash = 'always';

export function load({ locals }) {
	return {
		user: locals.user,
		guilds: locals.guilds
	};
}
