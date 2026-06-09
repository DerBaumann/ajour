<script lang="ts">
	import Input from '$lib/components/Input.svelte';
	import type { ActionData } from './$types';

	const { form }: { form: ActionData } = $props();
</script>

<h1 class="h1">Login</h1>

<form class="w-full max-w-md space-y-4 p-4" method="POST">
	<Input type="email" label="Email" name="email" placeholder="you@example.com" required />
	<Input type="password" label="Passwort" name="password" required />

	<button type="submit" class="btn preset-filled-primary-500">Login</button>
</form>

{#if form?.message}
	<p class="text-error-500">Error: {form?.message}</p>
{/if}
