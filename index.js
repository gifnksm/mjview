(()=>{"use strict";var e,t,a,n,i,r,s,o={},l={};function h(e){if(l[e])return l[e].exports;var t=l[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,h),t.loaded=!0,t.exports}h.m=o,e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",a=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},n=e=>!--e.r&&e(),i=(e,t)=>e?e.push(t):n(t),h.a=(r,s,o)=>{var l,h,d,c=o&&[],u=r.exports,m=!0,p=!1,f=(t,a,n)=>{p||(p=!0,a.r+=t.length,t.map(((t,i)=>{t[e](a,n)})),p=!1)},g=new Promise(((e,t)=>{d=t,h=()=>{e(u),a(c),c=0}}));g[t]=u,g[e]=(e,t)=>{if(m)return n(e);l&&f(l,e,t),i(c,e),g.catch(t)},r.exports=g,s((r=>{if(!r)return h();var s,o;l=(r=>r.map((r=>{if(null!==r&&"object"==typeof r){if(r[e])return r;if(r.then){var s=[];r.then((e=>{o[t]=e,a(s),s=0}));var o={[e]:(e,t)=>{i(s,e),r.catch(t)}};return o}}return{[e]:e=>{n(e)},[t]:r}})))(r);var d=new Promise(((e,a)=>{(s=()=>e(o=l.map((e=>e[t])))).r=0,f(l,s,a)}));return s.r?d:o})).then(h,d),m=!1},h.d=(e,t)=>{for(var a in t)h.o(t,a)&&!h.o(e,a)&&Object.defineProperty(e,a,{enumerable:!0,get:t[a]})},h.f={},h.e=e=>Promise.all(Object.keys(h.f).reduce(((t,a)=>(h.f[a](e,t),t)),[])),h.u=e=>e+".js",h.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),h.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),h.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),r={},s="mjview:",h.l=(e,t,a,n)=>{if(r[e])r[e].push(t);else{var i,o;if(void 0!==a)for(var l=document.getElementsByTagName("script"),d=0;d<l.length;d++){var c=l[d];if(c.getAttribute("src")==e||c.getAttribute("data-webpack")==s+a){i=c;break}}i||(o=!0,(i=document.createElement("script")).charset="utf-8",i.timeout=120,h.nc&&i.setAttribute("nonce",h.nc),i.setAttribute("data-webpack",s+a),i.src=e),r[e]=[t];var u=(t,a)=>{i.onerror=i.onload=null,clearTimeout(m);var n=r[e];if(delete r[e],i.parentNode&&i.parentNode.removeChild(i),n&&n.forEach((e=>e(a))),t)return t(a)},m=setTimeout(u.bind(null,void 0,{type:"timeout",target:i}),12e4);i.onerror=u.bind(null,i.onerror),i.onload=u.bind(null,i.onload),o&&document.head.appendChild(i)}},h.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;h.g.importScripts&&(e=h.g.location+"");var t=h.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var a=t.getElementsByTagName("script");a.length&&(e=a[a.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),h.p=e})(),(()=>{var e={826:0};h.f.j=(t,a)=>{var n=h.o(e,t)?e[t]:void 0;if(0!==n)if(n)a.push(n[2]);else{var i=new Promise(((a,i)=>{n=e[t]=[a,i]}));a.push(n[2]=i);var r=h.p+h.u(t),s=new Error;h.l(r,(a=>{if(h.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var i=a&&("load"===a.type?"missing":a.type),r=a&&a.target&&a.target.src;s.message="Loading chunk "+t+" failed.\n("+i+": "+r+")",s.name="ChunkLoadError",s.type=i,s.request=r,n[1](s)}}),"chunk-"+t,t)}};var t=(t,a)=>{for(var n,i,[r,s,o]=a,l=0,d=[];l<r.length;l++)i=r[l],h.o(e,i)&&e[i]&&d.push(e[i][0]),e[i]=0;for(n in s)h.o(s,n)&&(h.m[n]=s[n]);for(o&&o(h),t&&t(a);d.length;)d.shift()()},a=self.webpackChunkmjview=self.webpackChunkmjview||[];a.forEach(t.bind(null,0)),a.push=t.bind(null,a.push.bind(a))})(),h.v=(e,t,a,n)=>{var i=fetch(h.p+""+a+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(i,n).then((t=>Object.assign(e,t.instance.exports))):i.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)))};class d extends HTMLElement{static get observedAttributes(){return["hai"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("img");t.style.marginRight="1px",t.style.verticalAlign="bottom",t.style.color="red",e.appendChild(t),this.update()}get hai(){return this.getAttribute("hai")}set hai(e){this.setAttribute("hai",e)}attributeChangedCallback(e,t,a){t!==a&&this.update()}update(){let e=this.shadowRoot.querySelector("img"),t=this.getAttribute("hai");e.src=`image/paiga/${t}.png`,e.alt=t,e.title=t}}function c(e){let t=document.createElement("link");t.rel="prefetch",t.as="image",t.href=`/image/paiga/${e}.png`,document.head.appendChild(t)}customElements.define("mj-hai",d);for(let e of["m","p","s"])for(let t=1;t<=9;t++)c(`${t}${e}`),c(`y_${t}${e}`);for(let e=1;e<=7;e++)c(`${e}j`),c(`y_${e}j`);c("_");class u extends HTMLElement{static get observedAttributes(){return["tehai"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("style");t.textContent=f;let a=document.createElement("div");a.classList.add("tehai"),e.appendChild(t),e.appendChild(a),this.update()}get tehai(){return this.getAttribute("tehai")}set tehai(e){this.setAttribute("tehai",e)}attributeChangedCallback(e,t,a){t!==a&&this.update()}async update(){let e,t=this.shadowRoot.querySelector("div"),a=this.getAttribute("tehai");if(""===a)return void(t.textContent="");t.textContent="loading...";try{let{Tehai:t}=await h.e(235).then(h.bind(h,235));e=t.fromStr(a)}catch(e){return void(t.textContent=e)}t.textContent="";let n=document.createElement("div");n.classList.add("jun-tehai");for(let t of e.junTehai.toImage())n.appendChild(p(t));t.appendChild(n);for(let a of e.furo){let e=document.createElement("mj-furo");e.furo=a,t.appendChild(e)}let i=document.createElement("div");switch(i.classList.add("agari-hai"),e.agariHai.agari){case"!":i.classList.add("tsumo");break;case"?":i.classList.add("ron")}i.appendChild(p(e.agariHai.toImage())),t.appendChild(i)}}function m(e){let t=document.createElement("mj-hai");return t.hai=e,t}function p(e){switch(e.type){case"normal":return m(e.hai);case"sideways":return m(`y_${e.hai}`);case"hidden":return m("_");case"stack":{let t=m(`y_${e.top}`);t.classList.add("stack-top");let a=m(`y_${e.bottom}`);a.classList.add("stack-bottom");let n=document.createElement("span");return n.classList.add("stack"),n.appendChild(t),n.appendChild(a),n}}}const f='\n.tehai {\n  display: table-cell;\n  width: 500px;\n  height: 44px;\n  padding: 5px;\n  background-color: green;\n  color: white;\n  line-height: 1;\n}\nmj-furo,\n.jun-tehai,\n.agari-hai {\n  display: table-cell;\n  line-height: 44px;\n}\n.jun-tehai {\n  float: left;\n  margin-right: 5px;\n}\nmj-furo {\n  float: right;\n  margin-left: 5px;\n}\n.agari-hai::before {\n  display: inline-block;\n  width: 1em;\n  line-height: 1;\n  font-size: 15px;\n  text-align: center;\n  vertical-align: bottom;\n}\n.agari-hai {\n  margin-right: 20px;\n}\n.ron::before {\n  content: "ロン";\n}\n.tsumo::before {\n  content: "ツモ";\n}\n';customElements.define("mj-tehai",u);class g extends HTMLElement{static get observedAttributes(){return["furo"]}constructor(){super();let e=this.attachShadow({mode:"open"}),t=document.createElement("style");t.textContent=_;let a=document.createElement("div");a.classList.add("furo"),e.appendChild(t),e.appendChild(a),this.update()}get furo(){return this.getAttribute("furo")}set furo(e){this.setAttribute("furo",e)}attributeChangedCallback(e,t,a){t!==a&&this.update()}async update(){let e,t=this.shadowRoot.querySelector("div"),a=this.getAttribute("furo");if(""!==a){t.textContent="loading...";try{let{Furo:t}=await h.e(235).then(h.bind(h,235));e=t.fromStr(a)}catch(e){return void(t.textContent=e)}t.textContent="";for(let a of e.toImage())t.appendChild(v(a))}else t.textContent=""}}function b(e){let t=document.createElement("mj-hai");return t.hai=e,t}function v(e){switch(e.type){case"normal":return b(e.hai);case"sideways":return b(`y_${e.hai}`);case"hidden":return b("_");case"stack":{let t=b(`y_${e.top}`);t.classList.add("stack-top");let a=b(`y_${e.bottom}`);a.classList.add("stack-bottom");let n=document.createElement("span");return n.classList.add("stack"),n.appendChild(t),n.appendChild(a),n}}}const _="\n.stack {\n  display: inline-block;\n  line-height: 1;\n  text-align: center;\n  vertical-align: bottom;\n}\n.stack .stack-top,\n.stack .stack-bottom {\n  display: table-row;\n}\n";customElements.define("mj-furo",g);class y{constructor(e,t,a,n){this._wasmMod=e,this._form=t,this._tehaiElement=a,this._messageElement=n,this._tehai=null;let{Env:i}=e;this._env=new i,this._doraValid=!1,this._uradoraValid=!1;for(let e of t.elements)this._onChange(e),this._onInput(e);t.addEventListener("submit",(e=>{e.preventDefault(),this._update()})),t.addEventListener("change",(e=>this._onChange(e.target))),t.addEventListener("input",(e=>this._onInput(e.target)))}get tehai(){return this._tehai}get env(){return this._env}_update(){this._messageElement.textContent="";let e=this._tehai;if(null===e||!this._doraValid||!this._uradoraValid)return;let t=e.toAgariCombinations().map((e=>[e,e.judgeYaku(this._env)])).sort((([e,t],[a,n])=>{let i=t.compare(n);return 0!==i?-i:e.compare(a)}));for(let[e,a]of t){let t=document.createElement("dl"),n=document.createElement("dt");n.textContent=`${e} (${a.name}${a.point}点 ${a.rank} ${a.fu}符)`,t.appendChild(n);let i=document.createElement("dd"),r=document.createElement("ul");for(let[e,t]of a.detail){let a=document.createElement("li");a.textContent=`${e} (${t})`,r.appendChild(a)}i.appendChild(r),t.appendChild(i),this._messageElement.appendChild(t)}}_onChange(e){let{Hai:t}=this._wasmMod;switch(e.name){case"tenho":e.checked&&this._setTenho(e.value);break;case"richi":e.checked&&this._setRichi(e.value);break;case"ippatsu":case"rinshan":case"haitei":this._env[e.name]=e.checked;break;case"bakaze":case"jikaze":this._env[e.name]=t.fromStr(e.value)}this._update()}_onInput(e){switch(e.name){case"tehai":{let{Tehai:t}=this._wasmMod;try{let a=e.value;this._tehai=t.fromStr(a),this._tehaiElement.tehai=a,e.setCustomValidity("")}catch(t){this._tehai=null,this._tehaiElement.tehai="",e.setCustomValidity(t.toString())}break}case"dora":case"uradora":try{"dora"===e.name?(this._env.setDora(e.value),this._doraValid=!0):(this._env.setUradora(e.value),this._uradoraValid=!0),e.setCustomValidity("")}catch(t){e.setCustomValidity(t.toString()),"dora"==e.name&&(this._doraValid=!1,this._uradoraValid=!1)}}this._update()}_setTenho(e){switch(e){case"tenho":this._env.tenho=!0,this._env.chiho=!1;break;case"chiho":this._env.tenho=!1,this._env.chiho=!0;break;default:this._env.tenho=!1,this._env.chiho=!1}}_setRichi(e){switch(e){case"richi":this._env.richi=!0,this._env.daburi=!1;break;case"daburi":this._env.richi=!1,this._env.daburi=!0;break;default:this._env.richi=!1,this._env.daburi=!1}}}document.addEventListener("DOMContentLoaded",(e=>{!async function(){let e=await h.e(235).then(h.bind(h,235));new y(e,document.forms[0],document.getElementById("tehai-view"),document.getElementById("message"))}()}))})();