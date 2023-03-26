import Router from "preact-router";
import Home from "./routes/Home";
import {} from "react-icons/fa";
import "./monaco";
import NewScriptListener from "./components/NewScriptListener";
import ViewSource from "./components/ViewSource";
import SimpleHome from "./routes/SimpleHome";
import { useState } from "preact/hooks";

export function App<FC>() {
  let [simpleMode, setSimpleMode] = useState(false);
  return (
    <div>
      <div class="bg-base-200/20 w-full rounded-lg">
        <nav class="navbar max-w-[75rem] mx-auto">
          <div class="ml-4 flex-1 text-3xl">
            <p class="font-bold">A2</p>
            <p>M8</p>
          </div>
          <div class="flex-none">
            <ul class="menu menu-horizontal px-1">
              <li>
                <a>Wiki</a>
              </li>
              <li>
                <a>Discord</a>
              </li>
            </ul>
          </div>
          <div class="flex-none form-control">
            <label for="simple-mode" class="cursor-pointer label">
              <span class="label-text px-2">Simple mode</span>
              <input
                type="checkbox"
                class="toggle"
                id="simple-mode"
                onChange={(e) => setSimpleMode(e.target.checked)}
              />
            </label>
          </div>
        </nav>
      </div>
      <ViewSource />
      <NewScriptListener />
      <Router>
        {simpleMode ? <SimpleHome path="/" /> : <Home path="/" />}
      </Router>
    </div>
  );
}
