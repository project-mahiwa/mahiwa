<script lang="ts">
	export let inputMessage: string;
	export let name: string;
</script>

<div class="flex items-center justify-center w-full">
	<label
		for="dropzone-file"
		class="flex flex-col items-center justify-center w-2/3 h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
	>
		<div class="flex flex-col items-center justify-center pt-5 pb-6">
			<p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
				{inputMessage}
			</p>
		</div>
		<input id="dropzone-file" type="file" {name} class="hidden" />
	</label>
</div>
