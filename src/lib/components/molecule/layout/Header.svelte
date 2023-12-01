<script>
	import MahiwaTitle from '$lib/components/atom/text/MahiwaTitle.svelte';
	import { title } from '$lib/stores/header';
	let title_value = '';
	title.subscribe((value) => {
		title_value = value;
	});
</script>

<header class="flex items-center border-b-2 border-gray">
	<div class="w-1/3 pl-2">
		<a class="" href="/">
			<div class="flex justify-center items-center bg-black text-white w-8 h-8 rounded-full">
				<p class="text-xl">←</p>
			</div>
		</a>
	</div>
	<div class="w-1/3 text-2xl text-center">{title_value}</div>
	<div class="w-1/3 flex justify-end pr-4">
		<MahiwaTitle titleClass="text-xl md:text-xl" />
	</div>
</header>
