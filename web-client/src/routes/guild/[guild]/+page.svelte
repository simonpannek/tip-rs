<script>
	import { page } from '$app/stores';

	import { guilds } from '$lib/auth.js';

	import Event from '$lib/Event.svelte';

	export let data;

	$: guild = $guilds.get(BigInt($page.params.guild));
</script>

<h2 class="text-4xl text-center">{guild.name || 'Unknown server'}</h2>

<ul class="grid grid-cols-2 gap-6 mt-6">
	{#each data.db.events as event}
		<Event {event} />
	{/each}
</ul>
