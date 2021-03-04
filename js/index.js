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

    let maps = this._emitHaiCountWarning();

    if (this._env.ippatsu) {
      if (!this._env.richi && !this._env.daburi) {
        this._addWarningMessage(
          "guzen",
          "一発は立直/ダブル立直時のみ成立する役です",
        );
      }
      if (
        this._env.rinshan &&
        this._tehai !== null &&
        this._tehai.agariHai.agari == "!"
      ) {
        this._addWarningMessage("guzen", "一発は嶺上開花と複合しません");
      }
    }

    if (this._env.richi || this._env.daburi) {
      if (this._tehai !== null && !this._tehai.isMenzen) {
        this._addWarningMessage("richi", "立直/ダブル立直は門前時のみできます");
      }
    }

    if (this._env.daburi && this._env.ippatsu) {
      if (this._env.haitei) {
        this._addWarningMessage(
          "guzen",
          "ダブル立直の一発と海底/河底は複合しません",
        );
      }
      if (this._env.doraCount > 1) {
        this._addWarningMessage(
          "dora",
          "ダブル立直の一発の場合ドラ表示牌は必ず1枚です",
        );
      }
    }

    if (this._env.tenho) {
      if (this._tehai !== null && this._tehai.agariHai.agari !== "!") {
        this._addWarningMessage(
          "guzen",
          "天和/地和が成立するのはツモあがり時のみです",
        );
      }
      if (this._tehai !== null && this._tehai.furo.length > 0) {
        this._addWarningMessage(
          "guzen",
          "副露がある場合天和/地和にはなりません",
        );
      }
      if (this._env.richi || this._env.daburi) {
        this._addWarningMessage(
          "richi",
          "立直/ダブル立直は天和/地和と複合しません",
        );
      }
      if (this._env.rinshan) {
        this._addWarningMessage(
          "guzen",
          "嶺上開花/搶槓は天和/地和と複合しません",
        );
      }
      if (this._env.haitei) {
        this._addWarningMessage("guzen", "海底/河底は天和/地和と複合しません");
      }
    }

    if (
      this._env.rinshan &&
      this._tehai !== null &&
      this._tehai.agariHai.agari === "?"
    ) {
      let hai = this._tehai.agariHai.hai;
      if (maps.all.get(hai.toString()) > 1) {
        this._addWarningMessage(
          "tehai",
          "搶槓のあがり牌が純手牌/副露/ドラ表時牌/裏ドラ表示牌に含まれています",
        );
      }
    }

    if (this._env.doraCount == 0) {
      this._addWarningMessage("dora", "ドラ表示牌が0枚です");
    } else if (this._env.doraCount > 5) {
      this._addWarningMessage("dora", "ドラ表示牌が6枚以上あります");
    }

    if (this._env.richi || this._env.daburi) {
      if (this._env.doraCount != this._env.uradoraCount) {
        this._addWarningMessage(
          "uradora",
          "ドラ表示牌と裏ドラ表示牌の枚数が異なります",
        );
      }
      if (this._env.uradoraCount == 0) {
        this._addWarningMessage(
          "uradora",
          "立直/ダブル立直していますが裏ドラ表示牌が0枚です",
        );
      } else if (this._env.uradoraCount > 5) {
        this._addWarningMessage("uradora", "裏ドラ表示牌が6枚以上あります");
      }
    } else {
      if (this._env.uradoraCount > 0) {
        this._addWarningMessage(
          "uradora",
          "裏ドラが有効なのは立直/ダブル立直時のみです",
        );
      }
    }
  }

  _emitHaiCountWarning() {
    let maps = {
      all: new Map(),
      tehai: new Map(),
      dora: new Map(),
      uradora: new Map(),
    };
    let increment = (map, hai) => {
      let key = hai.toString();
      let count = map.get(key);
      if (count === undefined) {
        count = 0;
      }
      map.set(key, count + 1);
    };

    if (this._tehai !== null) {
      for (let hai of this._tehai.junTehai.toHaiArray()) {
        increment(maps.tehai, hai);
        increment(maps.all, hai);
      }
      for (let furo of this._tehai.furo) {
        for (let hai of furo.toHaiArray()) {
          increment(maps.tehai, hai);
          increment(maps.all, hai);
        }
      }
      increment(maps.tehai, this._tehai.agariHai.hai);
      increment(maps.all, this._tehai.agariHai.hai);
    }

    for (let name of ["dora", "uradora"]) {
      let map = maps[name];
      for (let hai of this._env[name]) {
        increment(map, hai);
        increment(maps.all, hai);
      }
    }
    for (let name of ["tehai", "dora", "uradora"]) {
      let myMap = maps[name];
      for (let [key, value] of myMap.entries()) {
        if (value > 4) {
          this._addWarningMessage(
            name,
            `\`${key}\` が5枚以上あります (${value}枚)`,
          );
          continue;
        }
        let allValue = maps.all.get(key);
        if (allValue > 4) {
          this._addWarningMessage(
            name,
            `手牌、ドラ表時牌、裏ドラ表示牌合わせて \`${key}\` が5枚以上あります (${allValue}枚)`,
          );
        }
      }
    }
    return maps;
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
