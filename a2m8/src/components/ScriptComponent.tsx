import { useState } from "preact/hooks";

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
    const handleStartup = () => setStartup(!startup);
    const handleViewContent = () => setShowContent(!showContent);

    return (
        <div className="bg-base-300 rounded-lg p-4">
            <div className="flex items-center justify-between">
                <div class={"grid grid-cols-2"}>
                    <h3 className="text-lg font-medium">{script.name}</h3><div 
                    className={`${status === 1 ? "bg-green-500" : status === 2 ? "bg-red-500" : "bg-orange-500"} rounded-full h-4 w-4 my-auto`}
                    />
                    <p className="text-base-content">{script.description}</p>
                </div>
                <button
                    className={`${isFavorite ? "text-yellow-500" : "text-gray-500"
                        } p-1 text-xl hover:text-black`}
                    onClick={handleFavorite}
                >
                    ❤️
                </button>
            </div>

            {script.error && (
                <div className="my-2 flex items-center">
                    <p className="text-red-500 mr-4">Error:</p>
                    <p className="text-gray-700">{script.error}</p>
                </div>
            )}
            <div className="my-2 flex items-center">
                <p className="text-gray-700 mr-4">Startup:</p>
                <select
                    className="bg-gray-200 rounded-lg p-2"
                    value={startup}
                    onChange={handleStartup}
                >
                    <option value={true}>Yes</option>
                    <option value={false}>No</option>
                </select>
            </div>
            <div className="my-2 flex items-center">
                <button
                    className="bg-blue-500 text-white p-2 rounded-lg hover:bg-blue-600"
                    onClick={handleViewContent}
                >
                    View Content
                </button>
            </div>
        </div>
    );
}