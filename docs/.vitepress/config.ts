import { defineConfig } from 'vitepress'

const title = 'Locus'
const description = 'The exact code chunk for the job — local Rust TF-IDF search for agents.'
const url = 'https://sylphxai.github.io/locus'
const ogImage = `${url}/og-image.png`

export default defineConfig({
	title,
	description,
	base: '/locus/',
	cleanUrls: true,
	ignoreDeadLinks: true,
	lastUpdated: true,
	appearance: 'dark',

	head: [
		// Favicon
		['link', { rel: 'icon', type: 'image/svg+xml', href: '/locus/logo.svg' }],
		['link', { rel: 'icon', type: 'image/png', href: '/locus/favicon.png' }],

		// SEO
		['meta', { name: 'theme-color', content: '#c4b5fd' }],
		['meta', { name: 'author', content: 'Sylphx' }],
		[
			'meta',
			{
				name: 'keywords',
				content: 'Locus, code search, TF-IDF, BM25, MCP, local search, AI assistant',
			},
		],

		// Open Graph
		['meta', { property: 'og:type', content: 'website' }],
		['meta', { property: 'og:title', content: title }],
		['meta', { property: 'og:description', content: description }],
		['meta', { property: 'og:url', content: url }],
		['meta', { property: 'og:image', content: ogImage }],
		['meta', { property: 'og:image:width', content: '1200' }],
		['meta', { property: 'og:image:height', content: '630' }],
		['meta', { property: 'og:site_name', content: 'Locus' }],
		['meta', { property: 'og:locale', content: 'en_US' }],

		// Twitter Card
		['meta', { name: 'twitter:card', content: 'summary_large_image' }],
		['meta', { name: 'twitter:title', content: title }],
		['meta', { name: 'twitter:description', content: description }],
		['meta', { name: 'twitter:image', content: ogImage }],
		['meta', { name: 'twitter:site', content: '@SylphxAI' }],

		// Canonical
		['link', { rel: 'canonical', href: url }],
	],

	sitemap: {
		hostname: `${url}/`,
	},

	themeConfig: {
		logo: '/logo.svg',
		siteTitle: 'Locus',

		nav: [
			{ text: 'Guide', link: '/guide/getting-started' },
			{ text: 'MCP Server', link: '/mcp/overview' },
			{
				text: 'Resources',
				items: [
					{
						text: 'Stop code-search guessing',
						link: '/articles/stop-code-search-guessing',
					},
					{ text: 'GitHub', link: 'https://github.com/SylphxAI/locus' },
					{ text: 'npm', link: 'https://www.npmjs.com/package/@sylphx/locus' },
					{ text: 'Changelog', link: 'https://github.com/SylphxAI/locus/releases' },
				],
			},
		],

		sidebar: {
			'/': [
				{
					text: 'Product',
					items: [
						{ text: 'Vision', link: '/vision' },
						{ text: 'Capabilities', link: '/capabilities' },
					],
				},
			],
			'/guide/': [
				{
					text: 'Introduction',
					items: [
						{ text: 'What is Locus?', link: '/guide/getting-started' },
						{ text: 'Installation', link: '/guide/installation' },
						{ text: 'Quickstart', link: '/guide/quickstart' },
					],
				},
				{
					text: 'Core Concepts',
					items: [
						{ text: 'How Search Works', link: '/guide/how-search-works' },
						{ text: 'Symbol chunks', link: '/guide/ast-chunking' },
						{ text: 'TF-IDF & BM25', link: '/guide/tfidf' },
						{ text: 'Language Support', link: '/guide/languages' },
					],
				},
			],
			'/mcp/': [
				{
					text: 'MCP Server',
					items: [
						{ text: 'Overview', link: '/mcp/overview' },
						{ text: 'Installation', link: '/mcp/installation' },
						{ text: 'Configuration', link: '/mcp/configuration' },
						{ text: 'Tools Reference', link: '/mcp/tools' },
						{ text: 'IDE Integration', link: '/mcp/ide-integration' },
					],
				},
			],
		},

		socialLinks: [
			{ icon: 'github', link: 'https://github.com/SylphxAI/locus' },
			{ icon: 'npm', link: 'https://www.npmjs.com/package/@sylphx/locus' },
		],

		footer: {
			message: 'Released under the MIT License.',
			copyright: 'Copyright © 2026 Sylphx',
		},

		search: {
			provider: 'local',
			options: {
				detailedView: true,
			},
		},

		editLink: {
			pattern: 'https://github.com/SylphxAI/locus/edit/main/docs/:path',
			text: 'Edit this page on GitHub',
		},

		outline: {
			level: [2, 3],
		},

		lastUpdated: {
			text: 'Last updated',
			formatOptions: {
				dateStyle: 'medium',
			},
		},
	},

	markdown: {
		theme: {
			light: 'github-light',
			dark: 'github-dark',
		},
		lineNumbers: true,
	},
})
