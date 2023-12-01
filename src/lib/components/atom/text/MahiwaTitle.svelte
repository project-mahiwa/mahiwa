<script lang="ts">
	import { _ } from 'svelte-i18n';
	import logo from '$lib/assets/logo/mahiwa-logo-v1.png';
	export let titleClass: string;
</script>

<div class="flex justify-center items-center">
	<h1 class={`text-gray text-bolyellowd font-kiwi-maru ${titleClass}`}>{$_('top.title')}</h1>
	<img src={logo} alt="logo" class="w-12 h-12" />
</div>
