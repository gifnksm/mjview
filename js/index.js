import "./mahjong_hai";

let mod;
import("../pkg/index.js")
  .then((wasm) => {
    mod = wasm;
    let message = document.getElementById("message");
    document.forms[0].addEventListener("submit", (e) => {
      e.preventDefault();
      message.textContent = "";
      let form = e.currentTarget;
      try {
        let res = mod.parse_tehai(form.elements["tehai"].value);
        console.log(res);
      } catch (e) {
        message.textContent = e;
      }
    });
  })
  .catch(console.error);

// test
let addHai = (hai) => {
  let elem = document.createElement("mj-hai");
  elem.hai = hai;
  document.forms[0].appendChild(elem);
};
for (let prefix of ["", "y_"]) {
  for (let cat of ["m", "p", "s"]) {
    for (let i = 1; i <= 9; i++) {
      addHai(`${prefix}${i}${cat}`);
    }
  }
  addHai(`${prefix}5$m`);
  addHai(`${prefix}5$p`);
  addHai(`${prefix}5$s`);
  for (let i = 1; i <= 7; i++) {
    addHai(`${prefix}${i}j`);
  }
}
