import { giteaApi } from "gitea-js";
import { GITEA } from "../consts";

export const gitea = giteaApi(GITEA, {
  token:
    typeof localStorage != "undefined" && localStorage.getItem("giteaToken") ||
    undefined,
});
