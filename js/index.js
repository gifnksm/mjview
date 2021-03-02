import "./mahjong_hai";
import "./mahjong_tehai";
import "./mahjong_furo";

class EnvInput {
  constructor(wasmMod, form, tehaiElement, messageElement) {
    this._wasmMod = wasmMod;
    this._form = form;
    this._tehaiElement = tehaiElement;
    this._messageElement = messageElement;

    this._tehai = null;
    let { Env } = wasmMod;
    this._env = new Env();
    this._doraValid = false;
    this._uradoraValid = false;

    for (let element of form.elements) {
      this._onChange(element);
      this._onInput(element);
    }

    form.addEventListener("submit", (e) => {
      e.preventDefault();
      this._update();
    });
    form.addEventListener("change", (e) => this._onChange(e.target));
    form.addEventListener("input", (e) => this._onInput(e.target));
  }

  get tehai() {
    return this._tehai;
  }

  get env() {
    return this._env;
  }

  _update() {
    this._messageElement.textContent = "";
    let tehai = this._tehai;
    if (tehai === null || !this._doraValid || !this._uradoraValid) {
      return;
    }

    let comb = tehai
      .toAgariCombinations()
      .map((agari) => [agari, agari.judgeYaku(this._env)])
      .sort(([agariA, yakuA], [agariB, yakuB]) => {
        let c = yakuA.compare(yakuB);
        if (c !== 0) {
          return -c;
        }
        return agariA.compare(agariB);
      });

    for (let [agari, yaku] of comb) {
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
      this._messageElement.appendChild(list);
    }
  }

  _onChange(target) {
    let { Hai } = this._wasmMod;
    switch (target.name) {
      case "tenho":
        if (target.checked) {
          this._setTenho(target.value);
        }
        break;
      case "richi":
        if (target.checked) {
          this._setRichi(target.value);
        }
        break;
      case "ippatsu":
      case "rinshan":
      case "haitei":
        this._env[target.name] = target.checked;
        break;
      case "bakaze":
      case "jikaze":
        this._env[target.name] = Hai.fromStr(target.value);
        break;
      default:
        break;
    }
    this._update();
  }

  _onInput(target) {
    switch (target.name) {
      case "tehai": {
        let { Tehai } = this._wasmMod;
        try {
          let tehai = target.value;
          this._tehai = Tehai.fromStr(tehai);
          this._tehaiElement.tehai = tehai;
          target.setCustomValidity("");
        } catch (e) {
          this._tehai = null;
          this._tehaiElement.tehai = "";
          target.setCustomValidity(e.toString());
        }
        break;
      }
      case "dora":
      case "uradora": {
        try {
          if (target.name === "dora") {
            this._env.setDora(target.value);
            this._doraValid = true;
          } else {
            this._env.setUradora(target.value);
            this._uradoraValid = true;
          }
          target.setCustomValidity("");
        } catch (err) {
          target.setCustomValidity(err.toString());
          if (target.name == "dora") {
            this._doraValid = false;
            this._uradoraValid = false;
          }
        }
        break;
      }
      default:
        break;
    }
    this._update();
  }

  _setTenho(value) {
    switch (value) {
      case "tenho":
        this._env.tenho = true;
        this._env.chiho = false;
        break;
      case "chiho":
        this._env.tenho = false;
        this._env.chiho = true;
        break;
      default:
        this._env.tenho = false;
        this._env.chiho = false;
        break;
    }
  }

  _setRichi(value) {
    switch (value) {
      case "richi":
        this._env.richi = true;
        this._env.daburi = false;
        break;
      case "daburi":
        this._env.richi = false;
        this._env.daburi = true;
        break;
      default:
        this._env.richi = false;
        this._env.daburi = false;
        break;
    }
  }
}

document.addEventListener("DOMContentLoaded", (_e) => {
  main();
});

async function main() {
  let wasmMod = await import("../pkg/index.js");
  new EnvInput(
    wasmMod,
    document.forms[0],
    document.getElementById("tehai-view"),
    document.getElementById("message"),
  );
}
