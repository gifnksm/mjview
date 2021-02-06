import "./mahjong_hai";
import "./mahjong_tehai";

let messageElem, tehaiViewElem;

document.addEventListener("DOMContentLoaded", (_e) => {
  messageElem = document.getElementById("message");
  tehaiViewElem = document.getElementById("tehai-view");
  main();
});

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
