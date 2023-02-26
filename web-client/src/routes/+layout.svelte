<script>
	import '../app.css';

	import { user } from '$lib/auth.js';

	export let data;

	$: user && user.set(data.user);
</script>

<svelte:head>
	<!-- Title -->
	<title>Tip - An event management bot</title>
	<!-- Meta description -->
	<meta name="description" content="Tip is a small discord bot for event planning/management, controlled by a web interface.">
</svelte:head>

<main>
	<slot />
</main>
