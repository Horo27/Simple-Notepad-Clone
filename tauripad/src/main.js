const { invoke } = window.__TAURI__.core;

const textArea = document.querySelector(".textarea");
const buttonOpen = document.querySelector("#open");
const buttonSave = document.querySelector("#save");
const buttonClear = document.querySelector("#clear");

let path

window.addEventListener("DOMContentLoaded", () => {
  buttonOpen.addEventListener("click", async () =>{
    textArea.value = "";
    const fileInfo = await invoke("open_file", {});
    path = fileInfo.filePath;
    textArea.value = fileInfo.fileContents;
  });

  buttonSave.addEventListener("click", async () =>{
    const newContent = textArea.value;
    console.log(path, newContent)
    await invoke("modify_file", {filePath:path,fileContents: newContent})
  })

  buttonClear.addEventListener("click", async () => {
    await invoke("modify_file", {filePath:path,fileContents: ""})
    textArea.value = "";
  })

});




