<script lang="ts">
	import { enhance } from '$app/forms';
	import { parseDate } from '@skeletonlabs/skeleton-svelte';
	import DatePicker from './DatePicker.svelte';
	import Input from './Input.svelte';
	import RadioButton from './RadioButton.svelte';
	import Textarea from './Textarea.svelte';
	import type { TaskError } from '$lib/tasks/types';
	import Dialog from './Dialog.svelte';

	type Props = {
		error: TaskError | undefined;
	};

	let { error }: Props = $props();
	let open = $state(false);
</script>

<Dialog bind:open title="Neue Aufgabe" trigger="Neue Aufgabe">
	{#snippet content()}
		<form
			method="POST"
			action="?/create"
			id="task-add-form"
			use:enhance={({ formElement }) => {
				return async ({ result, update }) => {
					await update();
					console.log('Form completed', result.type);
					if (result.type === 'success' || result.type === 'redirect') {
						formElement.reset();
						open = false;
					}
				};
			}}
		>
			<fieldset class="space-y-4">
				<Input label="Name" type="text" name="name" required />
				<Textarea
					label="Beschreibung"
					rows={4}
					name="description"
					placeholder="Optionale Beschreibung"
				/>
			</fieldset>

			<fieldset class="space-y-2">
				<p>Priorität</p>
				<RadioButton label="Sehr hoch" name="priority" value="very_high" checked />
				<RadioButton label="Hoch" name="priority" value="high" />
				<RadioButton label="Mittel" name="priority" value="medium" />
				<RadioButton label="Niedrig" name="priority" value="low" />
			</fieldset>

			<!-- TODO: Dynamic locale -->
			<DatePicker required label="Start" name="start" value={[parseDate(new Date())]} />
			<DatePicker label="Deadline" name="deadline" />

			<div>
				{#if error}
					{#if error.type === 'zod_error'}
						{#each error.issues.map((i) => i.message) as e (e)}
							<p class="text-error-500">{e}</p>
						{/each}
					{:else if error.type === 'http_error'}
						<p class="text-error-500">{error.status}: {error.message}</p>
					{/if}
				{/if}
			</div>
		</form>
	{/snippet}

	{#snippet footer()}
		<button type="submit" form="task-add-form" class="btn preset-filled-primary-500">
			Speichern
		</button>
	{/snippet}
</Dialog>
