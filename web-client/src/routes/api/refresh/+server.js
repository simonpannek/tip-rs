import { error } from '@sveltejs/kit';

import { DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET } from '$env/static/private';

export async function GET({ fetch, url }) {
	// Get refresh token
	const refresh_token = url.searchParams.get('refresh_token');

	if (!refresh_token) {
		throw error(400, 'No refresh token specified');
	}

	// Fetch Discord token from API
	const request = await fetch('https://discord.com/api/oauth2/token', {
		method: 'POST',
		body: new URLSearchParams({
			client_id: DISCORD_CLIENT_ID,
			client_secret: DISCORD_CLIENT_SECRET,
			grant_type: 'refresh_token',
			redirect_uri: `${url.origin}/api/callback`,
			refresh_token,
			scope: 'identify'
		}),
		headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
	});

	// Parse response
	const response = await request.json();

	if (response.error) {
		throw error(400, 'Failed to refresh access token');
	}

	// Fetch dynamic expire time of access token
	const access_token_expires_in = new Date(Date.now() + response.expires_in);
	// Set refresh token expire time to 30 days
	const refresh_token_expires_in = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000);

	// Set cookies and redirect client to main page
	return new Response(null, {
		headers: {
			'Set-Cookie': [
				`access_token=${response.access_token}; Path=/; HttpOnly; SameSite=None; Secure; Expires=${access_token_expires_in}}`,
				`refresh_token=${response.refresh_token}; Path=/; HttpOnly; SameSite=None; Secure; Expires=${refresh_token_expires_in}`
			],
			Location: '/'
		},
		status: 302
	});
}
