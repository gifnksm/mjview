export const IMAGE_WIDTH = 64;
export const IMAGE_HEIGHT = 96;

class MahjongHai extends HTMLElement {
  static get observedAttributes() {
    return ["hai", "sideways"];
  }

  constructor() {
    super();

    let shadow = this.attachShadow({ mode: "open" });
    let style = document.createElement("style");
    style.textContent = HAI_STYLE;
    let container = document.createElement("div");
    container.classList.add("container");
    let tile = document.createElement("div");
    tile.classList.add("tile");
    let img = document.createElement("img");
    tile.appendChild(img);
    container.appendChild(tile);
    shadow.appendChild(style);
    shadow.appendChild(container);

    this.update();
  }

  get hai() {
    return this.getAttribute("hai");
  }

  set hai(value) {
    this.setAttribute("hai", value);
  }

  get sideways() {
    return this.hasAttribute("sideways");
  }

  set sideways(value) {
    if (value) {
      this.setAttribute("sideways", "sideways");
    } else {
      this.removeAttribute("sideways");
    }
  }

  attributeChangedCallback(_name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.update();
    }
  }

  update() {
    let shadow = this.shadowRoot;
    let hai = this.getAttribute("hai");
    let sideways = this.hasAttribute("sideways");

    let containerElem = shadow.querySelector(".container");
    let tileElem = shadow.querySelector(".tile");
    let imgElem = shadow.querySelector("img");

    containerElem.classList.toggle("sideways", sideways);
    tileElem.classList.toggle("back", hai === "_");

    if (hai !== null) {
      imgElem.src = `/mjview/image/paiga/${hai}.svg`;
    } else {
      imgElem.src = null;
    }
    imgElem.alt = hai;
    imgElem.title = hai;
  }
}

function prefetchHai(hai) {
  let link = document.createElement("link");
  link.rel = "prefetch";
  link.as = "image";
  link.href = `/mjview/image/paiga/${hai}.svg`;
  document.head.appendChild(link);
}

for (let cat of ["m", "p", "s"]) {
  for (let i = 1; i <= 9; i++) {
    prefetchHai(`${i}${cat}`);
  }
}
for (let i = 1; i <= 7; i++) {
  prefetchHai(`${i}j`);
}
prefetchHai("_");

const HAI_STYLE = `
:host {
  --hai-width: 30px;
  --hai-height: calc(var(--hai-width) / 3 * 4);
}
.container {
  display: inline-block;
  vertical-align: bottom;
  position: relative;
  width: var(--hai-width);
  height: var(--hai-height);
}
.container.sideways {
  width: var(--hai-height);
  height: var(--hai-width);
  text-align: center;
  transform: rotate(-90deg) translateY(calc((var(--hai-width) - var(--hai-height)) / 2));
}
.tile {
  width: var(--hai-width);
  height: var(--hai-height);
  object-fit: contain;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0;
  border: 1px solid gray;
  padding: 5%;
  background-color: #f5f0eb;
  border-radius: 8%;
  box-sizing: border-box;
  display: inline-block;
  line-height: 1;
}
.tile img {
  width: 100%;
  height: auto;
}
.tile.back {
  background-color: orange;
}
.tile.back img {
  visibility: hidden;
}
`;

customElements.define("mj-hai", MahjongHai);
