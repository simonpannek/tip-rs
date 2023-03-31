import { derived } from 'svelte/store';
import { page } from '$app/stores';

import { guilds } from './auth';

// Current guild
export const guild = derived([page, guilds], ([$page, $guilds]) => {
	if (!isNaN($page.params.guild)) {
		return $guilds.get(BigInt($page.params.guild));
	}
});
