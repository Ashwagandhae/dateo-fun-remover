!function(){"use strict";function t(){}function n(t){return t()}function e(){return Object.create(null)}function o(t){t.forEach(n)}function u(t){return"function"==typeof t}function r(t,n){return t!=t?n==n:t!==n||t&&"object"==typeof t||"function"==typeof t}function i(t,n){t.appendChild(n)}function c(t,n,e){t.insertBefore(n,e||null)}function l(t){t.parentNode.removeChild(t)}function a(t){return document.createElement(t)}function s(t){return document.createTextNode(t)}function f(){return s(" ")}function d(t,n,e,o){return t.addEventListener(n,e,o),()=>t.removeEventListener(n,e,o)}function m(t,n,e){null==e?t.removeAttribute(n):t.getAttribute(n)!==e&&t.setAttribute(n,e)}function p(t){return""===t?null:+t}function h(t,n){n=""+n,t.wholeText!==n&&(t.data=n)}function g(t,n){t.value=null==n?"":n}let b;function $(t){b=t}const y=[],v=[],x=[],_=[],k=Promise.resolve();let w=!1;function C(t){x.push(t)}const S=new Set;let E=0;function N(){const t=b;do{for(;E<y.length;){const t=y[E];E++,$(t),D(t.$$)}for($(null),y.length=0,E=0;v.length;)v.pop()();for(let t=0;t<x.length;t+=1){const n=x[t];S.has(n)||(S.add(n),n())}x.length=0}while(y.length);for(;_.length;)_.pop()();w=!1,S.clear(),$(t)}function D(t){if(null!==t.fragment){t.update(),o(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(C)}}const A=new Set;function M(t,n){-1===t.$$.dirty[0]&&(y.push(t),w||(w=!0,k.then(N)),t.$$.dirty.fill(0)),t.$$.dirty[n/31|0]|=1<<n%31}function T(r,i,c,a,s,f,d,m=[-1]){const p=b;$(r);const h=r.$$={fragment:null,ctx:null,props:f,update:t,not_equal:s,bound:e(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(i.context||(p?p.$$.context:[])),callbacks:e(),dirty:m,skip_bound:!1,root:i.target||p.$$.root};d&&d(h.root);let g=!1;if(h.ctx=c?c(r,i.props||{},((t,n,...e)=>{const o=e.length?e[0]:n;return h.ctx&&s(h.ctx[t],h.ctx[t]=o)&&(!h.skip_bound&&h.bound[t]&&h.bound[t](o),g&&M(r,t)),n})):[],h.update(),g=!0,o(h.before_update),h.fragment=!!a&&a(h.ctx),i.target){if(i.hydrate){const t=function(t){return Array.from(t.childNodes)}(i.target);h.fragment&&h.fragment.l(t),t.forEach(l)}else h.fragment&&h.fragment.c();i.intro&&((y=r.$$.fragment)&&y.i&&(A.delete(y),y.i(v))),function(t,e,r,i){const{fragment:c,on_mount:l,on_destroy:a,after_update:s}=t.$$;c&&c.m(e,r),i||C((()=>{const e=l.map(n).filter(u);a?a.push(...e):o(e),t.$$.on_mount=[]})),s.forEach(C)}(r,i.target,i.anchor,i.customElement),N()}var y,v;$(p)}function j(t,n,e){const o=t.slice();return o[22]=n[e],o}function L(t){let n,e,u,r,i,s,h,b,$,y,v,x,_,k,w,C,S,E,N,D,A,M,T,j,L;return{c(){n=a("label"),n.textContent="Goal",e=f(),u=a("input"),r=f(),i=a("label"),i.textContent="Num1",s=f(),h=a("input"),b=f(),$=a("label"),$.textContent="Num2",y=f(),v=a("input"),x=f(),_=a("label"),_.textContent="Num3",k=f(),w=a("input"),C=f(),S=a("label"),S.textContent="Num4",E=f(),N=a("input"),D=f(),A=a("label"),A.textContent="Num5",M=f(),T=a("input"),m(n,"for","goal"),m(u,"type","number"),m(u,"id","goal"),m(i,"for","num1"),m(h,"type","number"),m(h,"id","num1"),m($,"for","num2"),m(v,"type","number"),m(v,"id","num2"),m(_,"for","num3"),m(w,"type","number"),m(w,"id","num3"),m(S,"for","num4"),m(N,"type","number"),m(N,"id","num4"),m(A,"for","num5"),m(T,"type","number"),m(T,"id","num5")},m(o,l){c(o,n,l),c(o,e,l),c(o,u,l),g(u,t[3]),c(o,r,l),c(o,i,l),c(o,s,l),c(o,h,l),g(h,t[4]),c(o,b,l),c(o,$,l),c(o,y,l),c(o,v,l),g(v,t[5]),c(o,x,l),c(o,_,l),c(o,k,l),c(o,w,l),g(w,t[6]),c(o,C,l),c(o,S,l),c(o,E,l),c(o,N,l),g(N,t[7]),c(o,D,l),c(o,A,l),c(o,M,l),c(o,T,l),g(T,t[8]),j||(L=[d(u,"input",t[13]),d(h,"input",t[14]),d(v,"input",t[15]),d(w,"input",t[16]),d(N,"input",t[17]),d(T,"input",t[18])],j=!0)},p(t,n){8&n&&p(u.value)!==t[3]&&g(u,t[3]),16&n&&p(h.value)!==t[4]&&g(h,t[4]),32&n&&p(v.value)!==t[5]&&g(v,t[5]),64&n&&p(w.value)!==t[6]&&g(w,t[6]),128&n&&p(N.value)!==t[7]&&g(N,t[7]),256&n&&p(T.value)!==t[8]&&g(T,t[8])},d(t){t&&l(n),t&&l(e),t&&l(u),t&&l(r),t&&l(i),t&&l(s),t&&l(h),t&&l(b),t&&l($),t&&l(y),t&&l(v),t&&l(x),t&&l(_),t&&l(k),t&&l(w),t&&l(C),t&&l(S),t&&l(E),t&&l(N),t&&l(D),t&&l(A),t&&l(M),t&&l(T),j=!1,o(L)}}}function O(t){let n,e,o;return{c(){n=a("input"),m(n,"type","date")},m(u,r){c(u,n,r),g(n,t[9]),e||(o=d(n,"input",t[12]),e=!0)},p(t,e){512&e&&g(n,t[9])},d(t){t&&l(n),e=!1,o()}}}function F(t){let n,e,o=t[22]+"";return{c(){n=a("li"),e=s(o)},m(t,o){c(t,n,o),i(n,e)},p(t,n){1&n&&o!==(o=t[22]+"")&&h(e,o)},d(t){t&&l(n)}}}function H(n){let e,u,r,p,g,b,$,y,v,x,_,k,w,C,S,E,N,D,A,M,T=n[1]?"Stop":"Start";function H(t,n){return t[2]?O:L}let q=H(n),B=q(n),G=n[0],I=[];for(let t=0;t<G.length;t+=1)I[t]=F(j(n,G,t));return{c(){e=a("h1"),e.textContent="Dateo Fun Remover!",u=f(),r=a("p"),r.innerHTML='Check out the <a href="http://dateo-math-game.com">for real fun.</a> This solver\n  was made in collaboration with Finn McKibbin.',p=f(),g=a("p"),g.innerHTML='<a href="https://github.com/Ashwagandhae/dateo-fun-remover">Source code</a>',b=f(),$=a("p"),y=f(),v=a("div"),x=a("button"),_=s(T),k=f(),w=a("label"),w.textContent="Use Date",C=f(),S=a("input"),E=f(),B.c(),N=f(),D=a("ul");for(let t=0;t<I.length;t+=1)I[t].c();m(w,"for","useDate"),m(S,"type","checkbox"),m(S,"id","useDate"),m(D,"class","content")},m(t,o){c(t,e,o),c(t,u,o),c(t,r,o),c(t,p,o),c(t,g,o),c(t,b,o),c(t,$,o),c(t,y,o),c(t,v,o),i(v,x),i(x,_),i(v,k),i(v,w),i(v,C),i(v,S),S.checked=n[2],i(v,E),B.m(v,null),i(v,N),i(v,D);for(let t=0;t<I.length;t+=1)I[t].m(D,null);A||(M=[d(x,"click",n[10]),d(S,"change",n[11])],A=!0)},p(t,[n]){if(2&n&&T!==(T=t[1]?"Stop":"Start")&&h(_,T),4&n&&(S.checked=t[2]),q===(q=H(t))&&B?B.p(t,n):(B.d(1),B=q(t),B&&(B.c(),B.m(v,N))),1&n){let e;for(G=t[0],e=0;e<G.length;e+=1){const o=j(t,G,e);I[e]?I[e].p(o,n):(I[e]=F(o),I[e].c(),I[e].m(D,null))}for(;e<I.length;e+=1)I[e].d(1);I.length=G.length}},i:t,o:t,d(t){t&&l(e),t&&l(u),t&&l(r),t&&l(p),t&&l(g),t&&l(b),t&&l($),t&&l(y),t&&l(v),B.d(),function(t,n){for(let e=0;e<t.length;e+=1)t[e]&&t[e].d(n)}(I,t),A=!1,o(M)}}}function q(t,n,e){let o,u=[],r=!1;let i=!0,c=1,[l,a,s,f,d]=[1,2,3,4,5],m=(new Date).toISOString().slice(0,10);return[u,r,i,c,l,a,s,f,d,m,function(){r?(e(1,r=!1),o.terminate()):(e(0,u=[]),e(1,r=!0),o=new Worker("./build/worker.js"),o.onmessage=({data:t})=>{"ready"===t.message&&o.postMessage({message:"start",useDate:i,goal:c,num1:l,num2:a,num3:s,num4:f,num5:d,date:m}),"solution"===t.message&&e(0,u=[...u,t.solution])})},function(){i=this.checked,e(2,i)},function(){m=this.value,e(9,m)},function(){c=p(this.value),e(3,c)},function(){l=p(this.value),e(4,l)},function(){a=p(this.value),e(5,a)},function(){s=p(this.value),e(6,s)},function(){f=p(this.value),e(7,f)},function(){d=p(this.value),e(8,d)}]}class B extends class{$destroy(){!function(t,n){const e=t.$$;null!==e.fragment&&(o(e.on_destroy),e.fragment&&e.fragment.d(n),e.on_destroy=e.fragment=null,e.ctx=[])}(this,1),this.$destroy=t}$on(t,n){const e=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return e.push(n),()=>{const t=e.indexOf(n);-1!==t&&e.splice(t,1)}}$set(t){var n;this.$$set&&(n=t,0!==Object.keys(n).length)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}{constructor(t){super(),T(this,t,q,H,r,{})}}(async()=>{new B({target:document.body})})()}();
//# sourceMappingURL=bundle.js.map
