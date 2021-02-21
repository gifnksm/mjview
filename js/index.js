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

  let { Tehai, Hai } = await import("../pkg/index.js");

  messageElem.textContent = "";
  let tehai = form.elements["tehai"].value;
  let bakaze = Hai.fromStr(form.elements["bakaze"].value);
  let jikaze = Hai.fromStr(form.elements["jikaze"].value);

  let res;
  try {
    tehaiViewElem.tehai = tehai;
    res = Tehai.fromStr(tehai);
  } catch (e) {
    messageElem.textContent = e;
    throw e;
  }

  let comb = res.toAgariCombinations();
  for (let agari of comb) {
    messageElem.appendChild(
      document.createTextNode(
        `${agari} (${agari.computeFu(bakaze, jikaze)}符)`,
      ),
    );
    messageElem.appendChild(document.createElement("br"));
  }
}

function main() {
  let form = document.forms[0];
  tehaiViewElem.tehai = form.elements["tehai"].value;
  form.addEventListener("submit", (e) => onSubmit(e, form));
}
