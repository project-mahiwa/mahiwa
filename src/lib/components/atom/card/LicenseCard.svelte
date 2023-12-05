<script lang="ts">
	import type { licenseT } from '$lib/types/license';
	export let licenseInfo: licenseT;
	export let number: number;
</script>

<!-- marginだとflexの計算壊れるのでdiv要素を追加する -->
<div class="p-2">
	<div class="border-2 border-gray p-2">
		<h3 class="text-xl">{number + 1}.{licenseInfo['name']}</h3>
		<p>
			<a target="_blank" href={licenseInfo['licenseUrl']}>
				{licenseInfo['license']}
			</a>
		</p>
		<p>
			<a target="_blank" href={licenseInfo['url']}>
				{licenseInfo.credit}
			</a>
		</p>
	</div>
</div>
