<!-- TODO: Cleanup -->
<script lang="ts">
	import { resolve } from '$app/paths';
	import DatePicker from '$lib/tasks/components/DatePicker.svelte';
	import Input from '$lib/tasks/components/Input.svelte';
	import RadioButton from '$lib/tasks/components/RadioButton.svelte';
	import Textarea from '$lib/tasks/components/Textarea.svelte';
	import { parseDate } from '@skeletonlabs/skeleton-svelte';
</script>

<h1 class="h1">Neue Aufgabe</h1>

<p>
	<a href={resolve('/tasks')} class="btn preset-outlined-secondary-500">Zurück </a>
</p>

<form class="w-full max-w-md space-y-4 p-4" method="POST">
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
	<DatePicker label="Start" value={[parseDate(new Date())]} />
	<DatePicker label="Deadline" />

	<fieldset class="flex justify-end">
		<button type="submit" class="btn preset-filled-primary-500">Speichern</button>
	</fieldset>
</form>
