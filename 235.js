(self.webpackChunkmjview=self.webpackChunkmjview||[]).push([[235],{235:(t,r,_)=>{"use strict";_.a(t,(async t=>{_.r(r),_.d(r,{Agari:()=>n.rE,AgariHai:()=>n.UM,Env:()=>n.Xb,Furo:()=>n.d1,Hai:()=>n.hS,HaiImage:()=>n.qA,JunTehai:()=>n.hH,Mentsu:()=>n.by,Rank:()=>n.yw,Tehai:()=>n.jE,Yaku:()=>n.cw,__wbg_agari_new:()=>n.qC,__wbg_furo_new:()=>n.Zl,__wbg_haiimage_new:()=>n.Zn,__wbg_new_04918f9bdadadf45:()=>n.Fo,__wbg_push_60db4345d488a9b8:()=>n.F,__wbg_rank_new:()=>n.lW,__wbindgen_object_drop_ref:()=>n.ug,__wbindgen_rethrow:()=>n.nD,__wbindgen_string_new:()=>n.h4,__wbindgen_throw:()=>n.Or,main_js:()=>n.NV});var e=_(716),n=_(838),i=t([e,n]);[e,n]=i.then?await i:i,e.__wbindgen_start()}))},838:(t,r,_)=>{"use strict";_.a(t,(async e=>{_.d(r,{NV:()=>v,rE:()=>O,UM:()=>A,Xb:()=>E,d1:()=>I,hS:()=>T,qA:()=>x,hH:()=>z,by:()=>C,yw:()=>D,jE:()=>q,cw:()=>F,Fo:()=>H,h4:()=>U,lW:()=>Z,ug:()=>M,F:()=>W,Zl:()=>Y,qC:()=>N,Zn:()=>V,Or:()=>X,nD:()=>B});var n=_(716);t=_.hmd(t);var i=e([n]);n=(i.then?await i:i)[0];let a=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});a.decode();let o=null;function s(){return null!==o&&o.buffer===n.memory.buffer||(o=new Uint8Array(n.memory.buffer)),o}function c(t,r){return a.decode(s().subarray(t,t+r))}const d=new Array(32).fill(void 0);d.push(void 0,null,!0,!1);let p=d.length;function u(t){p===d.length&&d.push(d.length+1);const r=p;return p=d[r],d[r]=t,r}function g(t){return d[t]}function w(t){const r=g(t);return function(t){t<36||(d[t]=p,p=t)}(t),r}let h=null;function b(){return null!==h&&h.buffer===n.memory.buffer||(h=new Int32Array(n.memory.buffer)),h}function f(t,r){if(!(t instanceof r))throw new Error(`expected instance of ${r.name}`);return t.ptr}let l=null;function y(t,r){const _=(null!==l&&l.buffer===n.memory.buffer||(l=new Uint32Array(n.memory.buffer)),l).subarray(t/4,t/4+r),e=[];for(let t=0;t<_.length;t++)e.push(w(_[t]));return e}function v(){n.main_js()}let k=0,m=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const j="function"==typeof m.encodeInto?function(t,r){return m.encodeInto(t,r)}:function(t,r){const _=m.encode(t);return r.set(_),{read:t.length,written:_.length}};function S(t,r,_){if(void 0===_){const _=m.encode(t),e=r(_.length);return s().subarray(e,e+_.length).set(_),k=_.length,e}let e=t.length,n=r(e);const i=s();let a=0;for(;a<e;a++){const r=t.charCodeAt(a);if(r>127)break;i[n+a]=r}if(a!==e){0!==a&&(t=t.slice(a)),n=_(n,e,e=a+3*t.length);const r=s().subarray(n+a,n+e);a+=j(t,r).written}return k=a,n}class O{static __wrap(t){const r=Object.create(O.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_agari_free(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.agari_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}judgeYaku(t){f(t,E);var r=n.agari_judgeYaku(this.ptr,t.ptr);return F.__wrap(r)}compare(t){return f(t,O),n.agari_compare(this.ptr,t.ptr)}}class A{static __wrap(t){const r=Object.create(A.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_agarihai_free(t)}get agari(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.agarihai_agari(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}toImage(){var t=n.agarihai_toImage(this.ptr);return x.__wrap(t)}}class E{static __wrap(t){const r=Object.create(E.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_env_free(t)}constructor(){var t=n.env_new_js();return E.__wrap(t)}get tenho(){return 0!==n.env_tenho(this.ptr)}set tenho(t){n.env_set_tenho(this.ptr,t)}get chiho(){return 0!==n.env_chiho(this.ptr)}set chiho(t){n.env_set_chiho(this.ptr,t)}get richi(){return 0!==n.env_richi(this.ptr)}set richi(t){n.env_set_richi(this.ptr,t)}get daburi(){return 0!==n.env_daburi(this.ptr)}set daburi(t){n.env_set_daburi(this.ptr,t)}get ippatsu(){return 0!==n.env_ippatsu(this.ptr)}set ippatsu(t){n.env_set_ippatsu(this.ptr,t)}get rinshan(){return 0!==n.env_rinshan(this.ptr)}set rinshan(t){n.env_set_rinshan(this.ptr,t)}get haitei(){return 0!==n.env_haitei(this.ptr)}set haitei(t){n.env_set_haitei(this.ptr,t)}get bakaze(){var t=n.env_bakaze(this.ptr);return T.__wrap(t)}set bakaze(t){f(t,T),n.env_set_bakaze_js(this.ptr,t.ptr)}get jikaze(){var t=n.env_jikaze(this.ptr);return T.__wrap(t)}set jikaze(t){f(t,T),n.env_set_jikaze_js(this.ptr,t.ptr)}setDora(t){var r=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),_=k;n.env_setDora(this.ptr,r,_)}setUradora(t){var r=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),_=k;n.env_setUradora(this.ptr,r,_)}}class I{static __wrap(t){const r=Object.create(I.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_furo_free(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}toImage(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toImage(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1],_=y(t,r).slice();return n.__wbindgen_free(t,4*r),_}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var r=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),_=k,e=n.furo_fromStr(r,_);return I.__wrap(e)}}class T{static __wrap(t){const r=Object.create(T.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_hai_free(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.hai_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}static fromStr(t){var r=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),_=k,e=n.hai_fromStr(r,_);return T.__wrap(e)}}class x{static __wrap(t){const r=Object.create(x.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_haiimage_free(t)}get type(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_type(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}get hai(){var t=n.haiimage_hai(this.ptr);return 0===t?void 0:T.__wrap(t)}get top(){var t=n.haiimage_top(this.ptr);return 0===t?void 0:T.__wrap(t)}get bottom(){var t=n.haiimage_bottom(this.ptr);return 0===t?void 0:T.__wrap(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}}class z{static __wrap(t){const r=Object.create(z.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_juntehai_free(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}toImage(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toImage(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1],_=y(t,r).slice();return n.__wbindgen_free(t,4*r),_}finally{n.__wbindgen_add_to_stack_pointer(16)}}}class C{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_mentsu_free(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.mentsu_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}}class D{static __wrap(t){const r=Object.create(D.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_rank_free(t)}get fan(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.rank_fan_js(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return 0===t?void 0:r>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}get yakuman(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.rank_yakuman_js(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return 0===t?void 0:r>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.rank_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}}class q{static __wrap(t){const r=Object.create(q.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_tehai_free(t)}toString(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toString(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}get junTehai(){var t=n.tehai_junTehai(this.ptr);return z.__wrap(t)}get furo(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_furo(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1],_=y(t,r).slice();return n.__wbindgen_free(t,4*r),_}finally{n.__wbindgen_add_to_stack_pointer(16)}}get agariHai(){var t=n.tehai_agariHai(this.ptr);return A.__wrap(t)}toAgariCombinations(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toAgariCombinations(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1],_=y(t,r).slice();return n.__wbindgen_free(t,4*r),_}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var r=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),_=k,e=n.tehai_fromStr(r,_);return q.__wrap(e)}}class F{static __wrap(t){const r=Object.create(F.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_yaku_free(t)}get name(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.yaku_name_js(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}get point(){return n.yaku_point_js(this.ptr)>>>0}get fu(){return n.yaku_fu_js(this.ptr)>>>0}get rank(){var t=n.yaku_rank_js(this.ptr);return D.__wrap(t)}get detail(){return w(n.yaku_detail_js(this.ptr))}compare(t){return f(t,F),n.yaku_compare(this.ptr,t.ptr)}}const H=function(){return u(new Array)},U=function(t,r){return u(c(t,r))},Z=function(t){return u(D.__wrap(t))},M=function(t){w(t)},W=function(t,r){return g(t).push(g(r))},Y=function(t){return u(I.__wrap(t))},N=function(t){return u(O.__wrap(t))},V=function(t){return u(x.__wrap(t))},X=function(t,r){throw new Error(c(t,r))},B=function(t){throw w(t)}}))},716:(t,r,_)=>{"use strict";var e=([e])=>_.v(r,t.id,"2b21ced564b52b20be7c",{"./index_bg.js":{__wbg_new_04918f9bdadadf45:e.Fo,__wbindgen_string_new:e.h4,__wbg_rank_new:e.lW,__wbindgen_object_drop_ref:e.ug,__wbg_push_60db4345d488a9b8:e.F,__wbg_furo_new:e.Zl,__wbg_agari_new:e.qC,__wbg_haiimage_new:e.Zn,__wbindgen_throw:e.Or,__wbindgen_rethrow:e.nD}});_.a(t,(t=>{var r=t([_(838)]);return r.then?r.then(e):e(r)}),1)}}]);