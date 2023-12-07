<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { invoke } from '@tauri-apps/api/tauri';
	import { portName } from '$lib/stores/flash';
	async function getPorts(): Promise<string[]> {
		return await invoke('list_serial_ports');
	}

	const handleSelectChange = (event: Event) => {
		const target = event.target as HTMLSelectElement;
		portName.set(target.value);
	};
</script>

<select
	class="bg-white border border-gray-300 rounded-md shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
	on:change={handleSelectChange}
>
	<option disabled value="">{$_('flash.input.board.select')}</option>
	<!-- 予め取得しとくけど、後で接続しても対応できるようにボタンからでも発火できるようにする -->
	{#await getPorts() then ports}
		{#each ports as item}
			<option value={item}>{item}</option>
		{/each}
	{:catch error}
		<p>error: {error.message}</p>
	{/await}
</select>
