const { invoke } = window.__TAURI__.core;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;

let fanyiMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  fanyiMsgEl = document.querySelector("#fanyi-msg");  

  const appWebview = getCurrentWebviewWindow();
  console.log("appWebview"+appWebview);
  appWebview.listen('fanyi', (event) => {
      console.log("fanyi event comming.");
      $('.loader-wrapper').removeClass('is-active');
      fanyiMsgEl.textContent = event.payload;
  });
  appWebview.listen('start', (event) => {
    console.log("start event comming.");
    $('.loader-wrapper').addClass('is-active');
    //setTimeout(() => {
    //  $('.loader-wrapper').removeClass('is-active')
    //}, 5000)
});
});
