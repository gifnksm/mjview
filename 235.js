"use strict";(self.webpackChunkmjview=self.webpackChunkmjview||[]).push([[235],{235:(t,_,r)=>{r.a(t,(async t=>{r.r(_),r.d(_,{Agari:()=>n.rE,AgariHai:()=>n.UM,Env:()=>n.Xb,Furo:()=>n.d1,Hai:()=>n.hS,HaiImage:()=>n.qA,JunTehai:()=>n.hH,Mentsu:()=>n.by,Rank:()=>n.yw,Tehai:()=>n.jE,Yaku:()=>n.cw,__wbg_agari_new:()=>n.qC,__wbg_furo_new:()=>n.Zl,__wbg_hai_new:()=>n.p,__wbg_haiimage_new:()=>n.Zn,__wbg_new_949bbc1147195c4e:()=>n.FU,__wbg_of_db9e1b8e0a7ed9bc:()=>n.YQ,__wbg_push_284486ca27c6aa8b:()=>n.gZ,__wbg_rank_new:()=>n.lW,__wbindgen_object_drop_ref:()=>n.ug,__wbindgen_rethrow:()=>n.nD,__wbindgen_string_new:()=>n.h4,__wbindgen_throw:()=>n.Or,main_js:()=>n.NV});var e=r(530),n=r(838),i=t([e,n]);[e,n]=i.then?await i:i,e.__wbindgen_start()}))},838:(t,_,r)=>{r.a(t,(async e=>{r.d(_,{NV:()=>k,rE:()=>A,UM:()=>O,Xb:()=>T,d1:()=>E,hS:()=>I,qA:()=>H,hH:()=>U,by:()=>Z,yw:()=>x,jE:()=>z,cw:()=>C,FU:()=>D,h4:()=>q,lW:()=>W,ug:()=>Y,gZ:()=>F,Zn:()=>M,p:()=>P,YQ:()=>Q,Zl:()=>N,qC:()=>V,Or:()=>X,nD:()=>B});var n=r(530);t=r.hmd(t);var i=e([n]);n=(i.then?await i:i)[0];let a=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});a.decode();let o=null;function s(){return null!==o&&o.buffer===n.memory.buffer||(o=new Uint8Array(n.memory.buffer)),o}function d(t,_){return a.decode(s().subarray(t,t+_))}const c=new Array(32).fill(void 0);c.push(void 0,null,!0,!1);let p=c.length;function w(t){p===c.length&&c.push(c.length+1);const _=p;return p=c[_],c[_]=t,_}function g(t){return c[t]}function u(t){const _=g(t);return function(t){t<36||(c[t]=p,p=t)}(t),_}let h=null;function b(){return null!==h&&h.buffer===n.memory.buffer||(h=new Int32Array(n.memory.buffer)),h}function f(t,_){if(!(t instanceof _))throw new Error(`expected instance of ${_.name}`);return t.ptr}let l=null;function y(t,_){const r=(null!==l&&l.buffer===n.memory.buffer||(l=new Uint32Array(n.memory.buffer)),l).subarray(t/4,t/4+_),e=[];for(let t=0;t<r.length;t++)e.push(u(r[t]));return e}function k(){n.main_js()}let v=0,m=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const j="function"==typeof m.encodeInto?function(t,_){return m.encodeInto(t,_)}:function(t,_){const r=m.encode(t);return _.set(r),{read:t.length,written:r.length}};function S(t,_,r){if(void 0===r){const r=m.encode(t),e=_(r.length);return s().subarray(e,e+r.length).set(r),v=r.length,e}let e=t.length,n=_(e);const i=s();let a=0;for(;a<e;a++){const _=t.charCodeAt(a);if(_>127)break;i[n+a]=_}if(a!==e){0!==a&&(t=t.slice(a)),n=r(n,e,e=a+3*t.length);const _=s().subarray(n+a,n+e);a+=j(t,_).written}return v=a,n}class A{static __wrap(t){const _=Object.create(A.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_agari_free(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.agari_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}judgeYaku(t){f(t,T);var _=n.agari_judgeYaku(this.ptr,t.ptr);return C.__wrap(_)}compare(t){return f(t,A),n.agari_compare(this.ptr,t.ptr)}}class O{static __wrap(t){const _=Object.create(O.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_agarihai_free(t)}get hai(){var t=n.agarihai_hai_js(this.ptr);return I.__wrap(t)}get agari(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.agarihai_agari_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}toImage(){const t=this.__destroy_into_raw();var _=n.agarihai_toImage(t);return H.__wrap(_)}}class T{static __wrap(t){const _=Object.create(T.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_env_free(t)}constructor(){var t=n.env_new_js();return T.__wrap(t)}get richi(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.env_richi(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}set richi(t){var _=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=v;n.env_set_richi(this.ptr,_,r)}get ippatsu(){return 0!==n.env_ippatsu(this.ptr)}set ippatsu(t){n.env_set_ippatsu(this.ptr,t)}get rinshan(){return 0!==n.env_rinshan(this.ptr)}set rinshan(t){n.env_set_rinshan(this.ptr,t)}get haitei(){return 0!==n.env_haitei(this.ptr)}set haitei(t){n.env_set_haitei(this.ptr,t)}get tenho(){return 0!==n.env_tenho(this.ptr)}set tenho(t){n.env_set_tenho(this.ptr,t)}get bakaze(){var t=n.env_bakaze(this.ptr);return I.__wrap(t)}set bakaze(t){f(t,I),n.env_set_bakaze_js(this.ptr,t.ptr)}get jikaze(){var t=n.env_jikaze(this.ptr);return I.__wrap(t)}set jikaze(t){f(t,I),n.env_set_jikaze_js(this.ptr,t.ptr)}setDora(t){var _=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=v;n.env_setDora(this.ptr,_,r)}get dora(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.env_dora_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}setUradora(t){var _=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=v;n.env_setUradora(this.ptr,_,r)}get uradora(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.env_uradora_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}get aotenjo(){return 0!==n.env_aotenjo(this.ptr)}set aotenjo(t){n.env_set_aotenjo(this.ptr,t)}checkPropsWithTehai(t){try{const i=n.__wbindgen_add_to_stack_pointer(-16);f(t,z),n.env_checkPropsWithTehai(i,this.ptr,t.ptr);var _=b()[i/4+0],r=b()[i/4+1],e=y(_,r).slice();return n.__wbindgen_free(_,4*r),e}finally{n.__wbindgen_add_to_stack_pointer(16)}}checkPropsWithoutTehai(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.env_checkPropsWithoutTehai(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}}class E{static __wrap(t){const _=Object.create(E.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_furo_free(t)}toHaiArray(){try{const e=this.__destroy_into_raw(),i=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toHaiArray(i,e);var t=b()[i/4+0],_=b()[i/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const r=this.__destroy_into_raw(),e=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toString(e,r);var t=b()[e/4+0],_=b()[e/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}toImage(){try{const e=this.__destroy_into_raw(),i=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toImage(i,e);var t=b()[i/4+0],_=b()[i/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var _=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=v,e=n.furo_fromStr(_,r);return E.__wrap(e)}}class I{static __wrap(t){const _=Object.create(I.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_hai_free(t)}toString(){try{const r=this.__destroy_into_raw(),e=n.__wbindgen_add_to_stack_pointer(-16);n.hai_toString(e,r);var t=b()[e/4+0],_=b()[e/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}static fromStr(t){var _=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=v,e=n.hai_fromStr(_,r);return I.__wrap(e)}}class H{static __wrap(t){const _=Object.create(H.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_haiimage_free(t)}get type(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_type(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get hai(){var t=n.haiimage_hai(this.ptr);return 0===t?void 0:I.__wrap(t)}get top(){var t=n.haiimage_top(this.ptr);return 0===t?void 0:I.__wrap(t)}get bottom(){var t=n.haiimage_bottom(this.ptr);return 0===t?void 0:I.__wrap(t)}toString(){try{const r=this.__destroy_into_raw(),e=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_toString(e,r);var t=b()[e/4+0],_=b()[e/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}}class U{static __wrap(t){const _=Object.create(U.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_juntehai_free(t)}toHaiArray(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.env_dora_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}toImage(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toImage(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}}class Z{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_mentsu_free(t)}toString(){try{const r=this.__destroy_into_raw(),e=n.__wbindgen_add_to_stack_pointer(-16);n.mentsu_toString(e,r);var t=b()[e/4+0],_=b()[e/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}}class x{static __wrap(t){const _=Object.create(x.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_rank_free(t)}get fan(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.rank_fan_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return 0===t?void 0:_>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}get yakuman(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.rank_yakuman_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return 0===t?void 0:_>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const r=this.__destroy_into_raw(),e=n.__wbindgen_add_to_stack_pointer(-16);n.rank_toString(e,r);var t=b()[e/4+0],_=b()[e/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}}class z{static __wrap(t){const _=Object.create(z.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_tehai_free(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get junTehai(){var t=n.tehai_jun_tehai_js(this.ptr);return U.__wrap(t)}get furo(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_furo_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}get agariHai(){var t=n.tehai_agari_hai_js(this.ptr);return O.__wrap(t)}toAgariCombinations(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toAgariCombinations(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=y(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var _=S(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=v,e=n.tehai_fromStr(_,r);return z.__wrap(e)}}class C{static __wrap(t){const _=Object.create(C.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_yaku_free(t)}get name(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.yaku_name_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get point(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.yaku_point_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get fu(){return n.yaku_fu_js(this.ptr)>>>0}get rank(){var t=n.yaku_rank_js(this.ptr);return x.__wrap(t)}get detail(){return u(n.yaku_detail_js(this.ptr))}compare(t){return f(t,C),n.yaku_compare(this.ptr,t.ptr)}}function D(){return w(new Array)}function q(t,_){return w(d(t,_))}function W(t){return w(x.__wrap(t))}function Y(t){u(t)}function F(t,_){return g(t).push(g(_))}function M(t){return w(H.__wrap(t))}function P(t){return w(I.__wrap(t))}function Q(t,_){return w(Array.of(g(t),g(_)))}function N(t){return w(E.__wrap(t))}function V(t){return w(A.__wrap(t))}function X(t,_){throw new Error(d(t,_))}function B(t){throw u(t)}}))},530:(t,_,r)=>{var e=([e])=>r.v(_,t.id,"4bab65e3171b44ee1bdd",{"./index_bg.js":{__wbg_new_949bbc1147195c4e:e.FU,__wbindgen_string_new:e.h4,__wbg_rank_new:e.lW,__wbindgen_object_drop_ref:e.ug,__wbg_push_284486ca27c6aa8b:e.gZ,__wbg_haiimage_new:e.Zn,__wbg_hai_new:e.p,__wbg_of_db9e1b8e0a7ed9bc:e.YQ,__wbg_furo_new:e.Zl,__wbg_agari_new:e.qC,__wbindgen_throw:e.Or,__wbindgen_rethrow:e.nD}});r.a(t,(t=>{var _=t([r(838)]);return _.then?_.then(e):e(_)}),1)}}]);