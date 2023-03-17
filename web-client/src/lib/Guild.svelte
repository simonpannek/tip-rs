<script>
	export let guild;

	$: icon = guild.icon ? `https://cdn.discordapp.com/icons/${guild.id}/${guild.icon}.webp` : '';
</script>

<li id={guild.id} class="h-80 relative rounded-xl overflow-clip drop-shadow-md text-primary">
	<a href="/guild/{guild.id}" class="hover:opacity-95 ease-linear duration-100">
		<div
			style:background="center/cover rgba(0, 0, 0, 0.7) url({icon})"
			class="h-full blur-lg scale-125 bg-blend-darken"
		/>
		<div
			class="flex flex-col items-center absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
		>
			<img
				src={icon}
				alt={guild.name
					.split(' ')
					.map((word) => word[0])
					.slice(0, 3)
					.join('')
					.toUpperCase()}
				class="w-32 h-32 bg-secondary-800 text-4xl leading-[8rem] text-center align-middle rounded-full"
			/>
			<h2 class="text-2xl mt-3">{guild.name}</h2>
		</div>
	</a>
</li>
