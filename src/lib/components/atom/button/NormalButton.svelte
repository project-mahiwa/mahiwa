<script lang="ts">
	export let bgColorVariable = 'gray';
	export let textColorVariable = 'white';
	export let title = 'ボタン名';
	export let url = '/link';
	export let className = '';
	export let isInternal = true;
</script>

<!-- bg-変数名 のように指定すると、Tailwindのバンドル時に対象外になってしまう危険があるが、他で絶対使うのでこの指定を許している -->
<div class={`flex items-center justify-center ${className}`}>
	{#if isInternal}
		<a href={url}>
			<div
				class="flex rounded-xl bg-{bgColorVariable} px-4 py-3 hover:bg-{textColorVariable} hover:text-{bgColorVariable} text-{textColorVariable} duration-500"
			>
				<p>{title}</p>
			</div>
		</a>
	{:else}
		<a target="_blank" rel="norefferrer" href={url}>
			<div
				class="flex rounded-xl bg-{bgColorVariable} px-4 py-3 hover:bg-{textColorVariable} hover:text-{bgColorVariable} text-{textColorVariable} duration-500"
			>
				<p>🔗{title}</p>
			</div>
		</a>
	{/if}
</div>
