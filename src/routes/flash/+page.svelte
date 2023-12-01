<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { title } from '$lib/stores/header';
	import SubTitle from '$lib/components/atom/text/SubTitle.svelte';
	let thisTitle = $_('flash.title');
	title.set(thisTitle);
</script>

<svelte:head>
	<title>{thisTitle}</title>
</svelte:head>
<SubTitle title="1. ほげほげ" />
