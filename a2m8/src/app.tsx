import Router from "preact-router";
import Home from "./routes/Home";
import {} from "react-icons/fa";
import "./monaco";

export function App<FC>() {
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
        </nav>
      </div>

      <Router>
        <Home path="/" />
      </Router>
    </div>
  );
}
