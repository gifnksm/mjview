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

  let { Tehai, Hai, Env } = await import("../pkg/index.js");

  messageElem.textContent = "";
  try {
    let tehai = form.elements["tehai"].value;
    tehaiViewElem.tehai = tehai;
    let res = Tehai.fromStr(tehai);

    let env = new Env();
    switch (form.elements["tenho"].value) {
      case "tenho":
        env.tenho = true;
        break;
      case "chiho":
        env.chiho = true;
        break;
      default:
        break;
    }
    switch (form.elements["richi"].value) {
      case "richi":
        env.richi = true;
        break;
      case "daburi":
        env.daburi = true;
        break;
      default:
        break;
    }
    env.ippatsu = form.elements["ippatsu"].checked;
    env.rinshan = form.elements["rinshan"].checked;
    env.haitei = form.elements["haitei"].checked;
    env.bakaze = Hai.fromStr(form.elements["bakaze"].value);
    env.jikaze = Hai.fromStr(form.elements["jikaze"].value);
    env.setDora(form.elements["dora"].value);
    env.setUradora(form.elements["uradora"].value);

    let comb = res.toAgariCombinations();
    for (let agari of comb) {
      messageElem.appendChild(
        document.createTextNode(`${agari} (${agari.computeFu(env)}符)`),
      );
      messageElem.appendChild(document.createElement("br"));
      for (let [yaku, rank] of agari.judgeYaku(env)) {
        let text = `${yaku}`;
        if (rank.fan !== undefined) {
          text += ` (${rank.fan}飜)`;
        }
        messageElem.appendChild(document.createTextNode(text));
        messageElem.appendChild(document.createElement("br"));
      }
    }
  } catch (e) {
    messageElem.textContent = e;
    throw e;
  }
}

function main() {
  let form = document.forms[0];
  tehaiViewElem.tehai = form.elements["tehai"].value;
  form.addEventListener("submit", (e) => onSubmit(e, form));
}
