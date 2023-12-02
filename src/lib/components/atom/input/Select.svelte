<script lang="ts">
	export let items: string[];
	export let name: string;
</script>

<select
	class="bg-white border border-gray-300 rounded-md shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
	{name}
>
	<option disabled value="">選択してください</option>
	{#each items as item}
		<option value={item}>{item}</option>
	{/each}
</select>
