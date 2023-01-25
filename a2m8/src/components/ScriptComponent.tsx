import { useEffect, useReducer, useRef, useState } from "preact/hooks";
import { FaStar, FaEllipsisV, FaCheck, FaBolt } from "react-icons/fa";
import { Menu, Popover, Transition } from "@headlessui/react";
import { Fragment } from "preact";
import { Script, scriptStatus, statusToText } from "../lib/script";
import { invoke } from "@tauri-apps/api";
import { fullReloadScripts, removeScript, scripts } from "../lib/scriptStore";
import { listen } from "@tauri-apps/api/event";
import ViewSource from "./ViewSource";
import { openScript } from "../lib/viewScriptState";

export default function ScriptComponent({
  script: scriptThing,
}: {
  script: Script;
}) {
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
        updateScript(script);
      }
    );
    return () => {
      unlisten.then((x) => x());
    };
  }, []);

  const openEditor = () => {
    openScript(script.id);
  };

  return (
    <div class={`bg-base-300 rounded-lg flex`}>
      <button
        onClick={start}
        class={`btn btn-sm p-4 mt-auto h-full w-9 rounded-r-none border-2 font-extrabold tracking-widest ${
          script.status == scriptStatus.running
            ? "btn-secondary-focus border-secondary"
            : "btn-primary-focus border-primary"
        }`}
      >
        <span class="-rotate-90">
          {script.status == scriptStatus.running ? "Stop" : "Run"}
        </span>
      </button>

      <div
        class="flex p-4 w-full"
        onContextMenu={(e) => {
          e.preventDefault();
          ref.current?.click();
        }}
      >
        <div>
          <div class="flex items-center justify-between">
            <div class="">
              <h3 class="text-lg font-medium">
                {script.name}
                <button
                  class={`mx-2 tooltip tooltip-info transition duration-200 ease-in-out  ${
                    script.favorite ? " text-yellow-400" : "text-gray-700"
                  }`}
                  data-tip={script.favorite ? "Un-favorite" : "Favorite"}
                  onClick={handleFavorite}
                >
                  <FaStar />
                </button>
                <button
                  class={`${
                    script.status == scriptStatus.running
                      ? "text-success"
                      : script.status == scriptStatus.error
                      ? "text-error"
                      : "text-info"
                  } tooltip tooltip-info cursor-default`}
                  data-tip={statusToText(script.status)}
                >
                  <FaBolt />
                </button>
              </h3>
              <p class="text-base-content">{script.description}</p>
            </div>
            <input
              type="button"
              name="rating-9"
              class={`mask text-xl mask-star-2 ${
                script.favorite ? "text-yellow-500" : "text-gray-500"
              }`}
              onClick={handleFavorite}
            />
          </div>
        </div>

        {script.error && (
          <div class="my-2 flex items-center">
            <p class="text-red-500 mr-4">Error:</p>
            <p class="text-gray-700">{script.error}</p>
          </div>
        )}
        <Popover class="relative ml-auto">
          <Popover.Button ref={ref}>
            <span class="sr-only">Options</span>
            <FaEllipsisV aria-hidden="true" />
          </Popover.Button>
          <Transition
            as={Fragment}
            enter="transition ease-out duration-100"
            enterFrom="transform opacity-0 scale-95"
            enterTo="transform opacity-100 scale-100"
            leave="transition ease-in duration-75"
            leaveFrom="transform opacity-100 scale-100"
            leaveTo="transform opacity-0 scale-95"
          >
            <Popover.Panel class="absolute z-10 right-4">
              <div class="px-4 py-2 bg-neutral rounded-md">
                <div class="form-control w-52">
                  <label class="cursor-pointer label">
                    <span class="label-text">Run on startup</span>
                    <input
                      type="checkbox"
                      class="toggle toggle-primary"
                      checked={script.startup}
                      onChange={handleStartup}
                    />
                  </label>
                </div>
                <div class="form-control w-52">
                  <label class="cursor-pointer label">
                    <span class="label-text">Edit script</span>
                    <button
                      type="button"
                      class="btn btn-square btn-outline btn-sm"
                      onClick={openEditor}
                    >
                      <FaCheck />
                    </button>
                  </label>
                </div>
                <div class="form-control w-52">
                  <label class="cursor-pointer label">
                    <span class="label-text">Reload</span>
                    <button
                      type="button"
                      disabled
                      class="btn btn-square btn-outline btn-sm"
                      onClick={openEditor}
                    >
                      <FaCheck />
                    </button>
                  </label>
                </div>
                <div class="form-control w-52 hover:bg-red-600 duration-300">
                  <label class="cursor-pointer label">
                    <span class="label-text">Delete</span>
                    <button
                      type="button"
                      class="btn btn-square btn-outline btn-sm"
                      onClick={deleteScript}
                    >
                      <FaCheck />
                    </button>
                  </label>
                </div>
              </div>
            </Popover.Panel>
          </Transition>
        </Popover>
      </div>
    </div>
  );
}
