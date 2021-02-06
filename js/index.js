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
        console.log(res.junTehai.toImage().toString());
        console.log(res.furo.map((f) => `{${f.toImage()}}`).toString());
        console.log(res.agari.toImage().toString());
      } catch (e) {
        message.textContent = e;
      }
    });
  })
  .catch(console.error);
