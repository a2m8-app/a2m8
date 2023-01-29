/** @jsxImportSource preact */
import type { FunctionalComponent } from "preact";
import { useState, useEffect } from "preact/hooks";

const repo = "a2m8-app/a2m8";

type ReleaseData = {
  name: string;
  tag_name: string;
  download_url: string;
};

export const Download = () => {
  let [os, setOs] = useState("");
  let [release, setRelease] = useState<ReleaseData>();
  useEffect(() => {
    if (navigator.userAgent.includes("Mac")) {
      setOs("mac");
    } else if (navigator.userAgent.includes("Win")) {
      setOs("windows");
    } else if (navigator.userAgent.includes("Linux")) {
      setOs("linux");
    }
    (async () => {
      let release = await fetch(
        `https://api.github.com/repos/${repo}/releases/latest`
      ).then((r) => r.json());
      let asset = release.assets.find((asset: any) => asset.name.includes(os));
      setRelease({
        tag_name: release.tag_name,
        name: asset.name,
        download_url: asset.browser_download_url,
      });
    })();
  }, []);
  return (
    <>
      <a
        class="btn btn-primary !no-underline"
        href={`${release?.download_url}`}
      >
        Download The app for {os}
      </a>
      <p class="text-sm">
        <a class="link link-hover" href={`https://github.com/${repo}/releases`}>
          Alternative downloads
        </a>
      </p>
    </>
  );
};
