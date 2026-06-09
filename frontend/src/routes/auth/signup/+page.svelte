<!-- TODO: Implement Signup -->
<script lang="ts">
	import Input from '$lib/components/Input.svelte';
</script>

<h1 class="h1">Registrierung</h1>

<form class="w-full max-w-md space-y-4 p-4" method="POST">
	<Input type="text" label="Nutzername" name="name" required />
	<Input type="email" label="Email" name="email" placeholder="you@example.com" required />
	<Input type="password" label="Passwort" name="password" required />
	<Input type="password" label="Passwort wiederholen" name="password-repeat" required />

	<button type="submit" class="btn preset-filled-primary-500">Registrieren</button>
</form>
