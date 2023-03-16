<script>
	import { guilds } from '$lib/auth.js';

	import Guild from '$lib/Guild.svelte';

	export let data;
</script>

<ul class="grid gap-6">
	{#each data.db.guilds as { id }}
		<Guild guild={$guilds.get(BigInt(id))} />
	{/each}
</ul>
