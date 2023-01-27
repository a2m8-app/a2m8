import { defineConfig } from "astro/config";

import tailwind from "@astrojs/tailwind";
import mdx from "@astrojs/mdx";
import cloudflare from "@astrojs/cloudflare";
import preact from "@astrojs/preact";
import react from "@astrojs/react";
import image from "@astrojs/image";

import { replaceCodePlugin } from "vite-plugin-replace";

import { exec } from "child_process";
import { promisify } from "util";
import { cp, mkdir } from "fs/promises";

let asyncExec = promisify(
  exec,
);

// https://astro.build/config
export default defineConfig({
  integrations: [
    tailwind(),
    mdx(),
    preact(),
    react(),
    image({
      serviceEntryPoint: "@astrojs/image/sharp",
    }),
  ],
  site: `https://a2m8.tricked.dev`,
  vite: {
    plugins: [
      replaceCodePlugin({
        replacements: [
          {
            from: "__GITEA__",
            to: "https://a2m8-git.tricked.dev",
          },
        ],
      }),
      {
        name: "build cargo docs",
        buildEnd: async () => {
          await asyncExec("cargo doc --no-deps --lib --package a2m8-lib", {});
          let path = new URL("../target/doc/", import.meta.url);
          let outPath = new URL("dist/lib/", import.meta.url);
          await mkdir(outPath, { recursive: true });
          await cp(path, outPath, { recursive: true });
        },
      },
    ],
  },
  // output: "server",
  // adapter: cloudflare()
});
