<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { title } from '$lib/stores/header';
	let thisTitle = $_('generator.title');
	title.set(thisTitle);
</script>

<svelte:head>
	<title>{thisTitle}</title>
</svelte:head>
generator
