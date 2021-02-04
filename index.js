(()=>{"use strict";var e,t,r,n,o,a,i,s={},l={};function c(e){if(l[e])return l[e].exports;var t=l[e]={id:e,loaded:!1,exports:{}};return s[e](t,t.exports,c),t.loaded=!0,t.exports}c.m=s,e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},n=e=>!--e.r&&e(),o=(e,t)=>e?e.push(t):n(t),c.a=(a,i,s)=>{var l,c,u,p=s&&[],h=a.exports,d=!0,m=!1,f=(t,r,n)=>{m||(m=!0,r.r+=t.length,t.map(((t,o)=>{t[e](r,n)})),m=!1)},g=new Promise(((e,t)=>{u=t,c=()=>{e(h),r(p),p=0}}));g[t]=h,g[e]=(e,t)=>{if(d)return n(e);l&&f(l,e,t),o(p,e),g.catch(t)},a.exports=g,i((a=>{if(!a)return c();var i,s;l=(a=>a.map((a=>{if(null!==a&&"object"==typeof a){if(a[e])return a;if(a.then){var i=[];a.then((e=>{s[t]=e,r(i),i=0}));var s={[e]:(e,t)=>{o(i,e),a.catch(t)}};return s}}return{[e]:e=>{n(e)},[t]:a}})))(a);var u=new Promise(((e,r)=>{(i=()=>e(s=l.map((e=>e[t])))).r=0,f(l,i,r)}));return i.r?u:s})).then(c,u),d=!1},c.d=(e,t)=>{for(var r in t)c.o(t,r)&&!c.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},c.f={},c.e=e=>Promise.all(Object.keys(c.f).reduce(((t,r)=>(c.f[r](e,t),t)),[])),c.u=e=>e+".js",c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),a={},i="mjview:",c.l=(e,t,r,n)=>{if(a[e])a[e].push(t);else{var o,s;if(void 0!==r)for(var l=document.getElementsByTagName("script"),u=0;u<l.length;u++){var p=l[u];if(p.getAttribute("src")==e||p.getAttribute("data-webpack")==i+r){o=p;break}}o||(s=!0,(o=document.createElement("script")).charset="utf-8",o.timeout=120,c.nc&&o.setAttribute("nonce",c.nc),o.setAttribute("data-webpack",i+r),o.src=e),a[e]=[t];var h=(t,r)=>{o.onerror=o.onload=null,clearTimeout(d);var n=a[e];if(delete a[e],o.parentNode&&o.parentNode.removeChild(o),n&&n.forEach((e=>e(r))),t)return t(r)},d=setTimeout(h.bind(null,void 0,{type:"timeout",target:o}),12e4);o.onerror=h.bind(null,o.onerror),o.onload=h.bind(null,o.onload),s&&document.head.appendChild(o)}},c.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var t=c.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");r.length&&(e=r[r.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),(()=>{var e={826:0};c.f.j=(t,r)=>{var n=c.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var o=new Promise(((r,o)=>{n=e[t]=[r,o]}));r.push(n[2]=o);var a=c.p+c.u(t),i=new Error;c.l(a,(r=>{if(c.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var o=r&&("load"===r.type?"missing":r.type),a=r&&r.target&&r.target.src;i.message="Loading chunk "+t+" failed.\n("+o+": "+a+")",i.name="ChunkLoadError",i.type=o,i.request=a,n[1](i)}}),"chunk-"+t,t)}};var t=(t,r)=>{for(var n,o,[a,i,s]=r,l=0,u=[];l<a.length;l++)o=a[l],c.o(e,o)&&e[o]&&u.push(e[o][0]),e[o]=0;for(n in i)c.o(i,n)&&(c.m[n]=i[n]);for(s&&s(c),t&&t(r);u.length;)u.shift()()},r=self.webpackChunkmjview=self.webpackChunkmjview||[];r.forEach(t.bind(null,0)),r.push=t.bind(null,r.push.bind(r))})(),c.v=(e,t,r,n)=>{var o=fetch(c.p+""+r+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,n).then((t=>Object.assign(e,t.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)))};class u extends HTMLElement{static get observedAttributes(){return["hai"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("img");t.style.marginRight="1px",e.appendChild(t),this.update()}get hai(){return this.getAttribute("hai")}set hai(e){this.setAttribute("hai",e)}attributeChangedCallback(e,t,r){t!==r&&this.update()}update(){let e=this.shadowRoot.querySelector("img"),t=this.getAttribute("hai");e.src=`image/paiga/${t}.png`}}let p;customElements.define("mj-hai",u),c.e(235).then(c.bind(c,235)).then((e=>{p=e;let t=document.getElementById("message");document.forms[0].addEventListener("submit",(e=>{e.preventDefault(),t.textContent="";let r=e.currentTarget;try{let e=p.parse_tehai(r.elements.tehai.value);console.log(e),console.log(e.junTehai.toImage().toString()),console.log(e.furo.map((e=>`{${e.toImage()}}`)).toString()),console.log(e.agari.toImage().toString())}catch(e){t.textContent=e}}))})).catch(console.error);let h=e=>{let t=document.createElement("mj-hai");t.hai=e,document.forms[0].appendChild(t)};for(let e of["","y_"]){for(let t of["m","p","s"])for(let r=1;r<=9;r++)h(`${e}${r}${t}`);h(`${e}5$m`),h(`${e}5$p`),h(`${e}5$s`);for(let t=1;t<=7;t++)h(`${e}${t}j`)}})();