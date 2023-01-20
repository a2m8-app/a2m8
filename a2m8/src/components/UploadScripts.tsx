import { Ref, useRef, useState } from "preact/hooks";

export default function UploadScripts() {
  //create a typescript function to handle the file upload type the e parenter
  const handleFileUpload = (e: any) => {
    console.log(e.target.files);
  };
  const accept = "application/lua, text/x-lua, .lua";
  const inputRef = useRef<HTMLInputElement | any>(null);
  const handleDrop = (event: any) => {
    event.preventDefault();
    const files = event.dataTransfer.files;
    if (files.length > 0) {
      onFileSubmit(files[0]);
    }
  };
  const onFileSubmit = (f: File) => {};

  return (
    <div class="mx-auto bg-red w-64">
      <div
        className="bg-blue-500 hover:bg-blue-600 text-white p-2 rounded-md cursor-pointer"
        onDrop={handleDrop}
        onClick={(event) => inputRef.current!.click()}
        onDragOver={(event) => event.preventDefault()}
      >
        <input
          type="file"
          className="hidden"
          accept={accept}
          ref={inputRef}
          onChange={(event: any) => onFileSubmit(event.target!.files[0])}
        />
        {"Drop file here or click to browse"}
      </div>
    </div>
  );
}
