<script>
	import { _ } from 'svelte-i18n';
</script>

<footer class="py-4 flex justify-center items-center flex-end">
	<div class="w-1/3"></div>
	<div class="w-1/3">
		<p class="text-xs text-center">
			<a target="_blank" rel="norefferrer" href="https://github.com/project-mahiwa">
				copyright Project Mahiwa
			</a>
		</p>
	</div>
	<div class="w-1/3">
		<p class="text-xs text-right mr-4">
			<a href="/license"> {$_('license.title')} </a>
		</p>
	</div>
</footer>
