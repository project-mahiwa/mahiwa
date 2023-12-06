<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { open } from '@tauri-apps/api/dialog';

	import { wasmFilePath } from '$lib/stores/flash';

	wasmFilePath.set('/home/usuyuki/ダウンロード/helloworld.wasm');
	async function openDialog() {
		await open({
			multiple: true,
			filters: [
				{
					name: 'wasm',
					extensions: ['wasm']
				}
			]
		}).then((res) => wasmFilePath.set(res === null ? '' : res.toString())); //ファイル選択外すとnullになるのを防ぐ
	}
</script>

<button
	type="button"
	class="p-2 border-gray border-4 rounded-lg border-dotted"
	on:click={openDialog}
	>{$wasmFilePath === ''
		? $_('flash.input.wasmFileInput.select')
		: $_('flash.input.wasmFileInput.reselect')}</button
>
