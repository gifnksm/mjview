(()=>{"use strict";var e,t,n,a,i,r,s,o={},l={};function d(e){if(l[e])return l[e].exports;var t=l[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,d),t.loaded=!0,t.exports}d.m=o,e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},a=e=>!--e.r&&e(),i=(e,t)=>e?e.push(t):a(t),d.a=(r,s,o)=>{var l,d,h,c=o&&[],u=r.exports,m=!0,p=!1,g=(t,n,a)=>{p||(p=!0,n.r+=t.length,t.map(((t,i)=>{t[e](n,a)})),p=!1)},f=new Promise(((e,t)=>{h=t,d=()=>{e(u),n(c),c=0}}));f[t]=u,f[e]=(e,t)=>{if(m)return a(e);l&&g(l,e,t),i(c,e),f.catch(t)},r.exports=f,s((r=>{if(!r)return d();var s,o;l=(r=>r.map((r=>{if(null!==r&&"object"==typeof r){if(r[e])return r;if(r.then){var s=[];r.then((e=>{o[t]=e,n(s),s=0}));var o={[e]:(e,t)=>{i(s,e),r.catch(t)}};return o}}return{[e]:e=>{a(e)},[t]:r}})))(r);var h=new Promise(((e,n)=>{(s=()=>e(o=l.map((e=>e[t])))).r=0,g(l,s,n)}));return s.r?h:o})).then(d,h),m=!1},d.d=(e,t)=>{for(var n in t)d.o(t,n)&&!d.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},d.f={},d.e=e=>Promise.all(Object.keys(d.f).reduce(((t,n)=>(d.f[n](e,t),t)),[])),d.u=e=>e+".js",d.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),d.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),d.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),r={},s="mjview:",d.l=(e,t,n,a)=>{if(r[e])r[e].push(t);else{var i,o;if(void 0!==n)for(var l=document.getElementsByTagName("script"),h=0;h<l.length;h++){var c=l[h];if(c.getAttribute("src")==e||c.getAttribute("data-webpack")==s+n){i=c;break}}i||(o=!0,(i=document.createElement("script")).charset="utf-8",i.timeout=120,d.nc&&i.setAttribute("nonce",d.nc),i.setAttribute("data-webpack",s+n),i.src=e),r[e]=[t];var u=(t,n)=>{i.onerror=i.onload=null,clearTimeout(m);var a=r[e];if(delete r[e],i.parentNode&&i.parentNode.removeChild(i),a&&a.forEach((e=>e(n))),t)return t(n)},m=setTimeout(u.bind(null,void 0,{type:"timeout",target:i}),12e4);i.onerror=u.bind(null,i.onerror),i.onload=u.bind(null,i.onload),o&&document.head.appendChild(i)}},d.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;d.g.importScripts&&(e=d.g.location+"");var t=d.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");n.length&&(e=n[n.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),d.p=e})(),(()=>{var e={826:0};d.f.j=(t,n)=>{var a=d.o(e,t)?e[t]:void 0;if(0!==a)if(a)n.push(a[2]);else{var i=new Promise(((n,i)=>{a=e[t]=[n,i]}));n.push(a[2]=i);var r=d.p+d.u(t),s=new Error;d.l(r,(n=>{if(d.o(e,t)&&(0!==(a=e[t])&&(e[t]=void 0),a)){var i=n&&("load"===n.type?"missing":n.type),r=n&&n.target&&n.target.src;s.message="Loading chunk "+t+" failed.\n("+i+": "+r+")",s.name="ChunkLoadError",s.type=i,s.request=r,a[1](s)}}),"chunk-"+t,t)}};var t=(t,n)=>{for(var a,i,[r,s,o]=n,l=0,h=[];l<r.length;l++)i=r[l],d.o(e,i)&&e[i]&&h.push(e[i][0]),e[i]=0;for(a in s)d.o(s,a)&&(d.m[a]=s[a]);for(o&&o(d),t&&t(n);h.length;)h.shift()()},n=self.webpackChunkmjview=self.webpackChunkmjview||[];n.forEach(t.bind(null,0)),n.push=t.bind(null,n.push.bind(n))})(),d.v=(e,t,n,a)=>{var i=fetch(d.p+""+n+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(i,a).then((t=>Object.assign(e,t.instance.exports))):i.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,a))).then((t=>Object.assign(e,t.instance.exports)))};class h extends HTMLElement{static get observedAttributes(){return["hai","sideways"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("style");t.textContent=u;let n=document.createElement("div");n.classList.add("container");let a=document.createElement("div");a.classList.add("tile");let i=document.createElement("img");a.appendChild(i),n.appendChild(a),e.appendChild(t),e.appendChild(n),this.update()}get hai(){return this.getAttribute("hai")}set hai(e){this.setAttribute("hai",e)}get sideways(){return this.hasAttribute("sideways")}set sideways(e){e?this.setAttribute("sideways","sideways"):this.removeAttribute("sideways")}attributeChangedCallback(e,t,n){t!==n&&this.update()}update(){let e=this.shadowRoot,t=this.getAttribute("hai"),n=this.hasAttribute("sideways"),a=e.querySelector(".container"),i=e.querySelector(".tile"),r=e.querySelector("img");a.classList.toggle("sideways",n),i.classList.toggle("back","_"===t),r.src=null!==t?`/mjview/image/paiga/${t}.svg`:null,r.alt=t,r.title=t}}function c(e){let t=document.createElement("link");t.rel="prefetch",t.as="image",t.href=`/mjview/image/paiga/${e}.svg`,document.head.appendChild(t)}for(let e of["m","p","s"])for(let t=1;t<=9;t++)c(`${t}${e}`);for(let e=1;e<=7;e++)c(`${e}j`);c("_");const u="\n:host {\n  --hai-width: 30px;\n  --hai-height: calc(var(--hai-width) / 3 * 4);\n}\n.container {\n  display: inline-block;\n  vertical-align: bottom;\n  position: relative;\n  width: var(--hai-width);\n  height: var(--hai-height);\n}\n.container.sideways {\n  width: var(--hai-height);\n  height: var(--hai-width);\n  text-align: center;\n  transform: rotate(-90deg) translateY(calc((var(--hai-width) - var(--hai-height)) / 2));\n}\n.tile {\n  width: var(--hai-width);\n  height: var(--hai-height);\n  object-fit: contain;\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  margin: 0;\n  border: 1px solid gray;\n  padding: 5%;\n  background-color: #f5f0eb;\n  border-radius: 8%;\n  box-sizing: border-box;\n  display: inline-block;\n  line-height: 1;\n}\n.tile img {\n  width: 100%;\n  height: auto;\n}\n.tile.back {\n  background-color: orange;\n}\n.tile.back img {\n  visibility: hidden;\n}\n";customElements.define("mj-hai",h);class m extends HTMLElement{static get observedAttributes(){return["tehai"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("style");t.textContent=f;let n=document.createElement("div");n.classList.add("tehai"),e.appendChild(t),e.appendChild(n),this.update()}get tehai(){return this.getAttribute("tehai")}set tehai(e){this.setAttribute("tehai",e)}attributeChangedCallback(e,t,n){t!==n&&this.update()}async update(){let e,t=this.shadowRoot.querySelector("div"),n=this.getAttribute("tehai");if(""===n)return void(t.textContent="");t.textContent="loading...";try{let{Tehai:t}=await d.e(235).then(d.bind(d,235));e=t.fromStr(n)}catch(e){return void(t.textContent=e)}t.textContent="";let a=document.createElement("div");a.classList.add("jun-tehai");for(let t of e.junTehai.toImage())a.appendChild(g(t));t.appendChild(a);for(let n of e.furo){let e=document.createElement("mj-furo");e.furo=n,t.appendChild(e)}let i=document.createElement("div");switch(i.classList.add("agari-hai"),e.agariHai.agari){case"!":i.classList.add("tsumo");break;case"?":i.classList.add("ron")}i.appendChild(g(e.agariHai.toImage())),t.appendChild(i)}}function p(e){let t=document.createElement("mj-hai");return t.hai=e,t}function g(e){switch(e.type){case"normal":return p(e.hai);case"sideways":return p(`y_${e.hai}`);case"hidden":return p("_");case"stack":{let t=p(`y_${e.top}`);t.classList.add("stack-top");let n=p(`y_${e.bottom}`);n.classList.add("stack-bottom");let a=document.createElement("span");return a.classList.add("stack"),a.appendChild(t),a.appendChild(n),a}}}const f='\n.tehai {\n  display: table-cell;\n  width: 800px;\n  height: 60px;\n  padding: 5px;\n  background-color: green;\n  color: white;\n  line-height: 1;\n}\nmj-furo,\n.jun-tehai,\n.agari-hai {\n  display: table-cell;\n  line-height: 60px;\n}\n.jun-tehai {\n  float: left;\n  margin-right: 5px;\n}\nmj-furo {\n  float: right;\n  margin-left: 5px;\n}\n.agari-hai::before {\n  display: inline-block;\n  width: 1em;\n  line-height: 1;\n  font-size: 15px;\n  text-align: center;\n  vertical-align: bottom;\n}\n.agari-hai {\n  margin-right: 20px;\n}\n.ron::before {\n  content: "ロン";\n}\n.tsumo::before {\n  content: "ツモ";\n}\n';customElements.define("mj-tehai",m);class b extends HTMLElement{static get observedAttributes(){return["furo"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("style");t.textContent=w;let n=document.createElement("div");n.classList.add("furo"),e.appendChild(t),e.appendChild(n),this.update()}get furo(){return this.getAttribute("furo")}set furo(e){this.setAttribute("furo",e)}attributeChangedCallback(e,t,n){t!==n&&this.update()}async update(){let e,t=this.shadowRoot.querySelector("div"),n=this.getAttribute("furo");if(""!==n){t.textContent="loading...";try{let{Furo:t}=await d.e(235).then(d.bind(d,235));e=t.fromStr(n)}catch(e){return void(t.textContent=e)}t.textContent="";for(let n of e.toImage())t.appendChild(y(n))}else t.textContent=""}}function v(e,t){let n=document.createElement("mj-hai");return n.hai=e,n.sideways=t,n}function y(e){switch(e.type){case"normal":return v(e.hai,!1);case"sideways":return v(`${e.hai}`,!0);case"hidden":return v("_",!1);case"stack":{let t=v(`${e.top}`,!0);t.classList.add("stack-top");let n=v(`${e.bottom}`,!0);n.classList.add("stack-bottom");let a=document.createElement("span");return a.classList.add("stack"),a.appendChild(t),a.appendChild(n),a}}}const w="\n.stack {\n  display: inline-block;\n  line-height: 1;\n  text-align: center;\n  vertical-align: bottom;\n}\n.stack .stack-top,\n.stack .stack-bottom {\n  display: table-row;\n}\n";customElements.define("mj-furo",b);class _{constructor(e,t,n,a){this._wasmMod=e,this._form=t,this._tehaiElement=n,this._outputElement=a,this._messageElementMap=new Map,this._messageElements=new Set;for(let e of t.elements){let t=e.name,n=e.parentNode;for(;"TD"!=n.nodeName;)n=n.parentNode;let a=n.querySelector("ul:last-child[id$=-message]");this._messageElementMap.set(t,a),null!==a&&this._messageElements.add(a)}this._tehai=null;let{Env:i}=e;this._env=new i;for(let[e,t]of new URLSearchParams(location.search)){let n=this._form[e];"checkbox"==n.type?n.checked=!0:n.value=t}for(let e of t.elements)this._onChange(e),this._onInput(e);t.addEventListener("submit",(e=>{e.preventDefault(),this._update()})),t.addEventListener("change",(e=>this._onChange(e.target))),t.addEventListener("input",(e=>this._onInput(e.target)))}_update(){this._outputElement.textContent="",this._updateWarning();let e=this._tehai;if(null===e||null!==this._form.querySelector(".error"))return;let t=e.toAgariCombinations().map((e=>[e,e.judgeYaku(this._env)])).sort((([e,t],[n,a])=>{let i=t.compare(a);return 0!==i?-i:e.compare(n)})),n=document.createElement("dl");for(let[e,a]of t){let t=document.createElement("dt");t.textContent=`${e} (${a.name}${a.point}点 ${a.rank} ${a.fu}符)`,n.appendChild(t);let i=document.createElement("dd"),r=document.createElement("ul");for(let[e,t]of a.detail){let n=document.createElement("li");n.textContent=`${e} (${t})`,r.appendChild(n)}i.appendChild(r),n.appendChild(i)}this._outputElement.appendChild(n);let a=new FormData(this._form),i=new URLSearchParams(a);history.replaceState(null,null,`?${i}`)}_updateWarning(){for(let e of this._messageElements)this._clearWarning(e);let e;e=null!==this._tehai?this._env.checkPropsWithTehai(this._tehai):this._env.checkPropsWithoutTehai();for(let[t,n]of e){let e=new Set;for(let n of t){let t=this._messageElementMap.get(n);e.add(t)}for(let t of e.values())this._addWarningMessage(t,n)}}_onChange(e){let{Hai:t}=this._wasmMod;switch(e.name){case"richi":e.checked&&(this._env[e.name]=e.value);break;case"ippatsu":case"rinshan":case"haitei":case"tenho":case"aotenjo":this._env[e.name]=e.checked;break;case"bakaze":case"jikaze":this._env[e.name]=t.fromStr(e.value)}this._update()}_addMessage(e,t,n,a){e.classList.contains(t)||(e.textContent=""),e.classList.remove("ok"),e.classList.remove("warning"),e.classList.remove("error"),e.classList.add(t),a&&(e.textContent="");let i=document.createElement("li");i.textContent=n,e.appendChild(i)}_setOKMessage(e){this._addMessage(e,"ok","OK",!0)}_clearWarning(e){e.classList.contains("warning")&&(e.classList.remove("warning"),e.textContent=""),e.classList.contains("error")||this._setOKMessage(e)}_addWarningMessage(e,t){e.classList.contains("error")||this._addMessage(e,"warning",t,!1)}_setErrorMessage(e,t){this._clearWarning(e),this._addMessage(e,"error",t,!0)}_onInput(e){switch(e.name){case"tehai":{let t=e.value,n=this._messageElementMap.get(e.name);this._tehaiElement.tehai=t;let{Tehai:a}=this._wasmMod;try{this._tehai=a.fromStr(t),e.setCustomValidity(""),this._setOKMessage(n)}catch(t){this._tehai=null,e.setCustomValidity(t.toString()),this._setErrorMessage(n,t.toString())}break}case"dora":case"uradora":{let t=this._messageElementMap.get(e.name);try{"dora"===e.name?this._env.setDora(e.value):this._env.setUradora(e.value),e.setCustomValidity(""),this._setOKMessage(t)}catch(n){e.setCustomValidity(n.toString()),this._setErrorMessage(t,n.toString())}break}}this._update()}}document.addEventListener("DOMContentLoaded",(e=>{!async function(){let e=await d.e(235).then(d.bind(d,235));new _(e,document.forms[0],document.getElementById("tehai-view"),document.getElementById("output"))}()}))})();