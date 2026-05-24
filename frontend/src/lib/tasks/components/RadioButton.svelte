<script lang="ts">
	interface Props {
		label: string;
		name: string;
		value: string;
		checked?: boolean;
	}

	const { label, name, value, checked = false }: Props = $props();
</script>

<label class="flex items-center space-x-2">
	<input class="radio" type="radio" {checked} {name} {value} />
	<p>{label}</p>
</label>
