<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { invoke } from '@tauri-apps/api/tauri';
	import { boardName } from '$lib/stores/flash';
	//Rustからボード名を取得してIIFEする
	async function loadBoards(): Promise<string[]> {
		return await invoke('get_boards_for_select');
	}
	const handleSelectChange = (event: Event) => {
		const target = event.target as HTMLSelectElement;
		boardName.set(target.value);
	};
</script>

<select
	class="bg-white border border-gray-300 rounded-md shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
	on:change={handleSelectChange}
>
	<option disabled value="">{$_('flash.input.board.select')}</option>
	{#await loadBoards() then boards}
		{#each boards as item}
			<option value={item}>{item}</option>
		{/each}
	{:catch error}
		<p>error: {error.message}</p>
	{/await}
</select>
