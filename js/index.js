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
    this._updateWarning();

    let tehai = this._tehai;
    if (tehai === null || this._form.querySelector(".error") !== null) {
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

    let list = document.createElement("dl");
    for (let [agari, yaku] of comb) {
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
    }
    this._messageElement.appendChild(list);
  }

  _updateWarning() {
    this._clearWarning("tehai");
    this._clearWarning("richi");
    this._clearWarning("guzen");
    this._clearWarning("dora");
    this._clearWarning("uradora");

    let warnings;
    if (this._tehai !== null) {
      warnings = this._env.checkPropsWithTehai(this._tehai);
    } else {
      warnings = this._env.checkPropsWithoutTehai();
    }

    for (let [items, message] of warnings) {
      let set = new Set();
      for (let item of items) {
        switch (item) {
          case "tehai":
            set.add("tehai");
            break;
          case "richi":
          case "daburi":
            set.add("richi");
            break;
          case "ippatsu":
          case "rinshan":
          case "haitei":
          case "tenho":
            set.add("guzen");
            break;
          case "bakaze":
          case "jikaze":
            throw new Error(`unexpected item: ${item}`);
          case "dora":
            set.add("dora");
            break;
          case "uradora":
            set.add("uradora");
            break;
          default:
            throw new Error(`unexpected item: ${item}`);
        }
      }
      for (let category of set.values()) {
        this._addWarningMessage(category, message);
      }
    }
  }

  _onChange(target) {
    let { Hai } = this._wasmMod;
    switch (target.name) {
      case "richi":
        if (target.checked) {
          this._setRichi(target.value);
        }
        break;
      case "ippatsu":
      case "rinshan":
      case "haitei":
      case "tenho":
      case "aotenjo":
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

  _removeMessageClass(name) {
    let element = document.getElementById(`${name}-message`);
    element.classList.remove("ok");
    element.classList.remove("warning");
    element.classList.remove("error");
  }

  _addMessage(name, addClass, message, clear) {
    let element = document.getElementById(`${name}-message`);
    if (!element.classList.contains(addClass)) {
      element.textContent = "";
    }
    this._removeMessageClass(name);
    element.classList.add(addClass);
    if (clear) {
      element.textContent = "";
    }
    let li = document.createElement("li");
    li.textContent = message;
    element.appendChild(li);
  }

  _setOKMessage(name) {
    this._addMessage(name, "ok", "OK", true);
  }

  _clearWarning(name) {
    let element = document.getElementById(`${name}-message`);
    if (element.classList.contains("warning")) {
      element.classList.remove("warning");
      element.textContent = "";
    }
    if (!element.classList.contains("error")) {
      this._setOKMessage(name);
    }
  }

  _addWarningMessage(name, message) {
    let element = document.getElementById(`${name}-message`);
    if (element.classList.contains("error")) {
      return;
    }
    this._addMessage(name, "warning", message, false);
  }

  _setErrorMessage(name, message) {
    this._clearWarning(name);
    this._addMessage(name, "error", message, true);
  }

  _onInput(target) {
    switch (target.name) {
      case "tehai": {
        let tehai = target.value;
        this._tehaiElement.tehai = tehai;
        let { Tehai } = this._wasmMod;
        try {
          this._tehai = Tehai.fromStr(tehai);
          target.setCustomValidity("");
          this._setOKMessage(target.name);
        } catch (err) {
          this._tehai = null;
          target.setCustomValidity(err.toString());
          this._setErrorMessage(target.name, err.toString());
        }
        break;
      }
      case "dora":
      case "uradora": {
        try {
          if (target.name === "dora") {
            this._env.setDora(target.value);
          } else {
            this._env.setUradora(target.value);
          }
          target.setCustomValidity("");
          this._setOKMessage(target.name);
        } catch (err) {
          target.setCustomValidity(err.toString());
          this._setErrorMessage(target.name, err.toString());
        }
        break;
      }
      default:
        break;
    }
    this._update();
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
