<script lang="ts">
	interface Props {
		label: string;
		rows: number;
		name: string;
		required?: boolean;
		placeholder?: string;
	}

	const { label, rows, name, required = false, placeholder }: Props = $props();
</script>

<label class="label">
	<span class="label-text">{label}</span>
	<textarea class="textarea rounded-container" {rows} {name} {placeholder} {required}></textarea>
</label>
