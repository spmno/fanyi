const { invoke } = window.__TAURI__.core;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;

let fanyiMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  fanyiMsgEl = document.querySelector("#fanyi-msg");  

  const appWebview = getCurrentWebviewWindow();
  console.log("appWebview"+appWebview);
  appWebview.listen('fanyi', (event) => {
      console.log("event comming.");
      fanyiMsgEl.textContent = event.payload;
  });
});
