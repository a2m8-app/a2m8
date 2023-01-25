import { invoke } from "@tauri-apps/api";
import { useRef } from "preact/hooks";
import { Script, scriptStatus } from "../lib/script";
import { addScript } from "../lib/scriptStore";
import { openScript } from "../lib/viewScriptState";

export default function UploadScripts() {
  const accept = "application/lua, text/x-lua, .lua";
  const inputRef = useRef<HTMLInputElement | any>(null);
  const handleDrop = async (event: any) => {
    event.preventDefault();
    const files = event.dataTransfer.files;
    if (files.length > 0) {
      await onFileSubmit(files[0]);
    }
  };
  const onFileSubmit = async (f: File) => {
    let content = await f.text();
    // extracts @description markers from the file
    let description = content.match(/@description\s(.*)/);
    const script: Script = {
      id: crypto.randomUUID(),
      name: f.name,
      description: description ? description[1] : "",
      startup: false,
      favorite: false,
      status: scriptStatus.stopped,
      content: content,
    };

    await invoke("create_script", { script: script });
    addScript(script);
    openScript(script.id);
    // @ts-ignore (us this comment if typescript raises an error)
    inputRef.current.value = "";
  };

  return (
    <div class="flex justify-center">
      <button
        class="btn btn-accent btn-sm"
        onDrop={handleDrop}
        onClick={(_) => inputRef.current!.click()}
        onDragOver={(event) => event.preventDefault()}
      >
        <input
          type="file"
          className="hidden"
          accept={accept}
          ref={inputRef}
          onChange={(event: any) => onFileSubmit(event.target!.files[0])}
        />
        Drop file here or click to browse
      </button>
    </div>
  );
}
