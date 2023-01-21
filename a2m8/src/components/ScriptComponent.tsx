import { useState } from "preact/hooks";
import { FaStar, FaEllipsisV, FaCheck, FaBolt } from "react-icons/fa";
import { Menu, Popover, Transition } from "@headlessui/react";
import { Fragment } from "preact";

export type Script = {
  id: string;
  name: `${string}.lua`;
  description: string;
  startup: boolean;
  favorite: boolean;
  content: string;
  error?: string;
  status: scriptStatus;
};
export type scriptStatus = typeof scriptStatus[keyof typeof scriptStatus];
export const scriptStatus = {
  running: 1,
  stopped: 2,
  ended: 3,
  error: 4,
} as const;

function statusToText(status: scriptStatus) {
  switch (status) {
    case scriptStatus.running:
      return "Running";
    case scriptStatus.stopped:
      return "Stopped";
    case scriptStatus.ended:
      return "Ended";
    case scriptStatus.error:
      return "Error";
  }
}

export default function ScriptComponent({ script }: { script: Script }) {
  const [isFavorite, setIsFavorite] = useState(script.favorite);
  const [status, setStatus] = useState(script.status);
  const [startup, setStartup] = useState(script.startup);
  const [showContent, setShowContent] = useState(false);

  const handleFavorite = () => setIsFavorite(!isFavorite);
  const handleStartup = (e: any) => {
    setStartup(!startup);
    e.stopPropagation();
  };
  const handleViewContent = () => setShowContent(!showContent);

  return (
    <div class={`bg-base-300 rounded-lg flex`}>
      <button
        class={`btn btn-sm p-4 mt-auto h-full w-9 rounded-r-none border-2 font-extrabold tracking-widest ${
          status == scriptStatus.running
            ? "btn-secondary-focus border-secondary"
            : "btn-primary-focus border-primary"
        }`}
      >
        <span class="-rotate-90">
          {status == scriptStatus.running ? "Stop" : "Run"}
        </span>
      </button>
      <div class="flex p-4 w-full">
        <div>
          <div class="flex items-center justify-between">
            <div class="">
              <h3 class="text-lg font-medium">
                {script.name}
                <button
                  class={`mx-2 tooltip tooltip-info transition duration-200 ease-in-out  ${
                    isFavorite ? " text-yellow-400" : "text-gray-700"
                  }`}
                  data-tip={isFavorite ? "Un-favorite" : "Favorite"}
                  onClick={handleFavorite}
                >
                  <FaStar />
                </button>
                <button
                  class={`${
                    status == scriptStatus.running
                      ? "text-success"
                      : status == scriptStatus.error
                      ? "text-error"
                      : "text-info"
                  } tooltip tooltip-info cursor-default`}
                  data-tip={statusToText(status)}
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
                isFavorite ? "text-yellow-500" : "text-gray-500"
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
          <Popover.Button>
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
                      checked={startup}
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
                      checked={showContent}
                      onChange={handleViewContent}
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
                      class="btn btn-square btn-outline btn-sm"
                      checked={showContent}
                      onChange={handleViewContent}
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
                      checked={showContent}
                      onChange={handleViewContent}
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
