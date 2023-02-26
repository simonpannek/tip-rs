<script>
	import '../app.css';

	import { user } from '$lib/auth.js';

	export let data;

	$: console.log(data.user);
</script>

<slot />
