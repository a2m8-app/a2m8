import { Dialog, Transition } from "@headlessui/react";
import Editor from "@monaco-editor/react";
import { useStore } from "@nanostores/react";
import { invoke } from "@tauri-apps/api";
import { Fragment } from "preact";
import { useEffect, useRef, useState } from "preact/hooks";
import { closeScript, viewScript } from "../lib/editScriptStore";
import { Script, scriptStatus } from "../lib/script";
import {
  addScript,
  getScriptFromId,
  removeScript,
  updateScript,
} from "../lib/scriptStore";

type RequiredData =
  | Script
  | { content: string; description?: string; name: string };

export default function ViewSource() {
  const data = useStore(viewScript);
  const [script, setScript] = useState<RequiredData>();
  const editorRef = useRef<any | null>(null);

  function handleEditorDidMount(editor: any, _monaco: unknown) {
    editorRef.current = editor;
    editor.focus();
  }

  useEffect(() => {
    if ("id" in data) {
      setScript(getScriptFromId(data.id));
    } else if ("name" in data) {
      setScript({
        content: data.content,
        description: data.description,
        name: data.name,
      });
    }
  }, [data]);

  if (!script) return <></>;
  return (
    <Transition.Root show={data.open} as={Fragment}>
      <Dialog as="div" class="relative z-10 " onClose={() => closeScript()}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div class="fixed inset-0 z-10 overflow-y-auto">
          <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0 mx-2">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel class="min-h-full min-w-full relative transform overflow-hidden rounded-lg bg-base-100 px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:p-6 h-full">
                <div class="min-h-full">
                  <div class="text-center">
                    <Dialog.Title
                      as="h3"
                      class="text-lg font-medium leading-6 text-base-content"
                    >
                      {"id" in script ? "Edit script" : "Add script"}
                    </Dialog.Title>
                    <div class="mt-2">
                      <Editor
                        height="70vh"
                        width="100%"
                        theme="vs-dark"
                        defaultLanguage="lua"
                        onMount={handleEditorDidMount}
                        defaultValue={script.content}
                        settings={{
                          scrollBeyondLastColumn: 5,
                          scrollBeyondLastLine: true,
                          wordWrap: "on",
                        }}
                      />
                    </div>
                  </div>
                </div>
                <div class="mt-5 sm:mt-6 flex gap-2">
                  <button
                    type="button"
                    class="inline-flex btn btn-error w-auto"
                    onClick={closeScript}
                  >
                    {"id" in script ? "Quit without saving" : "Cancel"}
                  </button>
                  <button
                    type="button"
                    class="inline-flex btn btn-success w-auto"
                    onClick={async () => {
                      if ("id" in script) {
                        updateScript({
                          ...script,
                          content: editorRef.current.getValue(),
                        });
                      } else {
                        const dat: Script = {
                          id: crypto.randomUUID(),
                          name: script.name,
                          description: script.description ?? "",
                          startup: false,
                          favorite: false,
                          status: scriptStatus.stopped,
                          content: editorRef.current.getValue(),
                        };

                        await invoke("create_script", { script: dat });
                        addScript(dat);
                      }

                      closeScript();
                    }}
                  >
                    {"id" in script ? "Save & quit" : "Add script"}
                  </button>
                  {"id" in script && (
                    <button
                      type="button"
                      class="ml-auto inline-flex btn btn-warning w-auto"
                      onClick={() => {
                        if ("id" in script) {
                          invoke("delete_script", { id: script.id }).then(
                            (r) => {
                              removeScript(script.id!);
                              closeScript();
                            }
                          );
                        }
                      }}
                    >
                      Delete Script this cannot be undone
                    </button>
                  )}
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}
