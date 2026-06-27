<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Navigation>
	<Navigation.Header />
	<Navigation.Content>
		<Navigation.Menu>
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
			{/if}
		</Navigation.Menu>
	</Navigation.Content>
	<Navigation.Footer />
</Navigation>

{@render children()}
