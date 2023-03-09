<script>
	import { guilds } from '$lib/auth.js';

	import Guild from '$lib/Guild.svelte';

	export let data;
</script>

<ul>
	{#each data.db.guilds as { id }}
		<Guild guild={$guilds.get(id)} />
	{/each}
</ul>
