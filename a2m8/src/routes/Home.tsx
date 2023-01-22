import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "preact/hooks";
import ScriptComponent from "../components/ScriptComponent";
import UploadScripts from "../components/UploadScripts";
import { useStore } from "@nanostores/react";
import { scripts, setScripts } from "../lib/scriptStore";
import { Script } from "../lib/script";
import { Transition } from "@headlessui/react";
import { Fragment } from "preact";

export default function Home({ path }: { path: string }) {
  const list = useStore(scripts);

  useEffect(() => {
    invoke<Script[]>("get_scripts").then((data) => {
      setScripts(data);
    });
  }, []);
  return (
    <div>
      <h1>Home</h1>
      <p>This is the Home component.</p>

      <div className={"max-w-[75rem] mx-auto grid grid-cols-1 gap-2 px-2"}>
        {list?.map((script) => (
          <ScriptComponent key={script.id} script={script} />
        ))}
      </div>
      <UploadScripts />
    </div>
  );
}
