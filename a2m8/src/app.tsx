import Router from "preact-router";
import Home from "./routes/Home";

import "./monaco";

export function App<FC>() {
  return (
    <Router>
      <Home path="/" />
    </Router>
  );
}
