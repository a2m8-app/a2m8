import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "preact/hooks";
import ScriptComponent from "../components/ScriptComponent";
import UploadScripts from "../components/UploadScripts";
import { useStore } from "@nanostores/react";
import { scripts, setScripts, updateScript } from "../lib/scriptStore";
import { Script } from "../lib/script";
import { Transition } from "@headlessui/react";
import { Fragment } from "preact";
import { emit, listen } from "@tauri-apps/api/event";

export default function Home({ path }: { path: string }) {
  const list = useStore(scripts);

  useEffect(() => {
    invoke<Script[]>("get_scripts").then((data) => {
      setScripts(data);
    });

    let unlisten = listen<Pick<Script, "status" | "id">>(
      "script_end",
      (event) => {
        const { id, status } = event.payload;
        const script = list.find((s) => s.id === id);
        if (script) {
          script.status = status;

          updateScript(script);
        }
      }
    );
    return () => {
      unlisten.then((unlisten) => unlisten());
    };
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
