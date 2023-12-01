<script>
	import MahiwaTitle from '$lib/components/attom/text/MahiwaTitle.svelte';
	import { title } from '$lib/stores/header';
	let title_value = '';
	title.subscribe((value) => {
		title_value = value;
	});
</script>

<header class="flex w-full h-20 border-2 border-b border-gray">
	<div class="w-1/3"><a href="/">戻るボタン</a></div>
	<div class="w-1/3">{title_value}</div>
	<div class="w-1/3"><MahiwaTitle /></div>
</header>
