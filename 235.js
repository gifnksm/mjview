(self.webpackChunkmjview=self.webpackChunkmjview||[]).push([[235],{235:(t,r,e)=>{"use strict";e.a(t,(async t=>{e.r(r),e.d(r,{Agari:()=>n.rE,AgariHai:()=>n.UM,Env:()=>n.Xb,Furo:()=>n.d1,Hai:()=>n.hS,HaiImage:()=>n.qA,JunTehai:()=>n.hH,Mentsu:()=>n.by,Rank:()=>n.yw,Tehai:()=>n.jE,Yaku:()=>n.cw,__wbg_agari_new:()=>n.qC,__wbg_furo_new:()=>n.Zl,__wbg_haiimage_new:()=>n.Zn,__wbg_new_1abc33d4f9ba3e80:()=>n.fI,__wbg_push_44968dcdf4cfbb43:()=>n.Kz,__wbg_rank_new:()=>n.lW,__wbindgen_object_drop_ref:()=>n.ug,__wbindgen_rethrow:()=>n.nD,__wbindgen_string_new:()=>n.h4,__wbindgen_throw:()=>n.Or,main_js:()=>n.NV});var _=e(716),n=e(838),i=t([_,n]);[_,n]=i.then?await i:i,_.__wbindgen_start()}))},838:(t,r,e)=>{"use strict";e.a(t,(async _=>{e.d(r,{NV:()=>y,rE:()=>O,UM:()=>I,Xb:()=>A,d1:()=>z,hS:()=>E,qA:()=>T,hH:()=>x,by:()=>C,yw:()=>D,jE:()=>q,cw:()=>H,fI:()=>U,h4:()=>Z,lW:()=>M,ug:()=>K,Kz:()=>W,Zn:()=>Y,Zl:()=>N,qC:()=>V,Or:()=>X,nD:()=>B});var n=e(716);t=e.hmd(t);var i=_([n]);n=(i.then?await i:i)[0];let a=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});a.decode();let o=null;function s(){return null!==o&&o.buffer===n.memory.buffer||(o=new Uint8Array(n.memory.buffer)),o}function c(t,r){return a.decode(s().subarray(t,t+r))}const p=new Array(32).fill(void 0);p.push(void 0,null,!0,!1);let d=p.length;function g(t){d===p.length&&p.push(p.length+1);const r=d;return d=p[r],p[r]=t,r}function u(t){return p[t]}function h(t){const r=u(t);return function(t){t<36||(p[t]=d,d=t)}(t),r}let w=null;function b(){return null!==w&&w.buffer===n.memory.buffer||(w=new Int32Array(n.memory.buffer)),w}let f=null;function l(t,r){const e=(null!==f&&f.buffer===n.memory.buffer||(f=new Uint32Array(n.memory.buffer)),f).subarray(t/4,t/4+r),_=[];for(let t=0;t<e.length;t++)_.push(h(e[t]));return _}function y(){n.main_js()}let v=0,k=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const m="function"==typeof k.encodeInto?function(t,r){return k.encodeInto(t,r)}:function(t,r){const e=k.encode(t);return r.set(e),{read:t.length,written:e.length}};function j(t,r,e){if(void 0===e){const e=k.encode(t),_=r(e.length);return s().subarray(_,_+e.length).set(e),v=e.length,_}let _=t.length,n=r(_);const i=s();let a=0;for(;a<_;a++){const r=t.charCodeAt(a);if(r>127)break;i[n+a]=r}if(a!==_){0!==a&&(t=t.slice(a)),n=e(n,_,_=a+3*t.length);const r=s().subarray(n+a,n+_);a+=m(t,r).written}return v=a,n}function S(t,r){if(!(t instanceof r))throw new Error(`expected instance of ${r.name}`);return t.ptr}class O{static __wrap(t){const r=Object.create(O.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_agari_free(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.agari_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}judgeYaku(t){S(t,A);var r=n.agari_judgeYaku(this.ptr,t.ptr);return H.__wrap(r)}}class I{static __wrap(t){const r=Object.create(I.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_agarihai_free(t)}get agari(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.agarihai_agari(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}toImage(){var t=n.agarihai_toImage(this.ptr);return T.__wrap(t)}}class A{static __wrap(t){const r=Object.create(A.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_env_free(t)}constructor(){var t=n.env_new_js();return A.__wrap(t)}get tenho(){return 0!==n.env_tenho(this.ptr)}set tenho(t){n.env_set_tenho(this.ptr,t)}get chiho(){return 0!==n.env_chiho(this.ptr)}set chiho(t){n.env_set_chiho(this.ptr,t)}get richi(){return 0!==n.env_richi(this.ptr)}set richi(t){n.env_set_richi(this.ptr,t)}get daburi(){return 0!==n.env_daburi(this.ptr)}set daburi(t){n.env_set_daburi(this.ptr,t)}get ippatsu(){return 0!==n.env_ippatsu(this.ptr)}set ippatsu(t){n.env_set_ippatsu(this.ptr,t)}get rinshan(){return 0!==n.env_rinshan(this.ptr)}set rinshan(t){n.env_set_rinshan(this.ptr,t)}get haitei(){return 0!==n.env_haitei(this.ptr)}set haitei(t){n.env_set_haitei(this.ptr,t)}get bakaze(){var t=n.env_bakaze(this.ptr);return E.__wrap(t)}set bakaze(t){S(t,E),n.env_set_bakaze_js(this.ptr,t.ptr)}get jikaze(){var t=n.env_jikaze(this.ptr);return E.__wrap(t)}set jikaze(t){S(t,E),n.env_set_jikaze_js(this.ptr,t.ptr)}setDora(t){var r=j(t,n.__wbindgen_malloc,n.__wbindgen_realloc),e=v;n.env_setDora(this.ptr,r,e)}setUradora(t){var r=j(t,n.__wbindgen_malloc,n.__wbindgen_realloc),e=v;n.env_setUradora(this.ptr,r,e)}}class z{static __wrap(t){const r=Object.create(z.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_furo_free(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}toImage(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toImage(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1],e=l(t,r).slice();return n.__wbindgen_free(t,4*r),e}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var r=j(t,n.__wbindgen_malloc,n.__wbindgen_realloc),e=v,_=n.furo_fromStr(r,e);return z.__wrap(_)}}class E{static __wrap(t){const r=Object.create(E.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_hai_free(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.hai_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}static fromStr(t){var r=j(t,n.__wbindgen_malloc,n.__wbindgen_realloc),e=v,_=n.hai_fromStr(r,e);return E.__wrap(_)}}class T{static __wrap(t){const r=Object.create(T.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_haiimage_free(t)}get type(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_type(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}get hai(){var t=n.haiimage_hai(this.ptr);return 0===t?void 0:E.__wrap(t)}get top(){var t=n.haiimage_top(this.ptr);return 0===t?void 0:E.__wrap(t)}get bottom(){var t=n.haiimage_bottom(this.ptr);return 0===t?void 0:E.__wrap(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}}class x{static __wrap(t){const r=Object.create(x.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_juntehai_free(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}toImage(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toImage(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1],e=l(t,r).slice();return n.__wbindgen_free(t,4*r),e}finally{n.__wbindgen_add_to_stack_pointer(16)}}}class C{free(){const t=this.ptr;this.ptr=0,n.__wbg_mentsu_free(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.mentsu_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}}class D{static __wrap(t){const r=Object.create(D.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_rank_free(t)}get fan(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.rank_fan_js(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return 0===t?void 0:r>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}get yakuman(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.rank_yakuman_js(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return 0===t?void 0:r>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.rank_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}}class q{static __wrap(t){const r=Object.create(q.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_tehai_free(t)}toString(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toString(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}get junTehai(){var t=n.tehai_junTehai(this.ptr);return x.__wrap(t)}get furo(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_furo(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1],e=l(t,r).slice();return n.__wbindgen_free(t,4*r),e}finally{n.__wbindgen_add_to_stack_pointer(16)}}get agariHai(){var t=n.tehai_agariHai(this.ptr);return I.__wrap(t)}toAgariCombinations(){try{const _=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toAgariCombinations(_,this.ptr);var t=b()[_/4+0],r=b()[_/4+1],e=l(t,r).slice();return n.__wbindgen_free(t,4*r),e}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var r=j(t,n.__wbindgen_malloc,n.__wbindgen_realloc),e=v,_=n.tehai_fromStr(r,e);return q.__wrap(_)}}class H{static __wrap(t){const r=Object.create(H.prototype);return r.ptr=t,r}free(){const t=this.ptr;this.ptr=0,n.__wbg_yaku_free(t)}get name(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.yaku_name_js(e,this.ptr);var t=b()[e/4+0],r=b()[e/4+1];return c(t,r)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,r)}}get point(){return n.yaku_point_js(this.ptr)>>>0}get fu(){return n.yaku_fu_js(this.ptr)>>>0}get rank(){var t=n.yaku_rank_js(this.ptr);return D.__wrap(t)}get detail(){return h(n.yaku_detail_js(this.ptr))}}const U=function(){return g(new Array)},Z=function(t,r){return g(c(t,r))},M=function(t){return g(D.__wrap(t))},K=function(t){h(t)},W=function(t,r){return u(t).push(u(r))},Y=function(t){return g(T.__wrap(t))},N=function(t){return g(z.__wrap(t))},V=function(t){return g(O.__wrap(t))},X=function(t,r){throw new Error(c(t,r))},B=function(t){throw h(t)}}))},716:(t,r,e)=>{"use strict";var _=([_])=>e.v(r,t.id,"46c6acd69e287aecc523",{"./index_bg.js":{__wbg_new_1abc33d4f9ba3e80:_.fI,__wbindgen_string_new:_.h4,__wbg_rank_new:_.lW,__wbindgen_object_drop_ref:_.ug,__wbg_push_44968dcdf4cfbb43:_.Kz,__wbg_haiimage_new:_.Zn,__wbg_furo_new:_.Zl,__wbg_agari_new:_.qC,__wbindgen_throw:_.Or,__wbindgen_rethrow:_.nD}});e.a(t,(t=>{var r=t([e(838)]);return r.then?r.then(_):_(r)}),1)}}]);