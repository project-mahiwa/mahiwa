<script>
	import Greet from '$lib/native/Greet.svelte';
	import { _ } from 'svelte-i18n';
	import MahiwaTitle from '$lib/components/attom/text/MahiwaTitle.svelte';
	import NormalButton from '$lib/components/attom/button/NormalButton.svelte';
</script>

<div class="pt-20 flex justify-center items-center flex-col">
	<MahiwaTitle />
	<div class="pt-12 flex justify-center imtes-center flex-wrap">
		<NormalButton className="p-4" title={$_('top.button.flash')} url="/flash" />
		<NormalButton className="p-4" title={$_('top.button.generate')} url="/generate" />
		<NormalButton
			className="p-4"
			isInternal={false}
			title={$_('top.button.document')}
			url="/document"
		/>
	</div>

	<Greet />
</div>
