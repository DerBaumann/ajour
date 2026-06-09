<script lang="ts">
	import type { HTMLInputTypeAttribute } from 'svelte/elements';

	interface Props {
		label: string;
		type: HTMLInputTypeAttribute;
		name: string;
		required: boolean;
		placeholder?: string;
	}

	const { label, type, name, required, placeholder }: Props = $props();
</script>

<label class="label">
	<span class="label-text">{label}</span>
	<input class="input" {type} {name} {required} {placeholder} />
</label>
