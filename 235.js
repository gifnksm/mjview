(self.webpackChunkmjview=self.webpackChunkmjview||[]).push([[235],{235:(t,_,r)=>{"use strict";r.a(t,(async t=>{r.r(_),r.d(_,{Agari:()=>n.rE,AgariHai:()=>n.UM,Env:()=>n.Xb,Furo:()=>n.d1,Hai:()=>n.hS,HaiImage:()=>n.qA,JunTehai:()=>n.hH,Mentsu:()=>n.by,Rank:()=>n.yw,Tehai:()=>n.jE,Yaku:()=>n.cw,__wbg_agari_new:()=>n.qC,__wbg_furo_new:()=>n.Zl,__wbg_hai_new:()=>n.p,__wbg_haiimage_new:()=>n.Zn,__wbg_new_04918f9bdadadf45:()=>n.Fo,__wbg_push_60db4345d488a9b8:()=>n.F,__wbg_rank_new:()=>n.lW,__wbindgen_object_drop_ref:()=>n.ug,__wbindgen_rethrow:()=>n.nD,__wbindgen_string_new:()=>n.h4,__wbindgen_throw:()=>n.Or,main_js:()=>n.NV});var e=r(716),n=r(838),i=t([e,n]);[e,n]=i.then?await i:i,e.__wbindgen_start()}))},838:(t,_,r)=>{"use strict";r.a(t,(async e=>{r.d(_,{NV:()=>k,rE:()=>A,UM:()=>O,Xb:()=>z,d1:()=>C,hS:()=>E,qA:()=>H,hH:()=>I,by:()=>T,yw:()=>x,jE:()=>D,cw:()=>q,Zn:()=>F,p:()=>U,h4:()=>Z,Zl:()=>M,qC:()=>W,Fo:()=>Y,lW:()=>N,ug:()=>V,F:()=>X,Or:()=>B,nD:()=>J});var n=r(716);t=r.hmd(t);var i=e([n]);n=(i.then?await i:i)[0];let a=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});a.decode();let o=null;function s(){return null!==o&&o.buffer===n.memory.buffer||(o=new Uint8Array(n.memory.buffer)),o}function d(t,_){return a.decode(s().subarray(t,t+_))}const c=new Array(32).fill(void 0);c.push(void 0,null,!0,!1);let p=c.length;function u(t){p===c.length&&c.push(c.length+1);const _=p;return p=c[_],c[_]=t,_}function g(t){return c[t]}function w(t){const _=g(t);return function(t){t<36||(c[t]=p,p=t)}(t),_}let h=null;function b(){return null!==h&&h.buffer===n.memory.buffer||(h=new Int32Array(n.memory.buffer)),h}let f=0,l=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof l.encodeInto?function(t,_){return l.encodeInto(t,_)}:function(t,_){const r=l.encode(t);return _.set(r),{read:t.length,written:r.length}};function v(t,_,r){if(void 0===r){const r=l.encode(t),e=_(r.length);return s().subarray(e,e+r.length).set(r),f=r.length,e}let e=t.length,n=_(e);const i=s();let a=0;for(;a<e;a++){const _=t.charCodeAt(a);if(_>127)break;i[n+a]=_}if(a!==e){0!==a&&(t=t.slice(a)),n=r(n,e,e=a+3*t.length);const _=s().subarray(n+a,n+e);a+=y(t,_).written}return f=a,n}function k(){n.main_js()}let m=null;function j(t,_){const r=(null!==m&&m.buffer===n.memory.buffer||(m=new Uint32Array(n.memory.buffer)),m).subarray(t/4,t/4+_),e=[];for(let t=0;t<r.length;t++)e.push(w(r[t]));return e}function S(t,_){if(!(t instanceof _))throw new Error(`expected instance of ${_.name}`);return t.ptr}class A{static __wrap(t){const _=Object.create(A.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_agari_free(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.agari_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}judgeYaku(t){S(t,z);var _=n.agari_judgeYaku(this.ptr,t.ptr);return q.__wrap(_)}compare(t){return S(t,A),n.agari_compare(this.ptr,t.ptr)}}class O{static __wrap(t){const _=Object.create(O.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_agarihai_free(t)}get hai(){var t=n.agarihai_hai_js(this.ptr);return E.__wrap(t)}get agari(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.agarihai_agari_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}toImage(){var t=n.agarihai_toImage(this.ptr);return H.__wrap(t)}}class z{static __wrap(t){const _=Object.create(z.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_env_free(t)}constructor(){var t=n.env_new_js();return z.__wrap(t)}get tenho(){return 0!==n.env_tenho(this.ptr)}set tenho(t){n.env_set_tenho(this.ptr,t)}get richi(){return 0!==n.env_richi(this.ptr)}set richi(t){n.env_set_richi(this.ptr,t)}get daburi(){return 0!==n.env_daburi(this.ptr)}set daburi(t){n.env_set_daburi(this.ptr,t)}get ippatsu(){return 0!==n.env_ippatsu(this.ptr)}set ippatsu(t){n.env_set_ippatsu(this.ptr,t)}get rinshan(){return 0!==n.env_rinshan(this.ptr)}set rinshan(t){n.env_set_rinshan(this.ptr,t)}get haitei(){return 0!==n.env_haitei(this.ptr)}set haitei(t){n.env_set_haitei(this.ptr,t)}get bakaze(){var t=n.env_bakaze(this.ptr);return E.__wrap(t)}set bakaze(t){S(t,E),n.env_set_bakaze_js(this.ptr,t.ptr)}get jikaze(){var t=n.env_jikaze(this.ptr);return E.__wrap(t)}set jikaze(t){S(t,E),n.env_set_jikaze_js(this.ptr,t.ptr)}setDora(t){var _=v(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=f;n.env_setDora(this.ptr,_,r)}get doraCount(){return n.env_dora_count_js(this.ptr)>>>0}get dora(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.env_dora_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}setUradora(t){var _=v(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=f;n.env_setUradora(this.ptr,_,r)}get uradoraCount(){return n.env_uradora_count_ja(this.ptr)>>>0}get uradora(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.env_uradora_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}}class C{static __wrap(t){const _=Object.create(C.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_furo_free(t)}toHaiArray(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toHaiArray(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}toImage(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.furo_toImage(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var _=v(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=f,e=n.furo_fromStr(_,r);return C.__wrap(e)}}class E{static __wrap(t){const _=Object.create(E.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_hai_free(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.hai_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}static fromStr(t){var _=v(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=f,e=n.hai_fromStr(_,r);return E.__wrap(e)}}class H{static __wrap(t){const _=Object.create(H.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_haiimage_free(t)}get type(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_type(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get hai(){var t=n.haiimage_hai(this.ptr);return 0===t?void 0:E.__wrap(t)}get top(){var t=n.haiimage_top(this.ptr);return 0===t?void 0:E.__wrap(t)}get bottom(){var t=n.haiimage_bottom(this.ptr);return 0===t?void 0:E.__wrap(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.haiimage_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}}class I{static __wrap(t){const _=Object.create(I.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_juntehai_free(t)}toHaiArray(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toHaiArray(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}toImage(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.juntehai_toImage(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}}class T{__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_mentsu_free(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.mentsu_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}}class x{static __wrap(t){const _=Object.create(x.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_rank_free(t)}get fan(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.rank_fan_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return 0===t?void 0:_>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}get yakuman(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.rank_yakuman_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return 0===t?void 0:_>>>0}finally{n.__wbindgen_add_to_stack_pointer(16)}}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.rank_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}}class D{static __wrap(t){const _=Object.create(D.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_tehai_free(t)}toString(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toString(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get isMenzen(){return 0!==n.tehai_is_menzen_js(this.ptr)}get junTehai(){var t=n.tehai_jun_tehai_js(this.ptr);return I.__wrap(t)}get furo(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_furo_js(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}get agariHai(){var t=n.tehai_agari_hai_js(this.ptr);return O.__wrap(t)}toAgariCombinations(){try{const e=n.__wbindgen_add_to_stack_pointer(-16);n.tehai_toAgariCombinations(e,this.ptr);var t=b()[e/4+0],_=b()[e/4+1],r=j(t,_).slice();return n.__wbindgen_free(t,4*_),r}finally{n.__wbindgen_add_to_stack_pointer(16)}}static fromStr(t){var _=v(t,n.__wbindgen_malloc,n.__wbindgen_realloc),r=f,e=n.tehai_fromStr(_,r);return D.__wrap(e)}}class q{static __wrap(t){const _=Object.create(q.prototype);return _.ptr=t,_}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();n.__wbg_yaku_free(t)}get name(){try{const r=n.__wbindgen_add_to_stack_pointer(-16);n.yaku_name_js(r,this.ptr);var t=b()[r/4+0],_=b()[r/4+1];return d(t,_)}finally{n.__wbindgen_add_to_stack_pointer(16),n.__wbindgen_free(t,_)}}get point(){return n.yaku_point_js(this.ptr)>>>0}get fu(){return n.yaku_fu_js(this.ptr)>>>0}get rank(){var t=n.yaku_rank_js(this.ptr);return x.__wrap(t)}get detail(){return w(n.yaku_detail_js(this.ptr))}compare(t){return S(t,q),n.yaku_compare(this.ptr,t.ptr)}}const F=function(t){return u(H.__wrap(t))},U=function(t){return u(E.__wrap(t))},Z=function(t,_){return u(d(t,_))},M=function(t){return u(C.__wrap(t))},W=function(t){return u(A.__wrap(t))},Y=function(){return u(new Array)},N=function(t){return u(x.__wrap(t))},V=function(t){w(t)},X=function(t,_){return g(t).push(g(_))},B=function(t,_){throw new Error(d(t,_))},J=function(t){throw w(t)}}))},716:(t,_,r)=>{"use strict";var e=([e])=>r.v(_,t.id,"2ce6ecf9f75ac65bc586",{"./index_bg.js":{__wbg_haiimage_new:e.Zn,__wbg_hai_new:e.p,__wbindgen_string_new:e.h4,__wbg_furo_new:e.Zl,__wbg_agari_new:e.qC,__wbg_new_04918f9bdadadf45:e.Fo,__wbg_rank_new:e.lW,__wbindgen_object_drop_ref:e.ug,__wbg_push_60db4345d488a9b8:e.F,__wbindgen_throw:e.Or,__wbindgen_rethrow:e.nD}});r.a(t,(t=>{var _=t([r(838)]);return _.then?_.then(e):e(_)}),1)}}]);