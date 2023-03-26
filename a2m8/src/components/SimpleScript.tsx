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
import createScriptUtilities from "../lib/createScriptUtilities";

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
