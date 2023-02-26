/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	console.log(event);

	const refresh_token = event.cookies.get('refresh_token');
	const access_token = event.cookies.get('access_token');

	console.log(refresh_token);
	console.log(access_token);

	// Access token has expired
	if (refresh_token && !access_token) {
		// Refresh access token
		return Response.redirect(`${event.url.origin}/api/refresh?refresh_token=${refresh_token}`, 302);
	}

	if (access_token) {
		// Request user information using the access token
		const request = await fetch('https://discordapp.com/api/users/@me', {
			headers: { Authorization: `Bearer ${access_token}` }
		});

		// Parse response
		const response = await request.json();

		if (response.id) {
			event.locals.user = {
				...response
			};
		}
	}

	return resolve(event);
}
