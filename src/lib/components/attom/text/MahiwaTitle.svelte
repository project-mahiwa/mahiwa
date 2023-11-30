<script>
	import { _ } from 'svelte-i18n';
	import logo from '$lib/assets/logo/mahiwa-logo-v1.png';
</script>

<div class="flex justify-center items-center">
	<h1 class="text-3xl text-gray text-bolyellowd">{$_('top.title')}</h1>
	<img src={logo} alt="logo" class="w-12 h-12" />
</div>
