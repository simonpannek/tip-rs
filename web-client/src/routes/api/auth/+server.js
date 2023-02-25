import { DISCORD_CLIENT_ID, VERCEL_URL } from '$env/static/private';

export function GET() {
	const endpoint = `https://discord.com/api/oauth2/authorize?client_id=${DISCORD_CLIENT_ID}&redirect_uri=https://${VERCEL_URL}/&response_type=code&scope=identify%20email`;

	return new Response(null, {
		headers: {
			Location: endpoint
		},
		status: 302
	});
}
