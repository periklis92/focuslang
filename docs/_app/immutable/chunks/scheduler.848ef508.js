function S(){}function C(t,n){for(const e in n)t[e]=n[e];return t}function M(t){return t()}function Q(){return Object.create(null)}function P(t){t.forEach(M)}function R(t){return typeof t=="function"}function W(t,n){return t!=t?n==n:t!==n||t&&typeof t=="object"||typeof t=="function"}function U(t){return Object.keys(t).length===0}function B(t,...n){if(t==null){for(const i of n)i(void 0);return S}const e=t.subscribe(...n);return e.unsubscribe?()=>e.unsubscribe():e}function V(t,n,e){t.$$.on_destroy.push(B(n,e))}function X(t,n,e,i){if(t){const r=w(t,n,e,i);return t[0](r)}}function w(t,n,e,i){return t[1]&&i?C(e.ctx.slice(),t[1](i(n))):e.ctx}function Y(t,n,e,i){if(t[2]&&i){const r=t[2](i(e));if(n.dirty===void 0)return r;if(typeof r=="object"){const a=[],c=Math.max(n.dirty.length,r.length);for(let u=0;u<c;u+=1)a[u]=n.dirty[u]|r[u];return a}return n.dirty|r}return n.dirty}function Z(t,n,e,i,r,a){if(r){const c=w(n,e,i,a);t.p(c,r)}}function $(t){if(t.ctx.length>32){const n=[],e=t.ctx.length/32;for(let i=0;i<e;i++)n[i]=-1;return n}return-1}let m=!1;function tt(){m=!0}function nt(){m=!1}function D(t,n,e,i){for(;t<n;){const r=t+(n-t>>1);e(r)<=i?t=r+1:n=r}return t}function L(t){if(t.hydrate_init)return;t.hydrate_init=!0;let n=t.childNodes;if(t.nodeName==="HEAD"){const l=[];for(let s=0;s<n.length;s++){const o=n[s];o.claim_order!==void 0&&l.push(o)}n=l}const e=new Int32Array(n.length+1),i=new Int32Array(n.length);e[0]=-1;let r=0;for(let l=0;l<n.length;l++){const s=n[l].claim_order,o=(r>0&&n[e[r]].claim_order<=s?r+1:D(1,r,j=>n[e[j]].claim_order,s))-1;i[l]=e[o]+1;const k=o+1;e[k]=l,r=Math.max(k,r)}const a=[],c=[];let u=n.length-1;for(let l=e[r]+1;l!=0;l=i[l-1]){for(a.push(n[l-1]);u>=l;u--)c.push(n[u]);u--}for(;u>=0;u--)c.push(n[u]);a.reverse(),c.sort((l,s)=>l.claim_order-s.claim_order);for(let l=0,s=0;l<c.length;l++){for(;s<a.length&&c[l].claim_order>=a[s].claim_order;)s++;const o=s<a.length?a[s]:null;t.insertBefore(c[l],o)}}function O(t,n){if(m){for(L(t),(t.actual_end_child===void 0||t.actual_end_child!==null&&t.actual_end_child.parentNode!==t)&&(t.actual_end_child=t.firstChild);t.actual_end_child!==null&&t.actual_end_child.claim_order===void 0;)t.actual_end_child=t.actual_end_child.nextSibling;n!==t.actual_end_child?(n.claim_order!==void 0||n.parentNode!==t)&&t.insertBefore(n,t.actual_end_child):t.actual_end_child=n.nextSibling}else(n.parentNode!==t||n.nextSibling!==null)&&t.appendChild(n)}function et(t,n,e){m&&!e?O(t,n):(n.parentNode!==t||n.nextSibling!=e)&&t.insertBefore(n,e||null)}function it(t){t.parentNode&&t.parentNode.removeChild(t)}function q(t){return document.createElement(t)}function g(t){return document.createTextNode(t)}function rt(){return g(" ")}function ct(){return g("")}function lt(t,n,e,i){return t.addEventListener(n,e,i),()=>t.removeEventListener(n,e,i)}function ut(t){return function(n){return n.preventDefault(),t.call(this,n)}}function at(t,n,e){e==null?t.removeAttribute(n):t.getAttribute(n)!==e&&t.setAttribute(n,e)}function st(t){return t.dataset.svelteH}function ot(t){return Array.from(t.childNodes)}function H(t){t.claim_info===void 0&&(t.claim_info={last_index:0,total_claimed:0})}function N(t,n,e,i,r=!1){H(t);const a=(()=>{for(let c=t.claim_info.last_index;c<t.length;c++){const u=t[c];if(n(u)){const l=e(u);return l===void 0?t.splice(c,1):t[c]=l,r||(t.claim_info.last_index=c),u}}for(let c=t.claim_info.last_index-1;c>=0;c--){const u=t[c];if(n(u)){const l=e(u);return l===void 0?t.splice(c,1):t[c]=l,r?l===void 0&&t.claim_info.last_index--:t.claim_info.last_index=c,u}}return i()})();return a.claim_order=t.claim_info.total_claimed,t.claim_info.total_claimed+=1,a}function T(t,n,e,i){return N(t,r=>r.nodeName===n,r=>{const a=[];for(let c=0;c<r.attributes.length;c++){const u=r.attributes[c];e[u.name]||a.push(u.name)}a.forEach(c=>r.removeAttribute(c))},()=>i(n))}function ft(t,n,e){return T(t,n,e,q)}function z(t,n){return N(t,e=>e.nodeType===3,e=>{const i=""+n;if(e.data.startsWith(i)){if(e.data.length!==i.length)return e.splitText(i.length)}else e.data=i},()=>g(n),!0)}function _t(t){return z(t," ")}function dt(t,n){n=""+n,t.data!==n&&(t.data=n)}function ht(t,n,e,i){e==null?t.style.removeProperty(n):t.style.setProperty(n,e,i?"important":"")}function F(t,n,{bubbles:e=!1,cancelable:i=!1}={}){return new CustomEvent(t,{detail:n,bubbles:e,cancelable:i})}function mt(t,n){return new t(n)}let h;function p(t){h=t}function v(){if(!h)throw new Error("Function called outside component initialization");return h}function pt(t){v().$$.on_mount.push(t)}function bt(t){v().$$.after_update.push(t)}function yt(){const t=v();return(n,e,{cancelable:i=!1}={})=>{const r=t.$$.callbacks[n];if(r){const a=F(n,e,{cancelable:i});return r.slice().forEach(c=>{c.call(t,a)}),!a.defaultPrevented}return!0}}const d=[],E=[];let _=[];const y=[],A=Promise.resolve();let x=!1;function I(){x||(x=!0,A.then(J))}function xt(){return I(),A}function G(t){_.push(t)}function gt(t){y.push(t)}const b=new Set;let f=0;function J(){if(f!==0)return;const t=h;do{try{for(;f<d.length;){const n=d[f];f++,p(n),K(n.$$)}}catch(n){throw d.length=0,f=0,n}for(p(null),d.length=0,f=0;E.length;)E.pop()();for(let n=0;n<_.length;n+=1){const e=_[n];b.has(e)||(b.add(e),e())}_.length=0}while(d.length);for(;y.length;)y.pop()();x=!1,b.clear(),p(t)}function K(t){if(t.fragment!==null){t.update(),P(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(G)}}function vt(t){const n=[],e=[];_.forEach(i=>t.indexOf(i)===-1?n.push(i):e.push(i)),e.forEach(i=>i()),_=n}export{G as A,lt as B,st as C,ut as D,P as E,yt as F,gt as G,Q as H,J as I,R as J,U as K,vt as L,h as M,p as N,M as O,d as P,I as Q,tt as R,nt as S,rt as a,bt as b,_t as c,it as d,ct as e,q as f,ft as g,ot as h,et as i,at as j,ht as k,g as l,z as m,dt as n,pt as o,E as p,mt as q,X as r,W as s,xt as t,Z as u,$ as v,Y as w,O as x,S as y,V as z};