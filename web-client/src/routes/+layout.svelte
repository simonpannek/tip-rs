<script>
	import '../app.css';

	import { user, guilds } from '$lib/auth.js';

	import Header from '$lib/Header.svelte';
	import Footer from '$lib/Footer.svelte';

	export let data;

	$: $user = data.user;
	$: $guilds = data.guilds;
</script>

<svelte:head>
	<!-- Title -->
	<title>Tip - An event management bot</title>
	<!-- Meta description -->
	<meta
		name="description"
		content="Tip is a small discord bot for event planning/management, controlled by a web interface."
	/>
</svelte:head>

<div class="flex flex-col justify-between min-h-screen">
	<Header />

	<main class="relative grow p-8 overflow-y-hidden">
		<slot />
	</main>

	<Footer />
</div>
