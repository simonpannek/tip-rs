<script>
	import Event from '$lib/Event.svelte';

	export let data;
</script>

<ul class="grid gap-6">
	{#each data.db.events as event}
		<Event {event} />
	{/each}
</ul>
