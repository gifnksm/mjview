import "./mahjong_hai";

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

function mjHai(hai) {
  let elem = document.createElement("mj-hai");
  elem.hai = hai;
  return elem;
}

function haiImage(hai) {
  switch (hai.type) {
    case "normal":
      return mjHai(hai.hai);
    case "sideways":
      return mjHai(`y_${hai.hai}`);
    case "hidden":
      return mjHai("_");
    case "stack": {
      let top = mjHai(`y_${hai.top}`);
      top.classList.add("stack-top");

      let bottom = mjHai(`y_${hai.bottom}`);
      bottom.classList.add("stack-bottom");

      let stack = document.createElement("span");
      stack.classList.add("stack");
      stack.appendChild(top);
      stack.appendChild(bottom);
      return stack;
    }
  }
}

function updateTehai(tehai) {
  let res = mod.parse_tehai(tehai);
  tehaiViewElem.textContent = "";

  let junTehaiElem = document.createElement("div");
  junTehaiElem.classList.add("jun-tehai");
  for (let hai of res.junTehai.toImage()) {
    junTehaiElem.appendChild(haiImage(hai));
  }
  tehaiViewElem.appendChild(junTehaiElem);

  for (let furo of res.furo) {
    let furoElem = document.createElement("div");
    furoElem.classList.add("furo");
    for (let hai of furo.toImage()) {
      furoElem.appendChild(haiImage(hai));
    }
    tehaiViewElem.appendChild(furoElem);
  }

  let agariHaiElem = document.createElement("div");
  agariHaiElem.classList.add("agari-hai");
  switch (res.agari.agari) {
    case "!":
      agariHaiElem.classList.add("tsumo");
      break;
    case "?":
      agariHaiElem.classList.add("ron");
      break;
  }
  agariHaiElem.appendChild(haiImage(res.agari.toImage()));
  tehaiViewElem.appendChild(agariHaiElem);
}

function main() {
  document.forms[0].addEventListener("submit", (e) => {
    e.preventDefault();
    messageElem.textContent = "";
    let form = e.currentTarget;
    try {
      updateTehai(form.elements["tehai"].value);
    } catch (e) {
      messageElem.textContent = e;
    }
  });
}
