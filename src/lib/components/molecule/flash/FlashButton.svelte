<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { boardName, wasmFilePath } from '$lib/stores/flash';
	import { goto } from '$app/navigation';
	let result = '';
	let buttonDisabled = false;
	async function runFlash() {
		buttonDisabled = true;
		console.log('flash');
		// バリデーション
		if ($boardName === '') {
			alert($_('flash.input.board.notSelected'));
			buttonDisabled = false;
			return;
		}
		if ($wasmFilePath === '') {
			alert($_('flash.input.wasmFileInput.notSelected'));
			buttonDisabled = false;
			return;
		}
		goto('/flash/progress');
	}
</script>

<button
	type="button"
	disabled={buttonDisabled}
	on:click={runFlash}
	class="bg-gray text-white font-bold p-4 m-2 mt-6 disabled:bg-black"
	>{$_('flash.input.submit.title')}</button
>
{result}
