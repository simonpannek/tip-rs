export function GET() {
	// Set cookies and redirect client to main page
	return new Response(null, {
		headers: {
			'Set-Cookie': [
				'access_token=logout; Path=/; HttpOnly; SameSite=None; Secure; Expires=Thu, 01 Jan 1970 00:00:00 GMT',
				'refresh_token=logout; Path=/; HttpOnly; SameSite=None; Secure; Expires=Thu, 01 Jan 1970 00:00:00 GMT'
			],
			Location: '/'
		},
		status: 302
	});
}
