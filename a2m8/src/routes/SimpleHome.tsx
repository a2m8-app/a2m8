import { useStore } from "@nanostores/react";
import { invoke } from "@tauri-apps/api";
import { Fragment } from "preact";
import { useEffect } from "preact/hooks";
import SimpleScript from "../components/SimpleScript";
import { Script, statusToText } from "../lib/script";
import { scripts, setScripts } from "../lib/scriptStore";
export default function BasicHome({ path }: { path: string }) {
  const list = useStore(scripts);

  function loadScripts() {
    invoke<Script[]>("get_scripts").then((data) => {
      setScripts(data);
    });
  }

  useEffect(loadScripts, []);

  return (
    <>
      <button class="btn rounded-none px-1">Reload Scripts</button>
      <table class="table table-compact w-full">
        <thead>
          <tr>
            <th>Name</th>
            <th>Favorite</th>
            <th>Startup</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {list
            ?.filter((x) => !x.draft)
            .map((script) => (
              <SimpleScript script={script}></SimpleScript>
            ))}
        </tbody>
      </table>
    </>
  );
}
