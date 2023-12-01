<script lang="ts">
	import type { licenseT } from '$lib/types/license';
	export let licenseInfo: licenseT;
	export let number: number;
</script>

<div class="border-2 border-gray">
	<h3 class="text-xl">{number + 1}.{licenseInfo['name']}</h3>
	<a href={licenseInfo['licenseUrl']}>
		{licenseInfo['license']}
	</a>
	<a href={licenseInfo['url']}>
		{licenseInfo.credit}
	</a>
</div>
