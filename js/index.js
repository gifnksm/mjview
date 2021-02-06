import "./mahjong_hai";
import "./mahjong_tehai";

let mod, messageElem, tehaiViewElem;

Promise.all([
  import("../pkg/index.js").then((wasm) => (mod = wasm)),
  new Promise((resolve) => {
    messageElem = document.getElementById("message");
    tehaiViewElem = document.getElementById("tehai-view");
    document.addEventListener("DOMContentLoaded", (_e) => resolve());
  }),
])
  .then(main)
  .catch(console.error);

function main() {
  let form = document.forms[0];
  tehaiViewElem.tehai = form.elements["tehai"].value;
  form.addEventListener("submit", (e) => {
    e.preventDefault();
    messageElem.textContent = "";
    try {
      tehaiViewElem.tehai = form.elements["tehai"].value;
    } catch (e) {
      messageElem.textContent = e;
    }
  });
}
