/** @jsxImportSource preact */
import type { FunctionalComponent } from "preact";
import { useState, useEffect } from "preact/hooks";

const repo = "a2m8-app/a2m8";

type ReleaseData = {
  name: string;
  tag_name: string;
  download_url: string;
};

const mappers = {
  mac: "dmg",
  windows: "msi",
  linux: "AppImage",
}

export const Download = () => {
  let [os, setOs] = useState("");
  let [release, setRelease] = useState<ReleaseData>();
  useEffect(() => {
    let os = ""
    if (navigator.userAgent.includes("Mac")) {
      os = "mac"
    } else if (navigator.userAgent.includes("Win")) {
      os = "windows";
    } else if (navigator.userAgent.includes("Linux")) {
      os = "linux";
    }
    setOs(os);

    (async () => {
      let release = await fetch(
        `https://api.github.com/repos/${repo}/releases/latest`
      ).then((r) => r.json());
      if (release.documentation_url) { return }
      let asset = release.assets.find((asset: any) => asset.name.includes(mappers[os as keyof typeof mappers]));

      setRelease({
        tag_name: release.tag_name,
        name: asset.name,
        download_url: asset.browser_download_url,
      });
    })();
  }, []);
  return (
    <>
      <div class="flex flex-wrap gap-2">
        {os && release && <a
          class="btn btn-primary !no-underline rounded-none"
          href={`${release?.download_url}`}
        >
          Download The app for {os}
        </a>}
        {os && !release && <a
          class="btn btn-primary !no-underline rounded-none"
          href={`https://github.com/${repo}/releases`}
        >
          Visit the github releases page
        </a>
        }
        <a
          class="btn btn-secondary btn-outline !no-underline rounded-none"
          href={`/docs/introduction`}
        >
          Read the docs
        </a>
      </div>
      <p class="text-sm">
        <a class="link link-hover" href={`https://github.com/${repo}/releases`}>
          Alternative downloads
        </a>
      </p>
    </>
  );
};
