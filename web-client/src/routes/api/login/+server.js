import { DISCORD_CLIENT_ID } from '$env/static/private';

export function GET({ url }) {
	const endpoint = `https://discord.com/api/oauth2/authorize?client_id=${DISCORD_CLIENT_ID}&redirect_uri=${url.origin}/api/callback&response_type=code&scope=identify%20guilds`;

	return new Response(null, {
		headers: {
			Location: endpoint
		},
		status: 302
	});
}
