import "./mahjong_hai";
import "./mahjong_tehai";
import "./mahjong_furo";

let messageElem, tehaiViewElem;

document.addEventListener("DOMContentLoaded", (_e) => {
  messageElem = document.getElementById("message");
  tehaiViewElem = document.getElementById("tehai-view");
  main();
});

async function onSubmit(e, form) {
  e.preventDefault();
  messageElem.textContent = "";
  let tehai = form.elements["tehai"].value;

  let res;
  try {
    tehaiViewElem.tehai = tehai;
    let mod = await import("../pkg/index.js");
    res = mod.parse_tehai(tehai);
  } catch (e) {
    messageElem.textContent = e;
  }

  let comb = res.toMentsuCombinations();
  messageElem.textContent = "[" + comb.join("] [") + "] + " + res.furo;
  console.log(comb);
}

function main() {
  let form = document.forms[0];
  tehaiViewElem.tehai = form.elements["tehai"].value;
  form.addEventListener("submit", (e) => onSubmit(e, form));
}
