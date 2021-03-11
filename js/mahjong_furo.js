import "./mahjong_hai";

class MahjongFuro extends HTMLElement {
  static get observedAttributes() {
    return ["furo"];
  }

  constructor() {
    super();

    let shadow = this.attachShadow({ mode: "open" });
    let style = document.createElement("style");
    style.textContent = FURO_STYLE;
    let div = document.createElement("div");
    div.classList.add("furo");
    shadow.appendChild(style);
    shadow.appendChild(div);

    this.update();
  }

  get furo() {
    return this.getAttribute("furo");
  }

  set furo(value) {
    this.setAttribute("furo", value);
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.update();
    }
  }

  async update() {
    let shadow = this.shadowRoot;
    let elem = shadow.querySelector("div");
    let furo = this.getAttribute("furo");

    if (furo === "") {
      elem.textContent = "";
      return;
    }
    elem.textContent = "loading...";

    let res;
    try {
      let { Furo } = await import("../pkg/index.js");
      res = Furo.fromStr(furo);
    } catch (e) {
      elem.textContent = e;
      return;
    }
    elem.textContent = "";

    for (let hai of res.toImage()) {
      elem.appendChild(haiImage(hai));
    }
  }
}

function mjHai(hai, sideways) {
  let elem = document.createElement("mj-hai");
  elem.hai = hai;
  elem.sideways = sideways;
  return elem;
}

function haiImage(hai) {
  switch (hai.type) {
    case "normal":
      return mjHai(hai.hai, false);
    case "sideways":
      return mjHai(`${hai.hai}`, true);
    case "hidden":
      return mjHai("_", false);
    case "stack": {
      let top = mjHai(`${hai.top}`, true);
      top.classList.add("stack-top");

      let bottom = mjHai(`${hai.bottom}`, true);
      bottom.classList.add("stack-bottom");

      let stack = document.createElement("span");
      stack.classList.add("stack");
      stack.appendChild(top);
      stack.appendChild(bottom);
      return stack;
    }
  }
}

const FURO_STYLE = `
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

customElements.define("mj-furo", MahjongFuro);
