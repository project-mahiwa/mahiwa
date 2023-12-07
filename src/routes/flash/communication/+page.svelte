<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { _ } from 'svelte-i18n';
	import { onMount, afterUpdate } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { portName } from '$lib/stores/flash';
	import SubTitle from '$lib/components/atom/text/SubTitle.svelte';
	let backendLog = '';
	let logContainer: HTMLDivElement;
	listen('btf-flash-communication', (event) => {
		// シリアル通信の改行を尊重したいので、こっちで付けないようにする
		backendLog += `${event.payload}`;
	});
	// unlisten();

	onMount(() => {
		logContainer.scrollTop = logContainer.scrollHeight;

		// Rustに渡す
		(async () => {
			await invoke('serial', {
				portName: $portName,
				baudRate: 115200
			})
				.then((res) => {
					return res;
				})
				.catch((err) => {
					console.log(err);
					return err;
				});
		})();
	});

	afterUpdate(() => {
		logContainer.scrollTop = logContainer.scrollHeight;
	});
</script>

<SubTitle title={$_('flash.communication.title')} />
<div bind:this={logContainer} class="overflow-y-scroll break-all h-96 text-white bg-black p-2">
	<pre class="">{backendLog}</pre>
</div>
