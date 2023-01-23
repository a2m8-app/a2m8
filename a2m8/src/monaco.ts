/*
 * license unknown
 * from https://github.com/suren-atoyan/monaco-react/issues/419#issuecomment-1335451932
 * kinda implies you can use it without license so i guess it's fine
 */

const init = async () => {
  // @ts-ignore shut up!!!
  const loader = await import("@monaco-editor/loader");
  const monaco = await import("monaco-editor");
  const editorWorker = await import(
    "monaco-editor/esm/vs/editor/editor.worker?worker"
  );
  const jsonWorker = await import(
    "monaco-editor/esm/vs/language/json/json.worker?worker"
  );
  const cssWorker = await import(
    "monaco-editor/esm/vs/language/css/css.worker?worker"
  );
  const htmlWorker = await import(
    "monaco-editor/esm/vs/language/html/html.worker?worker"
  );
  const tsWorker = await import(
    "monaco-editor/esm/vs/language/typescript/ts.worker?worker"
  );

  //@ts-ignore -
  self.MonacoEnvironment = {
    getWorker(_, label) {
      if (label === "json") {
        return new jsonWorker.default();
      }
      if (label === "css" || label === "scss" || label === "less") {
        return new cssWorker.default();
      }
      if (label === "html" || label === "handlebars" || label === "razor") {
        return new htmlWorker.default();
      }
      if (label === "typescript" || label === "javascript") {
        return new tsWorker.default();
      }
      return new editorWorker.default();
    },
  };
  loader.default.config({ monaco });
};

export default init();
