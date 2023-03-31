<script>
	export let data;

	let event = data.db.event;
</script>

<h2 class="text-4xl text-center">{event.name}</h2>
<h3 class="text-xl text-center">{event.description}</h3>

<a href="https://discordapp.com/users/{event.owner.id}" target="_blank" rel="noreferrer">
	<img
		src="https://cdn.discordapp.com/avatars/{event.owner.id}/{event.owner.avatar}.png"
		alt="{event.owner.name}'s avatar."
		class="absolute bottom-0 right-0 rounded-full h-16 m-4"
		title="Event owner: {event.owner.name}"
	/>
</a>
