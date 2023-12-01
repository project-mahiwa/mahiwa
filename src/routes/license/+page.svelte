<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { title } from '$lib/stores/header';
	import { type licenseT, licenses } from '$lib/types/license';
	let thisTitle = $_('license.title');
	title.set(thisTitle);
	import SubTitle from '$lib/components/atom/text/SubTitle.svelte';
	import LicenseCard from '$lib/components/atom/card/LicenseCard.svelte';

	const fontInfos: licenseT[] = [
		{
			name: 'Kiwi Maru',
			license: licenses['ofl11'],
			licenseUrl: 'https://github.com/Kiwi-KawagotoKajiru/Kiwi-Maru/blob/master/OFL.txt',
			credit:
				'Copyright 2020 The Kiwi Maru Project Authors (https://github.com/Kiwi-KawagotoKajiru/Kiwi-Maru)',
			url: 'https://github.com/Kiwi-KawagotoKajiru/Kiwi-Maru'
		},
		{
			name: 'DotGothic16',
			license: licenses['ofl11'],
			licenseUrl: 'https://github.com/fontworks-fonts/DotGothic16/blob/master/OFL.txt',
			credit:
				'Copyright 2020 The DotGothic16 Project Authors (https://github.com/fontworks-fonts/DotGothic16)',
			url: 'https://github.com/fontworks-fonts/DotGothic16'
		}
	];
</script>

<svelte:head>
	<title>{thisTitle}</title>
</svelte:head>
<SubTitle title="Font Licenses" />
<div class="flex items-center flex-col mt-4 mb-12">
	{#each fontInfos as fontInfo, index}
		<LicenseCard licenseInfo={fontInfo} number={index} />
	{/each}
</div>
<SubTitle title="Software Licenses" />
