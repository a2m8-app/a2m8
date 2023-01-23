import { atom } from "nanostores";

type ViewScript = {
  id: string;
  open: boolean;
};

export const viewScript = atom<ViewScript>({
  id: "",
  open: false,
});

export const setScripts = (newScript: ViewScript) => {
  viewScript.set(newScript);
};
export const openScript = (id: string) => {
  viewScript.set({ id, open: true });
};
export const closeScript = () => {
  viewScript.set({ id: "", open: false });
};
