<script lang="ts">
	import { enhance } from '$app/forms';
	import DatePicker from '$lib/components/DatePicker.svelte';
	import Input from '$lib/components/Input.svelte';
	import RadioButton from '$lib/components/RadioButton.svelte';
	import TaskItem from '$lib/components/TaskItem.svelte';
	import Textarea from '$lib/components/Textarea.svelte';
	import { XIcon } from '@lucide/svelte';
	import { Dialog, parseDate, Portal } from '@skeletonlabs/skeleton-svelte';

	let { data } = $props();

	const addDialogAnimation = `transition transition-discrete opacity-0 translate-y-[100px]
      starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-[100px]
      data-[state=open]:opacity-100 data-[state=open]:translate-y-0`;
</script>

<main class="space-y-4">
	<h1 class="h1">Heutige Aufgaben</h1>

	<!-- TODO: Filter & Sorting -->
	<div id="toolbar">
		<!-- TODO: Move dialog to its own component -->
		<!-- TODO: close dialog only on successfull submit -->
		<Dialog>
			<Dialog.Trigger class="btn preset-filled-secondary-500">Neue Aufgabe</Dialog.Trigger>
			<Portal>
				<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
				<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
					<Dialog.Content
						class="w-full max-w-xl space-y-4 card bg-surface-100-900 p-4 shadow-xl {addDialogAnimation}"
					>
						<form
							class="w-full max-w-md space-y-4 p-4"
							method="POST"
							action="?/create"
							use:enhance={({ formElement, formData, action, cancel, submitter }) => {
								return async ({ result, update }) => {
									await update();
									console.log(result.type);

									if (result.type === 'success') {
										// close dialog
										// reset form
										formElement.reset();
									}
								};
							}}
						>
							<header class="flex items-center justify-between">
								<Dialog.Title class="text-lg font-bold">Neue Aufgabe</Dialog.Title>
								<Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
									<XIcon class="size-4" />
								</Dialog.CloseTrigger>
							</header>
							<Dialog.Description>
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
							</Dialog.Description>
							<footer class="flex justify-end gap-2">
								<Dialog.CloseTrigger class="btn preset-tonal">Cancel</Dialog.CloseTrigger>
								<Dialog.CloseTrigger type="submit" class="btn preset-filled-primary-500">
									Speichern
								</Dialog.CloseTrigger>
							</footer>
						</form>
					</Dialog.Content>
				</Dialog.Positioner>
			</Portal>
		</Dialog>
	</div>

	<div class="space-y-4">
		{#each data.tasks as task (task)}
			<TaskItem {task} />
		{/each}
	</div>
</main>
