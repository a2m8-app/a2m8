import { defineConfig } from 'astro/config';

import tailwind from "@astrojs/tailwind";
import mdx from "@astrojs/mdx";
import cloudflare from "@astrojs/cloudflare";
import preact from '@astrojs/preact';
import react from '@astrojs/react';

// https://astro.build/config
export default defineConfig({
  integrations: [
    tailwind(), 
    mdx(),		
    preact(),
    react(),
  ],
	site: `https://astro.build`,
  output: "server",
  adapter: cloudflare()
});