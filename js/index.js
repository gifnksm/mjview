let imp = import("../pkg/index.js");
let mod;
imp
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
