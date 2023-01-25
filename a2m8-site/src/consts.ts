export const SITE = {
	title: 'Documentation',
	description: 'Your website description.',
	defaultLanguage: 'en-us',
} as const;

export const OPEN_GRAPH = {
	image: {
		src: 'https://github.com/withastro/astro/blob/main/assets/social/banner-minimal.png?raw=true',
		alt:
			'astro logo on a starry expanse of space,' +
			' with a purple saturn-like planet floating in the right foreground',
	},
	twitter: 'astrodotbuild',
};



export const GITHUB_EDIT_URL = `https://github.com/withastro/astro/tree/main/examples/docs`;

export const COMMUNITY_INVITE_URL = `https://astro.build/chat`;

// See "Algolia" section of the README for more information.
export const ALGOLIA = {
	indexName: 'XXXXXXXXXX',
	appId: 'XXXXXXXXXX',
	apiKey: 'XXXXXXXXXX',
};

export type Sidebar =
	Record<string, { text: string; link: string }[]>;
export const SIDEBAR: Sidebar = {
	'Section Header': [
		{ text: 'Introduction', link: 'introduction' },
		{ text: 'Page 2', link: 'page-2' },
		{ text: 'Page 3', link: 'page-3' },
	],
	'Another Section': [{ text: 'Page 4', link: 'page-4' }],
};
