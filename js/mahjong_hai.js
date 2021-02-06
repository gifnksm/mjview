export const IMAGE_WIDTH = 64;
export const IMAGE_HEIGHT = 96;

class MahjongHai extends HTMLElement {
  static get observedAttributes() {
    return ["hai"];
  }

  constructor() {
    super();

    let shadow = this.attachShadow({ mode: "open" });
    let img = document.createElement("img");
    img.style.marginRight = "1px";
    img.style.verticalAlign = "bottom";
    shadow.appendChild(img);

    this.update();
  }

  get hai() {
    return this.getAttribute("hai");
  }

  set hai(value) {
    this.setAttribute("hai", value);
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.update();
    }
  }

  update() {
    let shadow = this.shadowRoot;
    let imgElem = shadow.querySelector("img");

    let hai = this.getAttribute("hai");
    imgElem.src = `image/paiga/${hai}.png`;
  }
}

customElements.define("mj-hai", MahjongHai);

function preloadHai(hai) {
  let link = document.createElement("link");
  link.rel = "prefetch";
  link.as = "image";
  link.href = `image/paiga/${hai}.png`;
  document.head.appendChild(link);
}

for (let cat of ["m", "p", "s"]) {
  for (let i = 1; i <= 9; i++) {
    preloadHai(`${i}${cat}`);
    preloadHai(`y_${i}${cat}`);
  }
}
for (let i = 1; i <= 7; i++) {
  preloadHai(`${i}j`);
  preloadHai(`y_${i}j`);
}
preloadHai("_");
