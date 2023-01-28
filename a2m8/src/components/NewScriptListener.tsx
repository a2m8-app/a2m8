import { listen } from "@tauri-apps/api/event";
import { onMount } from "nanostores";
import { useEffect } from "preact/hooks";
import { openScript } from "../lib/editScriptStore";

export default function NewScriptListener() {
  useEffect(() => {
    let unlisten = listen<{ name: string; content: string }>(
      "create_w_prompt",
      ({ payload: { name, content } }) => {
        openScript({ name, content });
      }
    );
    return () => {
      unlisten.then((x) => x());
    };
  }, []);
  return <></>;
}
