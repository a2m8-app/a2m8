import { useEffect, useReducer, useRef, useState } from "preact/hooks";
import { FaStar, FaEllipsisV, FaCheck, FaBolt } from "react-icons/fa";
import { Menu, Popover, Transition } from "@headlessui/react";
import { Fragment } from "preact";
import { Script, scriptStatus, statusToText } from "../lib/script";
import { invoke } from "@tauri-apps/api";
import { fullReloadScripts, removeScript, scripts } from "../lib/scriptStore";
import { listen } from "@tauri-apps/api/event";
import ViewSource from "./ViewSource";
import { openScript } from "../lib/editScriptStore";

function createScriptUtilities(scriptThing: Script) {
  const [script, updateScript] = useReducer(
    (state: Script, data: Partial<Script>) => {
      let newScript = { ...state, ...data };
      invoke("update_script", { script: newScript });
      return newScript;
    },
    scriptThing
  );

  const ref = useRef<HTMLDivElement>(null);

  const handleFavorite = (e: any) =>
    updateScript({ favorite: !script.favorite });
  const handleStartup = (e: any) =>
    updateScript({
      startup: !script.startup,
    });
  const start = () => {
    if (script.status == scriptStatus.running) {
      invoke("stop_script", { id: script.id }).then((r) => {
        updateScript({ status: scriptStatus.stopped });
      });
    } else {
      invoke("start_script", { id: script.id }).then((r) => {
        updateScript({ status: scriptStatus.running });
      });
    }
  };

  const deleteScript = () => {
    invoke("delete_script", { id: script.id }).then((r) => {
      removeScript(script.id);
    });
  };

  useEffect(() => {
    let unlisten = listen<Pick<Script, "status" | "id">>(
      "script_end",
      (event) => {
        const { id, status } = event.payload;
        if (id != script.id) return;
        updateScript({ status, id });
        // lmao why did i think this was a good idea...
        // updateScript(script);
        console.log(event.payload);
      }
    );
    return () => {
      unlisten.then((x) => x());
    };
  }, []);

  const openEditor = () => {
    openScript({ id: script.id });
  };
  return {
    script,
    updateScript,
    ref,
    handleFavorite,
    handleStartup,
    start,
    deleteScript,
    openEditor,
  };
}

export default function ScriptComponent({
  script: scriptThing,
}: {
  script: Script;
}) {
  const {
    script,
    updateScript,
    ref,
    handleFavorite,
    handleStartup,
    start,
    deleteScript,
    openEditor,
  } = createScriptUtilities(scriptThing);

  return (
    <tr>
      <td>{script.name}</td>
      <td>{script.favorite ? "Yes" : "No"}</td>
      <td>{script.startup ? "Yes" : "No"}</td>
      <td>{statusToText(script.status)}</td>
      <td class="flex gap-2">
        <button onClick={start}>Run</button>
        <button onClick={start}>Stop</button>
        <button onClick={openEditor}>Edit</button>
        <button onClick={deleteScript}>Delete</button>
        <button onClick={handleStartup}>Startup</button>
        <button onClick={handleFavorite}>Favorite</button>
      </td>
    </tr>
  );
}
