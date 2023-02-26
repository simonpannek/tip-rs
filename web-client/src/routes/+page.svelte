<script>
	import { user } from '$lib/auth.js';
</script>

<h1 class="text-3xl text-accent1">Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if $user}
    Welcome {$user.username}#{$user.discriminator}!
{:else}
    Who are you?
{/if}
