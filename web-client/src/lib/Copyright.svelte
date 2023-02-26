<script>
	import Copyright from 'svelte-remixicon/lib/icons/CopyrightLine.svelte';

	const year = 2023;

	const currentYear = new Date().getFullYear();
</script>

<Copyright class="inline-flex mb-1" aria-label="Copyright" />
{#if year < currentYear}{year} - {/if}{currentYear}. All rights reserved.
