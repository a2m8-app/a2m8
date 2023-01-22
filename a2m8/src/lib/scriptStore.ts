import { invoke } from "@tauri-apps/api";
import { atom } from "nanostores";
import { Script } from "./script";

export const scripts = atom<Script[]>([]);

export const setScripts = (newScript: Script[]) => {
  scripts.set(newScript);
};

export function addScript(script: Script) {
  scripts.set([...scripts.get(), script]);
}
export function removeScript(id: string) {
  scripts.set(scripts.get().filter((script) => script.id !== id));
}
export function updateScript(script: Script) {
  scripts.set(
    scripts.get().map((s) => {
      if (s.id === script.id) {
        return script;
      }
      return s;
    }),
  );
}

export const fullReloadScripts = () =>
  invoke<Script[]>("get_scripts").then((data) => {
    setScripts(data);
  });
