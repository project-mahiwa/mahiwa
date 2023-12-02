<script>
	import './styles.css';
	import '../app.css';
	import { page } from '$app/stores';
	import Header from '$lib/components/molecule/layout/Header.svelte';
	import Footer from '$lib/components/molecule/layout/Footer.svelte';
</script>

{#if $page.url.pathname !== '/'}
	<Header />
{/if}
<div class="w-screen">
	<main>
		<slot />
	</main>
</div>
{#if $page.url.pathname === '/'}
	<Footer />
{/if}
