<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { boardName, wasmFilePath } from '$lib/stores/flash';
	import { invoke } from '@tauri-apps/api/tauri';
	let result = '';
	async function runFlash() {
		console.log('flash');
		// バリデーション
		if ($boardName === '') {
			alert($_('flash.input.board.notSelected'));
			return;
		}
		if ($wasmFilePath === '') {
			alert($_('flash.input.wasmFileInput.notSelected'));
			return;
		}
		// Rustに渡す
		result = await invoke('flash_to_mcu', {
			boardName: $boardName,
			wasmFilePath: $wasmFilePath
		})
			.then((res) => {
				return String(res);
			})
			.catch((err) => {
				console.log(err);
				return String(err);
			});
	}
</script>

<button type="button" on:click={runFlash} class="bg-gray text-white font-bold p-4 m-2 mt-6"
	>{$_('flash.input.submit.title')}</button
>
{result}
