import{s as R,f as g,g as p,h as v,d as h,j as d,A as de,k as A,i as D,x as b,B as le,y as x,a as L,C as z,c as I,D as fe,E as _e,F as he,l as be,m as ge,n as pe,p as te,G as ne}from"../chunks/scheduler.52e57df2.js";import{S as U,i as W,f as re,b as H,d as N,m as B,a as F,t as P,e as q}from"../chunks/index.069b48db.js";function se(t){return(t==null?void 0:t.length)!==void 0?t:Array.from(t)}let m;const ce=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&ce.decode();let S=null;function G(){return(S===null||S.byteLength===0)&&(S=new Uint8Array(m.memory.buffer)),S}function ae(t,e){return t=t>>>0,ce.decode(G().subarray(t,t+e))}const T=new Array(128).fill(void 0);T.push(void 0,null,!0,!1);let j=T.length;function C(t){j===T.length&&T.push(T.length+1);const e=j;return j=T[e],T[e]=t,e}function J(t){return T[t]}function me(t){t<132||(T[t]=j,j=t)}function Y(t){const e=J(t);return me(t),e}let Z=0;const K=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},ve=typeof K.encodeInto=="function"?function(t,e){return K.encodeInto(t,e)}:function(t,e){const n=K.encode(t);return e.set(n),{read:t.length,written:n.length}};function we(t,e,n){if(n===void 0){const l=K.encode(t),c=e(l.length,1)>>>0;return G().subarray(c,c+l.length).set(l),Z=l.length,c}let r=t.length,a=e(r,1)>>>0;const s=G();let o=0;for(;o<r;o++){const l=t.charCodeAt(o);if(l>127)break;s[a+o]=l}if(o!==r){o!==0&&(t=t.slice(o)),a=n(a,r,r=o+t.length*3,1)>>>0;const l=G().subarray(a+o,a+r),c=ve(t,l);o+=c.written}return Z=o,a}let V=null;function X(){return(V===null||V.byteLength===0)&&(V=new Int32Array(m.memory.buffer)),V}function ye(t,e){try{return t.apply(this,e)}catch(n){m.__wbindgen_exn_store(C(n))}}class Q{static __wrap(e){e=e>>>0;const n=Object.create(Q.prototype);return n.__wbg_ptr=e,n}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();m.__wbg_interpreter_free(e)}static new(){const e=m.interpreter_new();return Q.__wrap(e)}interpret_str_web(e){try{const s=m.__wbindgen_add_to_stack_pointer(-16),o=we(e,m.__wbindgen_malloc,m.__wbindgen_realloc),l=Z;m.interpreter_interpret_str_web(s,this.__wbg_ptr,o,l);var n=X()[s/4+0],r=X()[s/4+1],a=X()[s/4+2];if(a)throw Y(r);return Y(n)}finally{m.__wbindgen_add_to_stack_pointer(16)}}}async function xe(t,e){if(typeof Response=="function"&&t instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(t,e)}catch(r){if(t.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r);else throw r}const n=await t.arrayBuffer();return await WebAssembly.instantiate(n,e)}else{const n=await WebAssembly.instantiate(t,e);return n instanceof WebAssembly.Instance?{instance:n,module:t}:n}}function $e(){const t={};return t.wbg={},t.wbg.__wbindgen_string_new=function(e,n){const r=ae(e,n);return C(r)},t.wbg.__wbindgen_bigint_from_i64=function(e){return C(e)},t.wbg.__wbindgen_number_new=function(e){return C(e)},t.wbg.__wbindgen_object_drop_ref=function(e){Y(e)},t.wbg.__wbg_new_b51585de1b234aff=function(){const e=new Object;return C(e)},t.wbg.__wbg_set_092e06b0f9d71865=function(){return ye(function(e,n,r){return Reflect.set(J(e),J(n),J(r))},arguments)},t.wbg.__wbindgen_throw=function(e,n){throw new Error(ae(e,n))},t}function Te(t,e){return m=t.exports,ue.__wbindgen_wasm_module=e,V=null,S=null,m}async function ue(t){if(m!==void 0)return m;typeof t>"u"&&(t=new URL(""+new URL("../assets/interpreter_bg.a2f70d3a.wasm",import.meta.url).href,self.location));const e=$e();(typeof t=="string"||typeof Request=="function"&&t instanceof Request||typeof URL=="function"&&t instanceof URL)&&(t=fetch(t));const{instance:n,module:r}=await xe(await t,e);return Te(n,r)}const Ee=async()=>(await ue(),{interpreter:Q.new()}),Ne=Object.freeze(Object.defineProperty({__proto__:null,load:Ee},Symbol.toStringTag,{value:"Module"}));function De(t){let e,n,r,a;return{c(){e=g("div"),n=g("div"),this.h()},l(s){e=p(s,"DIV",{class:!0,style:!0});var o=v(e);n=p(o,"DIV",{contenteditable:!0,class:!0,placeholder:!0}),v(n).forEach(h),o.forEach(h),this.h()},h(){d(n,"contenteditable","true"),d(n,"class","code-editor svelte-1kx1mxy"),d(n,"placeholder","Insert code here..."),t[0]===void 0&&de(()=>t[1].call(n)),d(e,"class","d-flex text-bg-dark w-100 px-1 pt-2"),A(e,"height","85%")},m(s,o){D(s,e,o),b(e,n),t[0]!==void 0&&(n.innerText=t[0]),r||(a=le(n,"input",t[1]),r=!0)},p(s,[o]){o&1&&s[0]!==n.innerText&&(n.innerText=s[0])},i:x,o:x,d(s){s&&h(e),r=!1,a()}}}function Le(t,e,n){let{source:r=`type Point = {x: int, y: int}

	Point {x: 1, y: 2}`}=e;function a(){r=this.innerText,n(0,r)}return t.$$set=s=>{"source"in s&&n(0,r=s.source)},[r,a]}class Ie extends U{constructor(e){super(),W(this,e,Le,De,R,{source:0})}}function ke(t){let e,n,r,a='<span class="navbar-toggler-icon"></span>',s,o,l='<a class="navbar-brand" href="/">Focus Lang</a> <ul class="navbar-nav me-auto mb-2 mb-lg-0"></ul>',c,f,i,_,E='<i class="bi bi-play fs-2"></i>',$,O;return{c(){e=g("nav"),n=g("div"),r=g("button"),r.innerHTML=a,s=L(),o=g("div"),o.innerHTML=l,c=L(),f=g("ul"),i=g("li"),_=g("button"),_.innerHTML=E,this.h()},l(k){e=p(k,"NAV",{class:!0});var M=v(e);n=p(M,"DIV",{class:!0});var u=v(n);r=p(u,"BUTTON",{class:!0,type:!0,"data-bs-toggle":!0,"data-bs-target":!0,"aria-controls":!0,"aria-expanded":!0,"aria-label":!0,"data-svelte-h":!0}),z(r)!=="svelte-6a3bz6"&&(r.innerHTML=a),s=I(u),o=p(u,"DIV",{class:!0,id:!0,"data-svelte-h":!0}),z(o)!=="svelte-15a6lbw"&&(o.innerHTML=l),c=I(u),f=p(u,"UL",{class:!0});var w=v(f);i=p(w,"LI",{class:!0});var y=v(i);_=p(y,"BUTTON",{class:!0,"aria-current":!0,"data-svelte-h":!0}),z(_)!=="svelte-i92bm7"&&(_.innerHTML=E),y.forEach(h),w.forEach(h),u.forEach(h),M.forEach(h),this.h()},h(){d(r,"class","navbar-toggler"),d(r,"type","button"),d(r,"data-bs-toggle","collapse"),d(r,"data-bs-target","#navbarTogglerDemo01"),d(r,"aria-controls","navbarTogglerDemo01"),d(r,"aria-expanded","false"),d(r,"aria-label","Toggle navigation"),d(o,"class","collapse navbar-collapse"),d(o,"id","navbarTogglerDemo01"),d(_,"class","nav-link rounded-0"),d(_,"aria-current","page"),d(i,"class","nav-item"),d(f,"class","nav nav-pills mb-auto text-center mr-0 ml-auto"),d(n,"class","container-fluid"),d(e,"class","navbar navbar-expand-lg bg-body-tertiary")},m(k,M){D(k,e,M),b(e,n),b(n,r),b(n,s),b(n,o),b(n,c),b(n,f),b(f,i),b(i,_),$||(O=le(_,"click",fe(t[1])),$=!0)},p:x,i:x,o:x,d(k){k&&h(e),$=!1,O()}}}function Me(t){const e=_e();return[e,()=>e("run")]}class Ae extends U{constructor(e){super(),W(this,e,Me,ke,R,{})}}function oe(t,e,n){const r=t.slice();return r[1]=e[n],r}function ie(t){let e,n=t[1]+"",r,a;return{c(){e=g("div"),r=be(n),a=L(),this.h()},l(s){e=p(s,"DIV",{class:!0});var o=v(e);r=ge(o,n),a=I(o),o.forEach(h),this.h()},h(){d(e,"class","border-bottom border-dark w-100 mb-1")},m(s,o){D(s,e,o),b(e,r),b(e,a)},p(s,o){o&1&&n!==(n=s[1]+"")&&pe(r,n)},d(s){s&&h(e)}}}function Oe(t){let e,n,r='<div class="mx-1">Output</div>',a,s,o=se(t[0]),l=[];for(let c=0;c<o.length;c+=1)l[c]=ie(oe(t,o,c));return{c(){e=g("div"),n=g("span"),n.innerHTML=r,a=L(),s=g("div");for(let c=0;c<l.length;c+=1)l[c].c();this.h()},l(c){e=p(c,"DIV",{class:!0,style:!0});var f=v(e);n=p(f,"SPAN",{class:!0,"data-bs-theme":!0,"data-svelte-h":!0}),z(n)!=="svelte-19dnlzb"&&(n.innerHTML=r),a=I(f),s=p(f,"DIV",{class:!0,contenteditable:!0});var i=v(s);for(let _=0;_<l.length;_+=1)l[_].l(i);i.forEach(h),f.forEach(h),this.h()},h(){d(n,"class","bg-body-tertiary bg-dark"),d(n,"data-bs-theme","dark"),d(s,"class","output-text-area px-1 svelte-7kqvvn"),d(s,"contenteditable","false"),d(e,"class","d-flex flex-column text-bg-dark w-100"),A(e,"height","30%")},m(c,f){D(c,e,f),b(e,n),b(e,a),b(e,s);for(let i=0;i<l.length;i+=1)l[i]&&l[i].m(s,null)},p(c,[f]){if(f&1){o=se(c[0]);let i;for(i=0;i<o.length;i+=1){const _=oe(c,o,i);l[i]?l[i].p(_,f):(l[i]=ie(_),l[i].c(),l[i].m(s,null))}for(;i<l.length;i+=1)l[i].d(1);l.length=o.length}},i:x,o:x,d(c){c&&h(e),he(l,c)}}}function Se(t,e,n){let{output:r=[]}=e;return t.$$set=a=>{"output"in a&&n(0,r=a.output)},[r]}class Ce extends U{constructor(e){super(),W(this,e,Se,Oe,R,{output:0})}}function Ve(t){let e,n,r;return{c(){e=g("div"),n=L(),r=g("div"),this.h()},l(a){e=p(a,"DIV",{class:!0,style:!0}),v(e).forEach(h),n=I(a),r=p(a,"DIV",{class:!0}),v(r).forEach(h),this.h()},h(){d(e,"class","flex-shrink-0 p-3"),A(e,"width","12rem"),d(r,"class","divider bg-primary svelte-56h5ca")},m(a,s){D(a,e,s),D(a,n,s),D(a,r,s)},p:x,i:x,o:x,d(a){a&&(h(e),h(n),h(r))}}}class je extends U{constructor(e){super(),W(this,e,null,Ve,R,{})}}function Re(t){let e,n,r,a,s,o,l,c,f,i,_,E;n=new je({}),n.$on("run",t[2]),n.$on("clear",t[4]),s=new Ae({}),s.$on("run",t[2]);function $(u){t[5](u)}let O={};t[0]!==void 0&&(O.source=t[0]),l=new Ie({props:O}),te.push(()=>re(l,"source",$));function k(u){t[6](u)}let M={};return t[1]!==void 0&&(M.output=t[1]),i=new Ce({props:M}),te.push(()=>re(i,"output",k)),{c(){e=g("main"),H(n.$$.fragment),r=L(),a=g("div"),H(s.$$.fragment),o=L(),H(l.$$.fragment),f=L(),H(i.$$.fragment),this.h()},l(u){e=p(u,"MAIN",{class:!0,style:!0});var w=v(e);N(n.$$.fragment,w),r=I(w),a=p(w,"DIV",{class:!0});var y=v(a);N(s.$$.fragment,y),o=I(y),N(l.$$.fragment,y),f=I(y),N(i.$$.fragment,y),y.forEach(h),w.forEach(h),this.h()},h(){d(a,"class","d-flex flex-column w-100"),d(e,"class","d-flex flex-nowrap"),A(e,"height","100vh"),A(e,"max-height","100vh"),A(e,"overflow-x","auto"),A(e,"overflow-y","hidden")},m(u,w){D(u,e,w),B(n,e,null),b(e,r),b(e,a),B(s,a,null),b(a,o),B(l,a,null),b(a,f),B(i,a,null),E=!0},p(u,[w]){const y={};!c&&w&1&&(c=!0,y.source=u[0],ne(()=>c=!1)),l.$set(y);const ee={};!_&&w&2&&(_=!0,ee.output=u[1],ne(()=>_=!1)),i.$set(ee)},i(u){E||(F(n.$$.fragment,u),F(s.$$.fragment,u),F(l.$$.fragment,u),F(i.$$.fragment,u),E=!0)},o(u){P(n.$$.fragment,u),P(s.$$.fragment,u),P(l.$$.fragment,u),P(i.$$.fragment,u),E=!1},d(u){u&&h(e),q(n),q(s),q(l),q(i)}}}function Ue(t,e,n){let{data:r}=e,a,s;function o(){if(a){try{console.debug("Executing code...");let i=r.interpreter.interpret_str_web(a);console.log(i);const _=JSON.stringify(i,(E,$)=>typeof $=="bigint"?Number($):$);s.push(`${new Date().toLocaleTimeString()}: ${_}`),console.log(i),console.debug("Code executed successfully.")}catch(i){s.push(`${new Date().toLocaleTimeString()}: ${i}`)}n(1,s)}}const l=()=>n(0,a="");function c(i){a=i,n(0,a)}function f(i){s=i,n(1,s)}return t.$$set=i=>{"data"in i&&n(3,r=i.data)},[a,s,o,r,l,c,f]}class Be extends U{constructor(e){super(),W(this,e,Ue,Re,R,{data:3})}}export{Be as component,Ne as universal};