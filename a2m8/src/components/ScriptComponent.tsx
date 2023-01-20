import { useState } from "preact/hooks";
import { FaStar, FaEllipsisV } from "react-icons/fa";
import { Menu, Popover, Transition } from '@headlessui/react'
import { Fragment } from "preact";

export type Script = {
    id: string,
    name: `${string}.lua`,
    description: string,
    startup: boolean,
    favorite: boolean,
    content: string,
    error?: string,
    status: scriptStatus,
}
export type scriptStatus = typeof scriptStatus[keyof typeof scriptStatus];
export const scriptStatus = {
    running: 1,
    stopped: 2,
    error: 3,
} as const;



export default function ScriptComponent({ script }: { script: Script }) {
    const [isFavorite, setIsFavorite] = useState(script.favorite);
    const [status, setStatus] = useState(script.status);
    const [startup, setStartup] = useState(script.startup);
    const [showContent, setShowContent] = useState(false);

    const handleFavorite = () => setIsFavorite(!isFavorite);
    const handleStartup = (e) => {
        setStartup(!startup)
        e.stopPropagation();
    };
    const handleViewContent = () => setShowContent(!showContent);

    return (
        <div class="bg-base-300 rounded-lg p-4 flex">
            <div >
                <div class="flex items-center justify-between">
                    <div class="">
                        <h3 class="text-lg font-medium">{script.name}    <button onClick={handleFavorite}>
                            <FaStar color={isFavorite ? "yellow" : "gray"} />
                        </button>
                        </h3>
                        <p class="text-base-content">{script.description}</p>
                    </div>
                    <input type="button" name="rating-9" class={`mask text-xl mask-star-2 ${isFavorite ? "text-yellow-500" : "text-gray-500"}`} onClick={handleFavorite} />
                </div>

                {script.error && (
                    <div class="my-2 flex items-center">
                        <p class="text-red-500 mr-4">Error:</p>
                        <p class="text-gray-700">{script.error}</p>
                    </div>
                )}
            </div>



            <Popover class="relative ml-auto">
                <Popover.Button><span class="sr-only">Options</span> <FaEllipsisV aria-hidden="true" /></Popover.Button>
                <Transition
                    as={Fragment}
                    enter="transition ease-out duration-100"
                    enterFrom="transform opacity-0 scale-95"
                    enterTo="transform opacity-100 scale-100"
                    leave="transition ease-in duration-75"
                    leaveFrom="transform opacity-100 scale-100"
                    leaveTo="transform opacity-0 scale-95"
                >
                    <Popover.Panel class="absolute z-10">
                        <div class="px-4 py-2 bg-neutral rounded-md">
                            <div class="form-control w-52">
                                <label class="cursor-pointer label">
                                    <span class="label-text">Run on startup</span>
                                    <input type="checkbox" class="toggle toggle-primary" checked={startup} onChange={handleStartup} />
                                </label>
                            </div>
                        </div>
                    </Popover.Panel>
                </Transition>
            </Popover>

        </div>
    );
}