import{s as B,f as v,g as w,h as E,d as g,j as d,A as xe,k as $,i as A,x as b,B as ie,y as D,a as I,C as N,c as L,D as pe,E as ve,F as ae,l as U,m as R,n as ee,H as ke,e as oe,G as Ee,I as ce,p as se,J as $e}from"../chunks/scheduler.90f3e356.js";import{S as F,i as q,f as Te,b as z,d as P,m as J,a as G,t as K,e as Q}from"../chunks/index.ce8eba6d.js";function O(n){return(n==null?void 0:n.length)!==void 0?n:Array.from(n)}let T;const we=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&we.decode();let V=null;function X(){return(V===null||V.byteLength===0)&&(V=new Uint8Array(T.memory.buffer)),V}function ue(n,e){return n=n>>>0,we.decode(X().subarray(n,n+e))}const C=new Array(128).fill(void 0);C.push(void 0,null,!0,!1);let W=C.length;function j(n){W===C.length&&C.push(C.length+1);const e=W;return W=C[e],C[e]=n,e}function Y(n){return C[n]}function Ie(n){n<132||(C[n]=W,W=n)}function re(n){const e=Y(n);return Ie(n),e}let le=0;const Z=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},Le=typeof Z.encodeInto=="function"?function(n,e){return Z.encodeInto(n,e)}:function(n,e){const t=Z.encode(n);return e.set(t),{read:n.length,written:t.length}};function Ae(n,e,t){if(t===void 0){const o=Z.encode(n),c=e(o.length,1)>>>0;return X().subarray(c,c+o.length).set(o),le=o.length,c}let s=n.length,r=e(s,1)>>>0;const i=X();let l=0;for(;l<s;l++){const o=n.charCodeAt(l);if(o>127)break;i[r+l]=o}if(l!==s){l!==0&&(n=n.slice(l)),r=t(r,s,s=l+n.length*3,1)>>>0;const o=X().subarray(r+l,r+s),c=Le(n,o);l+=c.written}return le=l,r}let H=null;function ne(){return(H===null||H.byteLength===0)&&(H=new Int32Array(T.memory.buffer)),H}function Ce(n,e){try{return n.apply(this,e)}catch(t){T.__wbindgen_exn_store(j(t))}}class te{static __wrap(e){e=e>>>0;const t=Object.create(te.prototype);return t.__wbg_ptr=e,t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();T.__wbg_interpreter_free(e)}static new(){const e=T.interpreter_new();return te.__wrap(e)}interpret_str_web(e){try{const i=T.__wbindgen_add_to_stack_pointer(-16),l=Ae(e,T.__wbindgen_malloc,T.__wbindgen_realloc),o=le;T.interpreter_interpret_str_web(i,this.__wbg_ptr,l,o);var t=ne()[i/4+0],s=ne()[i/4+1],r=ne()[i/4+2];if(r)throw re(s);return re(t)}finally{T.__wbindgen_add_to_stack_pointer(16)}}}async function De(n,e){if(typeof Response=="function"&&n instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(n,e)}catch(s){if(n.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",s);else throw s}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}else{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}function Me(){const n={};return n.wbg={},n.wbg.__wbindgen_string_new=function(e,t){const s=ue(e,t);return j(s)},n.wbg.__wbindgen_bigint_from_i64=function(e){return j(e)},n.wbg.__wbindgen_number_new=function(e){return j(e)},n.wbg.__wbindgen_object_drop_ref=function(e){re(e)},n.wbg.__wbg_new_b51585de1b234aff=function(){const e=new Object;return j(e)},n.wbg.__wbg_set_092e06b0f9d71865=function(){return Ce(function(e,t,s){return Reflect.set(Y(e),Y(t),Y(s))},arguments)},n.wbg.__wbindgen_throw=function(e,t){throw new Error(ue(e,t))},n}function Oe(n,e){return T=n.exports,ye.__wbindgen_wasm_module=e,H=null,V=null,T}async function ye(n){if(T!==void 0)return T;typeof n>"u"&&(n=new URL(""+new URL("../assets/interpreter_bg.066519bb.wasm",import.meta.url).href,self.location));const e=Me();(typeof n=="string"||typeof Request=="function"&&n instanceof Request||typeof URL=="function"&&n instanceof URL)&&(n=fetch(n));const{instance:t,module:s}=await De(await n,e);return Oe(t,s)}const Se=async()=>(await ye(),{interpreter:te.new()}),Ze=Object.freeze(Object.defineProperty({__proto__:null,load:Se},Symbol.toStringTag,{value:"Module"}));function Ve(n){let e,t,s,r;return{c(){e=v("div"),t=v("div"),this.h()},l(i){e=w(i,"DIV",{class:!0,style:!0});var l=E(e);t=w(l,"DIV",{contenteditable:!0,class:!0,placeholder:!0}),E(t).forEach(g),l.forEach(g),this.h()},h(){d(t,"contenteditable","true"),d(t,"class","code-editor svelte-1kx1mxy"),d(t,"placeholder","Insert code here..."),n[0]===void 0&&xe(()=>n[1].call(t)),d(e,"class","d-flex text-bg-dark w-100 px-1 pt-2"),$(e,"height","85%")},m(i,l){A(i,e,l),b(e,t),n[0]!==void 0&&(t.innerText=n[0]),s||(r=ie(t,"input",n[1]),s=!0)},p(i,[l]){l&1&&i[0]!==t.innerText&&(t.innerText=i[0])},i:D,o:D,d(i){i&&g(e),s=!1,r()}}}function je(n,e,t){let{source:s=`type Point = {x: int, y: int}

	Point {x: 1, y: 2}`}=e;function r(){s=this.innerText,t(0,s)}return n.$$set=i=>{"source"in i&&t(0,s=i.source)},[s,r]}class He extends F{constructor(e){super(),q(this,e,je,Ve,B,{source:0})}}function Ue(n){let e,t,s,r="Focus Lang",i,l,o,c,_='<i class="bi bi-play fs-2"></i>',a,p;return{c(){e=v("nav"),t=v("div"),s=v("a"),s.textContent=r,i=I(),l=v("ul"),o=v("li"),c=v("button"),c.innerHTML=_,this.h()},l(u){e=w(u,"NAV",{class:!0});var h=E(e);t=w(h,"DIV",{class:!0});var x=E(t);s=w(x,"A",{class:!0,href:!0,style:!0,"data-svelte-h":!0}),N(s)!=="svelte-113u1yy"&&(s.textContent=r),i=L(x),l=w(x,"UL",{class:!0});var m=E(l);o=w(m,"LI",{class:!0});var f=E(o);c=w(f,"BUTTON",{class:!0,"aria-current":!0,"data-svelte-h":!0}),N(c)!=="svelte-i92bm7"&&(c.innerHTML=_),f.forEach(g),m.forEach(g),x.forEach(g),h.forEach(g),this.h()},h(){d(s,"class","navbar-brand"),d(s,"href","/"),$(s,"margin-left","100px"),d(c,"class","nav-link rounded-0"),d(c,"aria-current","page"),d(o,"class","nav-item"),d(l,"class","nav nav-pills mb-auto text-center mr-0 ml-auto"),d(t,"class","container-fluid"),d(e,"class","navbar navbar-expand-lg bg-body-tertiary")},m(u,h){A(u,e,h),b(e,t),b(t,s),b(t,i),b(t,l),b(l,o),b(o,c),a||(p=ie(c,"click",pe(n[1])),a=!0)},p:D,i:D,o:D,d(u){u&&g(e),a=!1,p()}}}function Re(n){const e=ve();return[e,()=>e("run")]}class We extends F{constructor(e){super(),q(this,e,Re,Ue,B,{})}}function de(n,e,t){const s=n.slice();return s[2]=e[t],s}function fe(n){let e,t=n[2].timestamp.toLocaleTimeString()+"",s,r,i=n[2].message+"",l,o;return{c(){e=v("div"),s=U(t),r=U(": "),l=U(i),o=I(),this.h()},l(c){e=w(c,"DIV",{class:!0});var _=E(e);s=R(_,t),r=R(_,": "),l=R(_,i),o=L(_),_.forEach(g),this.h()},h(){d(e,"class","border-bottom border-secondary w-100 bg-dark")},m(c,_){A(c,e,_),b(e,s),b(e,r),b(e,l),b(e,o)},p(c,_){_&1&&t!==(t=c[2].timestamp.toLocaleTimeString()+"")&&ee(s,t),_&1&&i!==(i=c[2].message+"")&&ee(l,i)},d(c){c&&g(e)}}}function Ne(n){let e,t,s='<div class="mx-1">Output</div>',r,i,l=O(n[0]),o=[];for(let c=0;c<l.length;c+=1)o[c]=fe(de(n,l,c));return{c(){e=v("div"),t=v("span"),t.innerHTML=s,r=I(),i=v("div");for(let c=0;c<o.length;c+=1)o[c].c();this.h()},l(c){e=w(c,"DIV",{class:!0,style:!0});var _=E(e);t=w(_,"SPAN",{class:!0,"data-bs-theme":!0,"data-svelte-h":!0}),N(t)!=="svelte-19dnlzb"&&(t.innerHTML=s),r=L(_),i=w(_,"DIV",{class:!0,contenteditable:!0});var a=E(i);for(let p=0;p<o.length;p+=1)o[p].l(a);a.forEach(g),_.forEach(g),this.h()},h(){d(t,"class","bg-body-tertiary bg-dark"),d(t,"data-bs-theme","dark"),d(i,"class","output-text-area svelte-7kqvvn"),d(i,"contenteditable","false"),d(e,"class","d-flex flex-column text-bg-dark w-100"),$(e,"height","30%")},m(c,_){A(c,e,_),b(e,t),b(e,r),b(e,i);for(let a=0;a<o.length;a+=1)o[a]&&o[a].m(i,null)},p(c,[_]){if(_&1){l=O(c[0]);let a;for(a=0;a<l.length;a+=1){const p=de(c,l,a);o[a]?o[a].p(p,_):(o[a]=fe(p),o[a].c(),o[a].m(i,null))}for(;a<o.length;a+=1)o[a].d(1);o.length=l.length}},i:D,o:D,d(c){c&&g(e),ae(o,c)}}}function Be(n,e,t){let{output:s=[]}=e;function r(i){s.push({timestamp:new Date,message:i}),t(0,s)}return n.$$set=i=>{"output"in i&&t(0,s=i.output)},[s,r]}class Fe extends F{constructor(e){super(),q(this,e,Be,Ne,B,{output:0,log:1})}get log(){return this.$$.ctx[1]}}function _e(n,e,t){const s=n.slice();return s[5]=e[t],s[7]=t,s}function he(n,e,t){const s=n.slice();return s[8]=e[t],s}function qe(n){let e,t;return{c(){e=new ke(!1),t=oe(),this.h()},l(s){e=Ee(s,!1),t=oe(),this.h()},h(){e.a=t},m(s,r){e.m(n[0],s,r),A(s,t,r)},p(s,r){r&1&&e.p(s[0])},d(s){s&&(g(t),e.d())}}}function ze(n){let e,t=O(n[1]),s=[];for(let r=0;r<t.length;r+=1)s[r]=ge(_e(n,t,r));return{c(){e=v("ul");for(let r=0;r<s.length;r+=1)s[r].c();this.h()},l(r){e=w(r,"UL",{class:!0});var i=E(e);for(let l=0;l<s.length;l+=1)s[l].l(i);i.forEach(g),this.h()},h(){d(e,"class","nav nav-pills flex-column mb-auto")},m(r,i){A(r,e,i);for(let l=0;l<s.length;l+=1)s[l]&&s[l].m(e,null)},p(r,i){if(i&6){t=O(r[1]);let l;for(l=0;l<t.length;l+=1){const o=_e(r,t,l);s[l]?s[l].p(o,i):(s[l]=ge(o),s[l].c(),s[l].m(e,null))}for(;l<s.length;l+=1)s[l].d(1);s.length=t.length}},d(r){r&&g(e),ae(s,r)}}}function me(n){let e,t;return{c(){e=v("i"),this.h()},l(s){e=w(s,"I",{class:!0,style:!0}),E(e).forEach(g),this.h()},h(){d(e,"class",t=ce(`${n[5].icon}`)+" svelte-1ikk9me"),$(e,"margin-right","4px")},m(s,r){A(s,e,r)},p(s,r){r&2&&t!==(t=ce(`${s[5].icon}`)+" svelte-1ikk9me")&&d(e,"class",t)},d(s){s&&g(e)}}}function be(n){let e,t,s=n[8].title+"",r,i,l,o,c;function _(){return n[4](n[8])}return{c(){e=v("li"),t=v("a"),r=U(s),l=I(),this.h()},l(a){e=w(a,"LI",{});var p=E(e);t=w(p,"A",{id:!0,href:!0,class:!0});var u=E(t);r=R(u,s),u.forEach(g),l=L(p),p.forEach(g),this.h()},h(){d(t,"id",i=n[8].id),d(t,"href","/"),d(t,"class","link-body-emphasis d-inline-flex text-decoration-none rounded svelte-1ikk9me")},m(a,p){A(a,e,p),b(e,t),b(t,r),b(e,l),o||(c=ie(t,"click",pe(_)),o=!0)},p(a,p){n=a,p&2&&s!==(s=n[8].title+"")&&ee(r,s),p&2&&i!==(i=n[8].id)&&d(t,"id",i)},d(a){a&&g(e),o=!1,c()}}}function ge(n){let e,t,s,r=n[5].title+"",i,l,o,c,_,a=n[5].icon&&me(n),p=O(n[5].items),u=[];for(let h=0;h<p.length;h+=1)u[h]=be(he(n,p,h));return{c(){e=v("li"),t=v("a"),a&&a.c(),s=I(),i=U(r),l=I(),o=v("div"),c=v("ul");for(let h=0;h<u.length;h+=1)u[h].c();_=I(),this.h()},l(h){e=w(h,"LI",{class:!0});var x=E(e);t=w(x,"A",{href:!0,class:!0,type:!0,"data-bs-toggle":!0,"data-bs-target":!0,"aria-expanded":!0,"aria-controls":!0});var m=E(t);a&&a.l(m),s=L(m),i=R(m,r),m.forEach(g),l=L(x),o=w(x,"DIV",{class:!0,id:!0});var f=E(o);c=w(f,"UL",{class:!0});var y=E(c);for(let k=0;k<u.length;k+=1)u[k].l(y);y.forEach(g),f.forEach(g),_=L(x),x.forEach(g),this.h()},h(){d(t,"href","/"),d(t,"class","d-flex mb-3 mb-md-0 me-md-auto text-white text-decoration-none"),d(t,"type","button"),d(t,"data-bs-toggle","collapse"),d(t,"data-bs-target",`#_menu-item-${n[7]}`),d(t,"aria-expanded","false"),d(t,"aria-controls",`_menu-item-${n[7]}`),d(c,"class","btn-toggle-nav list-unstyled fw-normal pb-1 small svelte-1ikk9me"),d(o,"class","collapse"),d(o,"id",`_menu-item-${n[7]}`),d(e,"class","nav-item mb-2")},m(h,x){A(h,e,x),b(e,t),a&&a.m(t,null),b(t,s),b(t,i),b(e,l),b(e,o),b(o,c);for(let m=0;m<u.length;m+=1)u[m]&&u[m].m(c,null);b(e,_)},p(h,x){if(h[5].icon?a?a.p(h,x):(a=me(h),a.c(),a.m(t,s)):a&&(a.d(1),a=null),x&2&&r!==(r=h[5].title+"")&&ee(i,r),x&6){p=O(h[5].items);let m;for(m=0;m<p.length;m+=1){const f=he(h,p,m);u[m]?u[m].p(f,x):(u[m]=be(f),u[m].c(),u[m].m(c,null))}for(;m<u.length;m+=1)u[m].d(1);u.length=p.length}},d(h){h&&g(e),a&&a.d(),ae(u,h)}}}function Pe(n){let e,t,s,r,i='<span class="fs-4">Menu</span>',l,o,c,_,a,p='<i class="bi bi-list fs-2"></i>',u,h;function x(y,k){return y[0]?qe:ze}let m=x(n),f=m(n);return{c(){e=v("div"),t=v("div"),s=v("div"),r=v("a"),r.innerHTML=i,l=I(),o=v("hr"),c=I(),f.c(),_=I(),a=v("button"),a.innerHTML=p,u=I(),h=v("div"),this.h()},l(y){e=w(y,"DIV",{class:!0});var k=E(e);t=w(k,"DIV",{class:!0,id:!0});var S=E(t);s=w(S,"DIV",{class:!0,style:!0});var M=E(s);r=w(M,"A",{href:!0,class:!0,"data-svelte-h":!0}),N(r)!=="svelte-1a5mwyw"&&(r.innerHTML=i),l=L(M),o=w(M,"HR",{}),c=L(M),f.l(M),M.forEach(g),S.forEach(g),_=L(k),a=w(k,"BUTTON",{class:!0,style:!0,type:!0,"data-bs-toggle":!0,"data-bs-target":!0,"aria-expanded":!0,"aria-controls":!0,"data-svelte-h":!0}),N(a)!=="svelte-8w5qwj"&&(a.innerHTML=p),k.forEach(g),u=L(y),h=w(y,"DIV",{class:!0}),E(h).forEach(g),this.h()},h(){d(r,"href","/"),d(r,"class","d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white text-decoration-none"),d(s,"class","flex-shrink-0 p-3"),$(s,"width","320px"),d(t,"class","collapse collapse-horizontal show"),d(t,"id","collapseExample"),d(a,"class","btn btn-secondary position-absolute translate-end badge"),$(a,"width","80px"),$(a,"z-index","9999"),$(a,"height","50px"),$(a,"left","100%"),$(a,"top","15px"),$(a,"margin-left","15px"),d(a,"type","button"),d(a,"data-bs-toggle","collapse"),d(a,"data-bs-target","#collapseExample"),d(a,"aria-expanded","false"),d(a,"aria-controls","collapseExample"),d(e,"class","position-relative"),d(h,"class","divider bg-primary svelte-1ikk9me")},m(y,k){A(y,e,k),b(e,t),b(t,s),b(s,r),b(s,l),b(s,o),b(s,c),f.m(s,null),b(e,_),b(e,a),A(y,u,k),A(y,h,k)},p(y,[k]){m===(m=x(y))&&f?f.p(y,k):(f.d(1),f=m(y),f&&(f.c(),f.m(s,null)))},i:D,o:D,d(y){y&&(g(e),g(u),g(h)),f.d()}}}function Je(n,e,t){let{menu:s}=e,{content:r=void 0}=e;const i=ve();function l(c){t(0,r=c)}const o=c=>i("selected",c.id);return n.$$set=c=>{"menu"in c&&t(1,s=c.menu),"content"in c&&t(0,r=c.content)},[r,s,i,l,o]}class Ge extends F{constructor(e){super(),q(this,e,Je,Pe,B,{menu:1,content:0,dispatcher:2,setContent:3})}get dispatcher(){return this.$$.ctx[2]}get setContent(){return this.$$.ctx[3]}}function Ke(n){let e,t,s,r,i,l,o,c,_,a,p,u={menu:[{title:"Tutorials",icon:"bi bi-book",items:[{title:"01. Basics",id:"tutorial-01"}]},{title:"Examples",icon:"bi bi-journals",items:[{title:"01. Fibonnachi",id:"example-01"}]}]};t=new Ge({props:u}),n[5](t),t.$on("selected",n[6]),i=new We({}),i.$on("run",n[3]);function h(f){n[7](f)}let x={};n[2]!==void 0&&(x.source=n[2]),o=new He({props:x}),se.push(()=>Te(o,"source",h));let m={};return a=new Fe({props:m}),n[8](a),{c(){e=v("main"),z(t.$$.fragment),s=I(),r=v("div"),z(i.$$.fragment),l=I(),z(o.$$.fragment),_=I(),z(a.$$.fragment),this.h()},l(f){e=w(f,"MAIN",{class:!0,style:!0});var y=E(e);P(t.$$.fragment,y),s=L(y),r=w(y,"DIV",{class:!0});var k=E(r);P(i.$$.fragment,k),l=L(k),P(o.$$.fragment,k),_=L(k),P(a.$$.fragment,k),k.forEach(g),y.forEach(g),this.h()},h(){d(r,"class","d-flex flex-column w-100"),d(e,"class","d-flex flex-nowrap"),$(e,"height","100vh"),$(e,"max-height","100vh"),$(e,"overflow-x","auto"),$(e,"overflow-y","hidden")},m(f,y){A(f,e,y),J(t,e,null),b(e,s),b(e,r),J(i,r,null),b(r,l),J(o,r,null),b(r,_),J(a,r,null),p=!0},p(f,[y]){const k={};t.$set(k);const S={};!c&&y&4&&(c=!0,S.source=f[2],$e(()=>c=!1)),o.$set(S);const M={};a.$set(M)},i(f){p||(G(t.$$.fragment,f),G(i.$$.fragment,f),G(o.$$.fragment,f),G(a.$$.fragment,f),p=!0)},o(f){K(t.$$.fragment,f),K(i.$$.fragment,f),K(o.$$.fragment,f),K(a.$$.fragment,f),p=!1},d(f){f&&g(e),n[5](null),Q(t),Q(i),Q(o),n[8](null),Q(a)}}}function Qe(n,e,t){let{data:s}=e,r,i,l;function o(){if(l)try{console.debug("Executing code...");let u=s.interpreter.interpret_str_web(l);console.log(u);const h=JSON.stringify(u,(x,m)=>typeof m=="bigint"?Number(m):m);r.log(h),console.debug("Code executed successfully.")}catch(u){r.log(u)}}function c(u){se[u?"unshift":"push"](()=>{i=u,t(1,i)})}const _=u=>{i.setContent(`<h1>${u.detail}</h1>`)};function a(u){l=u,t(2,l)}function p(u){se[u?"unshift":"push"](()=>{r=u,t(0,r)})}return n.$$set=u=>{"data"in u&&t(4,s=u.data)},[r,i,l,o,s,c,_,a,p]}class et extends F{constructor(e){super(),q(this,e,Qe,Ke,B,{data:4})}}export{et as component,Ze as universal};