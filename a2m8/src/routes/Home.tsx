import ScriptComponent, { Script, scriptStatus } from "../components/ScriptComponent";

export default function Home({ path }: { path: string }) {
    let scripts:Script[] = [
        {
            id: crypto.randomUUID(),
            name: "mycustomshortcuts.lua",
            description: "My custom shortcuts",
            startup: true,
            favorite: false,
            status: scriptStatus.running,
            content: "some content thats made with lua"
        },
        {
            id: crypto.randomUUID(),
            name: "screenshots.lua",
            description: "makes screenshots every 10 minutes",
            startup: true,
            favorite: false,
            status: scriptStatus.stopped,
            content: "some content thats made with lua"
        },
        {
            id: crypto.randomUUID(),
            name: "startupcommands.lua",
            description: "some commands that run on startup",
            startup: true,
            favorite: false,
            status: scriptStatus.error,
            content: "some content thats made with lua",
        },
        {
            id: crypto.randomUUID(),
            name: "automategame.lua",
            description: "automates a game",
            startup: false,
            favorite: true,
            status: scriptStatus.running,
            content: "some content thats made with lua",
        }
    ]
    return (
        <div>
            <h1>Home</h1>
            <p>This is the Home component.</p>
            <div class={"w-[75rem] mx-auto grid grid-cols-1 gap-2"}>
                {scripts.map((script) => (
                    <ScriptComponent script={script} />
                ))}
            
            </div>
        </div>
    );
}