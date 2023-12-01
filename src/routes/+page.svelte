<script>
	import Greet from '$lib/native/Greet.svelte';
	import { _ } from 'svelte-i18n';
	import MahiwaTitle from '$lib/components/attom/text/MahiwaTitle.svelte';
	import NormalButton from '$lib/components/attom/button/NormalButton.svelte';
</script>

<div class="pt-20 flex justify-center items-center flex-col">
	<MahiwaTitle />
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

	<Greet />
</div>
<div class="hidden">
	<!-- Tailwind明示のためのダミー -->
	<div class="text-white bg-white"></div>
	<div class="text-black bg-black"></div>
	<div class="text-gray bg-gray"></div>
	<div class="text-yellow bg-yellow"></div>
	<div class="text-green bg-green"></div>
</div>
