<script>
	export let event;
</script>

<li id={event.id} class="h-60 relative rounded-xl overflow-clip drop-shadow-md text-primary">
	<a href="event/{event.id}/" class="hover:opacity-95 ease-linear duration-100">
		<div
			style:background="center/cover rgba(0, 0, 0, 0.7)"
			class="h-full blur-lg scale-125 bg-blend-darken"
		/>
		<div
			class="flex flex-col items-center absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
		>
			<h3 class="text-2xl mt-3">{event.name}</h3>
			<h4 class="text-xl mt-3">{event.description}</h4>
		</div>
	</a>

	<a href="https://discordapp.com/users/{event.owner.id}" target="_blank" rel="noreferrer">
		<img
			src="https://cdn.discordapp.com/avatars/{event.owner.id}/{event.owner.avatar}.png"
			alt="{event.owner.name}'s avatar."
			class="absolute bottom-0 right-0 rounded-full h-16 m-4"
			title="Event owner: {event.owner.name}"
		/>
	</a>
</li>
