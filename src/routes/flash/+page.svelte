<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { title } from '$lib/stores/header';
	import SubTitle from '$lib/components/atom/text/SubTitle.svelte';
	import FileInput from '$lib/components/atom/input/FileInput.svelte';
	import Submit from '$lib/components/atom/input/Submit.svelte';
	import Select from '$lib/components/atom/input/Select.svelte';

	let thisTitle = $_('flash.title');
	const boards = ['M5 Stack Core2', 'Raspberry Pi Pico W', 'Seeed Studio XIAO ESP32C3'];
	title.set(thisTitle);
</script>

<svelte:head>
	<title>{thisTitle}</title>
</svelte:head>
<form method="POST" class="flex flex-col items-center justify-center">
	<SubTitle title={'1. ' + $_('flash.input.board.title')} />
	<Select name="board" items={boards} />
	<SubTitle title={'2. ' + $_('flash.input.wasmFileInput.title')} />
	<FileInput name="wasm" inputMessage={$_('flash.input.wasmFileInput.select')} />
	<!-- <SubTitle title={'3. ' + $_('flash.input.addLibraries.title')} /> -->
	<Submit title={$_('flash.input.submit.title')} />
</form>
