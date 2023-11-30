<script>
	import Greet from '../lib/Greet.svelte';
	import { _ } from 'svelte-i18n';
</script>

<h1 class="text-3xl">{$_('top.top')}</h1>
<Greet />
