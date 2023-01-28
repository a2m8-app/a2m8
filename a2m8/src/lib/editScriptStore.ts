import { atom } from "nanostores";

type EditScript = {
  id: string;
};

type CreateScript = {
  name: string;
  content: string;
  description?: string;
};

type ViewScript = (EditScript | CreateScript) & {
  open: boolean;
};

export const viewScript = atom<ViewScript>({
  id: "",
  open: false,
});

export const openScript = (data: EditScript | CreateScript) => {
  viewScript.set({ ...data, open: true });
};
export const closeScript = () => {
  viewScript.set({ id: "", open: false });
};
