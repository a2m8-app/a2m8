import { Dialog, Transition } from "@headlessui/react";
import Editor from "@monaco-editor/react";
import { useStore } from "@nanostores/react";
import { Fragment } from "preact";
import { useEffect, useState } from "preact/hooks";
import { Script } from "../lib/script";
import { getScriptFromId } from "../lib/scriptStore";
import { closeScript, viewScript } from "../lib/viewScriptState";

export default function ViewSource() {
  const data = useStore(viewScript);
  const [script, setScript] = useState<Script>();

  useEffect(() => {
    if (data.id) {
      setScript(getScriptFromId(data.id));
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
                      Edit script
                    </Dialog.Title>
                    <div class="mt-2">
                      <Editor
                        height="70vh"
                        width="100%"
                        theme="vs-dark"
                        defaultLanguage="lua"
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
                    Quit without saving
                  </button>
                  <button
                    type="button"
                    class="inline-flex btn btn-success w-auto"
                    onClick={closeScript}
                  >
                    Save & quit
                  </button>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}