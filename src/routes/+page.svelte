<script>
	import { _ } from 'svelte-i18n';
	import MahiwaTitle from '$lib/components/atom/text/MahiwaTitle.svelte';
	import NormalButton from '$lib/components/atom/button/NormalButton.svelte';
</script>

<div class="w-full h-full flex justify-center items-center flex-col">
	<MahiwaTitle titleClass="text-2xl md:text-3xl" />
	<div class="pt-12 flex justify-center imtes-center flex-wrap">
		<NormalButton className="p-4" title={$_('flash.title')} url="/flash" />
		<NormalButton className="p-4" title={$_('generator.title')} url="/generator" />
		<NormalButton
			className="p-4"
			isInternal={false}
			title={$_('document.title')}
			url="https://mahiwa.usuyuki.net"
		/>
	</div>
</div>
<div class="hidden">
	<!-- Tailwind明示のためのダミー -->
	<div class="text-white bg-white"></div>
	<div class="text-black bg-black"></div>
	<div class="text-gray bg-gray"></div>
	<div class="text-yellow bg-yellow"></div>
	<div class="text-green bg-green"></div>
</div>
