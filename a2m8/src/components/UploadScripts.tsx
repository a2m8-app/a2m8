import { invoke } from "@tauri-apps/api";
import { useRef } from "preact/hooks";
import { openScript } from "../lib/editScriptStore";
import { Script, scriptStatus } from "../lib/script";
import { addScript } from "../lib/scriptStore";

export default function UploadScripts() {
  const accept = "application/lua, text/x-lua, .lua";
  const inputRef = useRef<HTMLInputElement | any>(null);
  const handleDrop = async (event: any) => {
    const files = event.dataTransfer.files;
    if (files.length > 0) {
      await onFileSubmit(files[0]);
    }
  };
  const onFileSubmit = async (f: File) => {
    if (!f) return;
    let content = await f.text();
    // extracts @description markers from the file
    let description = content.match(/@description\s(.*)/);

    openScript({
      name: f.name,
      description: description?.[1] ? description[1] : "",
      content: content,
    });
    // @ts-ignore (us this comment if typescript raises an error)
    inputRef.current.value = "";
  };

  return (
    <div class="mx-auto flex justify-center">
      <div class="form-control flex justify-center">
        <button class="btn btn-primary">
          <label class="custom-file-upload ">
            <input
              type="file"
              class="hidden"
              accept={accept}
              ref={inputRef}
              onDrop={(event: any) => handleDrop(event)}
              onChange={(event: any) => onFileSubmit(event.target!.files[0])}
            />
            Drop file here or click to browse
          </label>
        </button>
      </div>
    </div>
  );
}
