import { defineConfig } from "astro/config";

import tailwind from "@astrojs/tailwind";
import mdx from "@astrojs/mdx";
import cloudflare from "@astrojs/cloudflare";
import preact from "@astrojs/preact";
import react from "@astrojs/react";

import { replaceCodePlugin } from "vite-plugin-replace";

// https://astro.build/config
export default defineConfig({
  integrations: [
    tailwind(),
    mdx(),
    preact(),
    react(),
  ],
  site: `https://astro.build`,
  vite: {
    plugins: [replaceCodePlugin({
      replacements: [
        {
          from: "__GITEA__",
          to: "https://a2m8-git.tricked.dev",
        },
      ],
    })],
  },
  // output: "server",
  // adapter: cloudflare()
});
