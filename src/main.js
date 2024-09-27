const { invoke } = window.__TAURI__.core;
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

let fanyiMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  fanyiMsgEl = document.querySelector("#fanyi-msg");
  fanyiMsgEl.textContent = getCurrentWebviewWindow().id;
  const appWebview = getCurrentWebviewWindow();
    appWebview.listen<string>('fanyi', (event) => {
    fanyiMsgEl.textContent = event.payload;
  });
});
