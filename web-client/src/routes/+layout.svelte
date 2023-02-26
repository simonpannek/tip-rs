<script>
	import '../app.css';

	import { user } from '$lib/auth.js';

	export let data;

	$: user && user.set(data.user);
</script>

<slot />
