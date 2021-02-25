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
  await update(form);
}

async function update(form) {
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
      let yaku = agari.judgeYaku(env);
      let list = document.createElement("dl");
      let header = document.createElement("dt");
      header.textContent = `${agari} (${yaku.name}${yaku.point}点 ${yaku.rank} ${yaku.fu}符)`;
      list.appendChild(header);
      let body = document.createElement("dd");
      let ul = document.createElement("ul");
      for (let [name, rank] of yaku.detail) {
        let li = document.createElement("li");
        li.textContent = `${name} (${rank})`;
        ul.appendChild(li);
      }
      body.appendChild(ul);
      list.appendChild(body);
      messageElem.appendChild(list);
    }
  } catch (e) {
    messageElem.textContent = e;
    throw e;
  }
}

function main() {
  let form = document.forms[0];
  update(form);
  form.addEventListener("submit", (e) => onSubmit(e, form));
}
