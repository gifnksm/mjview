class MahjongTehai extends HTMLElement {
  static get observedAttributes() {
    return ["tehai"];
  }

  constructor() {
    super();

    let shadow = this.attachShadow({ mode: "open" });
    let style = document.createElement("style");
    style.textContent = STYLE;
    let div = document.createElement("div");
    div.classList.add("tehai");
    shadow.appendChild(style);
    shadow.appendChild(div);

    this.update();
  }

  get tehai() {
    return this.getAttribute("tehai");
  }

  set tehai(value) {
    this.setAttribute("tehai", value);
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.update();
    }
  }

  async update() {
    let shadow = this.shadowRoot;
    let elem = shadow.querySelector("div");
    let tehai = this.getAttribute("tehai");

    if (tehai === "") {
      elem.textContent = "";
      return;
    }

    let res;
    try {
      let mod = await import("../pkg/index.js");
      res = mod.parse_tehai(tehai);
    } catch (e) {
      elem.textContent = e;
      return;
    }
    elem.textContent = "";

    let junTehaiElem = document.createElement("div");
    junTehaiElem.classList.add("jun-tehai");
    for (let hai of res.junTehai.toImage()) {
      junTehaiElem.appendChild(haiImage(hai));
    }
    elem.appendChild(junTehaiElem);

    for (let furo of res.furo) {
      let furoElem = document.createElement("div");
      furoElem.classList.add("furo");
      for (let hai of furo.toImage()) {
        furoElem.appendChild(haiImage(hai));
      }
      elem.appendChild(furoElem);
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
    elem.appendChild(agariHaiElem);
  }
}

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

const STYLE = `
.tehai {
  display: table-cell;
  width: 500px;
  height: 50px;
  padding: 5px;
  background-color: green;
  color: white;
  line-height: 1;
}
.jun-tehai,
.furo,
.agari-hai {
  display: table-cell;
  line-height: 50px;
}
.jun-tehai {
  float: left;
  margin-right: 5px;
}
.furo {
  float: right;
  margin-left: 5px;
}
.agari-hai::before {
  display: inline-block;
  width: 1em;
  line-height: 1;
  font-size: 15px;
  text-align: center;
  vertical-align: bottom;
}
.ron::before {
  content: "ロン";
}
.tsumo::before {
  content: "ツモ";
}
.stack {
  display: inline-block;
  line-height: 1;
  text-align: center;
  vertical-align: bottom;
}
.stack .stack-top,
.stack .stack-bottom {
  display: table-row;
}
`;

customElements.define("mj-tehai", MahjongTehai);