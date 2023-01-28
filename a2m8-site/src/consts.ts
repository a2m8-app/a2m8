export const SITE = {
  title: "A2M8 website!",
  description: "A2M8 landing page wiki and workshop!",
  defaultLanguage: "en-us",
} as const;

export const OPEN_GRAPH = {
  image: {
    //@someone please make some cool art!
    src: "",
    alt: "",
    // src: 'https://github.com/withastro/astro/blob/main/assets/social/banner-minimal.png?raw=true',
    // alt: "astro logo on a starry expanse of space," +
    //   " with a purple saturn-like planet floating in the right foreground",
  },
  twitter: "trickeddev",
};

export const GITHUB_EDIT_URL =
  `https://github.com/a2m8-app/a2m8/tree/master/a2m8-site`;

export const COMMUNITY_INVITE_URL = `/discord`;

export type Sidebar = Record<string, { text: string; link: string }[]>;
export const SIDEBAR: Sidebar = {
  "Main": [
    { text: "Introduction", link: "introduction" },
    { text: "Creating", link: "creating" },
    // { text: "Page 3", link: "page-3" },
  ],
  // 'Another Section': [{ text: 'Page 4', link: 'page-4' }],
};

export const GITEA = "https://a2m8-git.tricked.dev";
