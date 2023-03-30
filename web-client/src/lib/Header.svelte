<script>
	import { page } from '$app/stores';

	import { user } from '$lib/auth.js';

	import tip from '$lib/assets/tip.png';

	const navItems = [
		{ label: 'Home', href: '/' },
		{ label: 'About', href: '/about' },
		{ label: 'GitHub', href: 'https://github.com/simonpannek/tip-rs/', new_tab: true }
	];

	// Hide the menu on default
	let hideMenu = true;
</script>

<header class="bg-primary shadow w-full">
	<div
		class="container mx-auto p-4 flex flex-wrap items-center justify-center md:flex-no-wrap flat:flex-wrap"
	>
		<!-- Logo -->
		<div class="mr-4 md:mr-8 flat:mr-4 w-48">
			<a href="/" class="flex items-center gap-2">
				<img src={tip} alt="Tip the penguin." class="rounded-xl h-16" />
				<h1>Tip</h1>
			</a>
		</div>
		<!-- Menu button (mobile) -->
		<div class="ml-auto md:hidden flat:flex">
			<button
				class="flex items-center px-3 py-2 border rounded"
				type="button"
				on:click={() => (hideMenu = !hideMenu)}
			>
				<svg class="h-3 w-3" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
					<title>Menu</title>
					<path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" />
				</svg>
			</button>
		</div>
		<!-- Navigation list -->
		<nav
			class="w-full md:w-auto flat:w-full md:flex-grow md:flex flat:block md:items-center md:visible"
			class:hidden={hideMenu}
			class:flat:hidden={hideMenu}
		>
			<!-- First navigation column -->
			<ul class="mt-4 -mx-4 md:mx-0 md:mt-0 md:mr-4 lg:mr-8">
				{#each navItems as item}
					<li>
						<a
							sveltekit:prefetch
							href={item.href}
							class="block px-4 py-1 md:p-2 lg:px-4 border-accent2"
							class:text-accent2={item.href === $page.url.pathname}
							class:border-b-2={item.href === $page.url.pathname}
							target={item.new_tab && '_blank'}
							rel={item.new_tab && 'noreferrer'}
						>
							{item.label}
						</a>
					</li>
				{/each}
			</ul>
			<!-- Second navigation column -->
			<ul class="mt-4 -mx-4 md:mx-0 md:mt-0 md:ml-auto border-accent2">
				{#if $user}
					<li>
						<a href="/api/logout">Logout</a>
					</li>
					<li>
						<img
							src={`https://cdn.discordapp.com/avatars/${$user.id}/${$user.avatar}.png`}
							alt="Your discord avatar."
							class="rounded-full h-16"
							title="{$user.username}#{$user.discriminator}"
						/>
					</li>
				{:else}
					<li>
						<a href="/api/login">Login</a>
					</li>
				{/if}
			</ul>
		</nav>
	</div>
</header>

<style lang="postcss">
	ul {
		@apply flex flex-col md:flex-row md:items-center border-t md:border-0 pt-4 md:pt-0;
	}

	li a {
		@apply block px-4 py-1 md:p-2 lg:px-4;
	}

	ul {
		@apply list-none;
	}
</style>
