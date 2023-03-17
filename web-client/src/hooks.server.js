/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	// Always handle api requests the normal way
	if (event.url.pathname.startsWith('/api/')) {
		return resolve(event);
	}

	const refresh_token = event.cookies.get('refresh_token');
	const access_token = event.cookies.get('access_token');

	// Access token has expired
	if (refresh_token && !access_token) {
		// Refresh access token
		return Response.redirect(`${event.url.origin}/api/refresh?refresh_token=${refresh_token}`, 302);
	}

	if (access_token) {
		// Request user information using the access token
		const user_request = await fetch('https://discordapp.com/api/users/@me', {
			headers: { Authorization: `Bearer ${access_token}` }
		});

		// Parse response
		const user_response = await user_request.json();

		if (user_response.id) {
			event.locals.user = {
				id: user_response.id,
				username: user_response.username,
				discriminator: user_response.discriminator,
				avatar: user_response.avatar
			};

			const test_request = await fetch('https://discordapp.com/api/guilds/780800506204651572/members/244459328847872000', {
				headers: { Authorization: `Bearer ${access_token}` }
			});

			const test_response = await test_request.json();

            console.log(test_response);

			// Request guilds information
			const guilds_request = await fetch('https://discordapp.com/api/users/@me/guilds', {
				headers: { Authorization: `Bearer ${access_token}` }
			});

			// Parse response
			const guilds_response = await guilds_request.json();

			if (guilds_response.length) {
				event.locals.guilds = new Map(
					guilds_response.map((guild) => [
						BigInt(guild.id),
						{
							id: guild.id,
							name: guild.name,
							owner: guild.owner,
							icon: guild.icon
						}
					])
				);
			}
		}
	}

	return resolve(event);
}
