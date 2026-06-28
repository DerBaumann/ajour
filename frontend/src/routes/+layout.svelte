<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import type { LayoutProps } from './$types';
	import { authClient } from '$lib/auth-client';
	import { goto, invalidateAll } from '$app/navigation';
	import { resolve } from '$app/paths';

	let { children, data }: LayoutProps = $props();

	async function logout() {
		await authClient.signOut({
			fetchOptions: {
				onSuccess: async () => {
					await invalidateAll();
					goto(resolve('/auth/signin'));
				}
			}
		});
	}
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Navigation>
	<Navigation.Header />
	<Navigation.Content>
		<Navigation.Menu>
			<p class="mr-16 h4">Ajour</p>
			{#if !data.user}
				<Navigation.TriggerAnchor href="/auth/signin" class="btn preset-filled">
					<Navigation.TriggerText class="text-lg">Login</Navigation.TriggerText>
				</Navigation.TriggerAnchor>
				<Navigation.TriggerAnchor href="/auth/signup" class="btn preset-filled">
					<Navigation.TriggerText class="text-lg">Registrieren</Navigation.TriggerText>
				</Navigation.TriggerAnchor>
			{:else}
				<Navigation.TriggerAnchor href="/tasks" class="btn preset-filled">
					<Navigation.TriggerText class="text-lg">Heutige Aufgaben</Navigation.TriggerText>
				</Navigation.TriggerAnchor>
				<Navigation.Trigger onclick={logout} class="btn preset-filled-primary-500">
					Logout
				</Navigation.Trigger>
			{/if}
		</Navigation.Menu>
	</Navigation.Content>
	<Navigation.Footer />
</Navigation>

{@render children()}
