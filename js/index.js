import "./mahjong_hai";
import "./mahjong_tehai";
import "./mahjong_furo";

class EnvInput {
  constructor(wasmMod, form, tehaiElement, outputElement) {
    this._wasmMod = wasmMod;
    this._form = form;
    this._tehaiElement = tehaiElement;
    this._outputElement = outputElement;
    this._messageElementMap = new Map();
    this._messageElements = new Set();

    for (let elem of form.elements) {
      let name = elem.name;
      let base = elem.parentNode;
      while (base.nodeName != "TD") {
        base = base.parentNode;
      }
      let message = base.querySelector("ul:last-child[id$=-message]");
      this._messageElementMap.set(name, message);
      if (message !== null) {
        this._messageElements.add(message);
      }
    }

    this._tehai = null;
    let { Env } = wasmMod;
    this._env = new Env();

    for (let [name, value] of new URLSearchParams(location.search)) {
      let element = this._form[name];
      if (element.type == "checkbox") {
        element.checked = true;
      } else {
        element.value = value;
      }
    }

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

  _update() {
    this._outputElement.textContent = "";
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
    this._outputElement.appendChild(list);

    let formData = new FormData(this._form);
    let params = new URLSearchParams(formData);
    history.replaceState(null, null, `?${params}`);
  }

  _updateWarning() {
    for (let elem of this._messageElements) {
      this._clearWarning(elem);
    }

    let warnings;
    if (this._tehai !== null) {
      warnings = this._env.checkPropsWithTehai(this._tehai);
    } else {
      warnings = this._env.checkPropsWithoutTehai();
    }

    for (let [items, message] of warnings) {
      let set = new Set();
      for (let item of items) {
        let messageElement = this._messageElementMap.get(item);
        set.add(messageElement);
      }
      for (let element of set.values()) {
        this._addWarningMessage(element, message);
      }
    }
  }

  _onChange(target) {
    let { Hai } = this._wasmMod;
    switch (target.name) {
      case "richi":
        if (target.checked) {
          this._env[target.name] = target.value;
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

  _addMessage(element, addClass, message, clear) {
    if (!element.classList.contains(addClass)) {
      element.textContent = "";
    }
    element.classList.remove("ok");
    element.classList.remove("warning");
    element.classList.remove("error");
    element.classList.add(addClass);
    if (clear) {
      element.textContent = "";
    }
    let li = document.createElement("li");
    li.textContent = message;
    element.appendChild(li);
  }

  _setOKMessage(element) {
    this._addMessage(element, "ok", "OK", true);
  }

  _clearWarning(element) {
    if (element.classList.contains("warning")) {
      element.classList.remove("warning");
      element.textContent = "";
    }
    if (!element.classList.contains("error")) {
      this._setOKMessage(element);
    }
  }

  _addWarningMessage(element, message) {
    if (element.classList.contains("error")) {
      return;
    }
    this._addMessage(element, "warning", message, false);
  }

  _setErrorMessage(element, message) {
    this._clearWarning(element);
    this._addMessage(element, "error", message, true);
  }

  _onInput(target) {
    switch (target.name) {
      case "tehai": {
        let tehai = target.value;
        let messageElement = this._messageElementMap.get(target.name);
        this._tehaiElement.tehai = tehai;
        let { Tehai } = this._wasmMod;
        try {
          this._tehai = Tehai.fromStr(tehai);
          target.setCustomValidity("");
          this._setOKMessage(messageElement);
        } catch (err) {
          this._tehai = null;
          target.setCustomValidity(err.toString());
          this._setErrorMessage(messageElement, err.toString());
        }
        break;
      }
      case "dora":
      case "uradora": {
        let messageElement = this._messageElementMap.get(target.name);
        try {
          if (target.name === "dora") {
            this._env.setDora(target.value);
          } else {
            this._env.setUradora(target.value);
          }
          target.setCustomValidity("");
          this._setOKMessage(messageElement);
        } catch (err) {
          target.setCustomValidity(err.toString());
          this._setErrorMessage(messageElement, err.toString());
        }
        break;
      }
      default:
        break;
    }
    this._update();
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
    document.getElementById("output"),
  );
}
