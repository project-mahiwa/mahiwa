<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { title } from '$lib/stores/header';
	import SubTitle from '$lib/components/atom/text/SubTitle.svelte';
	import WasmInput from '$lib/components/molecule/flash/WasmInput.svelte';
	import SelectBoard from '$lib/components/molecule/flash/SelectBoard.svelte';
	import FlashButton from '$lib/components/molecule/flash/FlashButton.svelte';
	import { boardName, wasmFilePath } from '$lib/stores/flash';

	let thisTitle = $_('flash.title');
	title.set(thisTitle);
</script>

<svelte:head>
	<title>{thisTitle}</title>
</svelte:head>
<form method="POST" class="flex flex-col items-center justify-center">
	<SubTitle title={'1. ' + $_('flash.input.board.title')} />
	<SelectBoard />
	<SubTitle title={'2. ' + $_('flash.input.wasmFileInput.title')} />
	<WasmInput />
	<!-- <SubTitle title={'3. ' + $_('flash.input.addLibraries.title')} /> -->
	<p class="text-center mt-12 text-gray">
		Board:{$boardName === '' ? $_('flash.notSelected') : $boardName}
	</p>
	<p class="text-center mt-2 text-gray">
		Wasm:{$wasmFilePath === '' ? $_('flash.notSelected') : $wasmFilePath}
	</p>
	<FlashButton />
</form>
