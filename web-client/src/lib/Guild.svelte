<script>
	export let guild;

	function guildIcon(guild) {
		return guild.icon && `https://cdn.discordapp.com/icons/${guild.id}/${guild.icon}.webp`;
	}
</script>

<li id={guild.id}>
	<div class="flex flex-col items-center">
	<img
		src={guildIcon(guild)}
		alt={guild.name
			.split(' ')
			.map((word) => word[0])
			.slice(0, 3)
			.join('')
			.toUpperCase()}
	/>
	<h2>{guild.name}</h2>
	</div>
	<div style:background={`center/cover url(${guildIcon(guild)})`} class="blur absolute" />
</li>
