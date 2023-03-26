import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useReducer, useRef } from "preact/hooks";
import { openScript } from "./editScriptStore";
import { Script, scriptStatus } from "./script";
import { removeScript, scripts } from "./scriptStore";

export default function createScriptUtilities(scriptThing: Script) {
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
      }
    );
    // this fixes a bug where saving the script from the editor does not change the contents
    let unlisten2 = scripts.listen((s) => {
      const newScript = s.find((s) => s.id == script.id);
      if (newScript) {
        updateScript(newScript);
      }
    });
    return () => {
      unlisten.then((x) => x());
      unlisten2();
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
