<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData } from './$types';

	let { data }: { data: PageServerData } = $props();
</script>

<p>Logged in as: {data.user.name}</p>
<p>Current JWT: {data.token}</p>

<form method="post" action="?/signOut" use:enhance>
	<button class="btn preset-filled-primary-500">Sign out</button>
</form>
