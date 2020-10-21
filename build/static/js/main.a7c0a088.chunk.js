(this.webpackJsonpfrontend=this.webpackJsonpfrontend||[]).push([[0],{21:function(e,n,t){},24:function(e,n,t){e.exports=t(35)},35:function(e,n,t){"use strict";t.r(n);var a,r=t(0),l=t.n(r),i=t(11),o=t.n(i),c=t(3),s=t(2),u=t(4),p=t(1),m=t(9);!function(e){e[e.loggedOut=0]="loggedOut",e[e.loggedIn=1]="loggedIn",e[e.registered=2]="registered",e[e.challengeReady=3]="challengeReady"}(a||(a={}));var b,d={value:a.challengeReady},h=Object(m.b)({name:"auth",initialState:d,reducers:{setLoggedIn:function(e){e.value=a.loggedIn},setLoggedOut:function(e){e.value=a.loggedOut},setRegistered:function(e){e.value=a.registered},setChallengeReady:function(e){e.value=a.challengeReady}}}),g=h.actions,f=g.setLoggedIn,v=g.setLoggedOut,E=g.setRegistered,y=(g.setChallengeReady,function(e){return e.authenticator.value}),T=(h.reducer,Object(m.b)({name:"startTime",initialState:{value:null},reducers:{set_start_time:function(e,n){e.value=n.payload}}})),w=T.actions.set_start_time,k=function(e){return e.startTimesetter.value},O=(T.reducer,function(e){var n=e.initTimeinSeconds,t=e.initTimeinMinutes,a=Object(r.useState)(n),i=Object(u.a)(a,2),o=i[0],c=i[1],s=Object(r.useState)(t),m=Object(u.a)(s,2),b=m[0],d=m[1],h=Object(p.g)();Object(r.useEffect)((function(){var e=setTimeout((function(){return g()}),1e3);return function(){return clearTimeout(e)}}));var g=function(){1===o&&0===b&&h.push("/logout"),0===o?(c(59),d(b-1)):c(o-1)};return l.a.createElement("div",null,S(b),":",S(o))}),S=function(e){return e<10?"0"+e.toString():e.toString()},j=function(){var e=Object(s.c)(y),n=Object(s.c)(k);if(e!==a.challengeReady)return l.a.createElement("div",null);if(null===n)return console.error("can't mount timer,\n        startTime yet to be received from API. startTime: ".concat(n)),l.a.createElement("div",null);var t=n+1500-Math.floor(Date.now()/1e3),r=t%60,i=(t-r)/60;return l.a.createElement(O,{initTimeinMinutes:i,initTimeinSeconds:r})},N=function(){return l.a.createElement("div",{dangerouslySetInnerHTML:{__html:'<p class="ascii-art">\u2588\u2588\u2588\u2557&nbsp&nbsp&nbsp\u2588\u2588\u2557\u2588\u2588\u2557&nbsp&nbsp&nbsp\u2588\u2588\u2557\u2588\u2588\u2557&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2557&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2588\u2588\u2588\u2588\u2557\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2557\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2557<br /> \u2588\u2588\u2588\u2588\u2557&nbsp&nbsp\u2588\u2588\u2551\u2588\u2588\u2551&nbsp&nbsp&nbsp\u2588\u2588\u2551\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2554\u2550\u2550\u2550\u2550\u255d\u255a\u2550\u2550\u2588\u2588\u2554\u2550\u2550\u255d\u2588\u2588\u2554\u2550\u2550\u2550\u2550\u255d<br /> \u2588\u2588\u2554\u2588\u2588\u2557&nbsp\u2588\u2588\u2551\u2588\u2588\u2551&nbsp&nbsp&nbsp\u2588\u2588\u2551\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp\u2588\u2588\u2588\u2588\u2588\u2557&nbsp&nbsp<br /> \u2588\u2588\u2551\u255a\u2588\u2588\u2557\u2588\u2588\u2551\u2588\u2588\u2551&nbsp&nbsp&nbsp\u2588\u2588\u2551\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp\u2588\u2588\u2554\u2550\u2550\u255d&nbsp&nbsp<br /> \u2588\u2588\u2551&nbsp\u255a\u2588\u2588\u2588\u2588\u2551\u255a\u2588\u2588\u2588\u2588\u2588\u2588\u2554\u255d\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2557\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2557&nbsp&nbsp&nbsp&nbsp\u255a\u2588\u2588\u2588\u2588\u2588\u2588\u2557&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp\u2588\u2588\u2551&nbsp&nbsp&nbsp&nbsp&nbsp<br /> \u255a\u2550\u255d&nbsp&nbsp\u255a\u2550\u2550\u2550\u255d&nbsp\u255a\u2550\u2550\u2550\u2550\u2550\u255d&nbsp\u255a\u2550\u2550\u2550\u2550\u2550\u2550\u255d\u255a\u2550\u2550\u2550\u2550\u2550\u2550\u255d&nbsp&nbsp&nbsp&nbsp&nbsp\u255a\u2550\u2550\u2550\u2550\u2550\u255d&nbsp&nbsp&nbsp\u255a\u2550\u255d&nbsp&nbsp&nbsp\u255a\u2550\u255d&nbsp&nbsp&nbsp&nbsp&nbsp<br />'}})},C=function(){return l.a.createElement("div",{className:"mainheader"},l.a.createElement(c.b,{to:"/"},l.a.createElement(N,null)),l.a.createElement("div",{className:"filler"}),l.a.createElement("div",{className:"timer"},l.a.createElement(j,null)))},I=function(){return l.a.createElement("li",null,l.a.createElement(c.c,{to:"/challenges"},"/challenges"))},_=function(){return l.a.createElement("li",null,l.a.createElement(c.c,{to:"/leaderboard"},"/leaderboard"))},x=function(){return l.a.createElement("li",null,l.a.createElement(c.c,{to:"/logout"},"/logout"))},R=function(){return l.a.createElement("li",null,l.a.createElement(c.c,{to:"/login"},"/login"))},B=function(){return Object(s.c)(y)===a.challengeReady?l.a.createElement("ul",{className:"navbar"},l.a.createElement(I,null),l.a.createElement(_,null),l.a.createElement(x,null)):l.a.createElement("ul",{className:"navbar"},l.a.createElement(R,null))},F=function(e){var n=e.children;return l.a.createElement("div",{className:"menuoptions"},n)};!function(e){e[e.big=0]="big",e[e.small=1]="small",e[e.mobile=2]="mobile",e[e.loading=3]="loading"}(b||(b={}));var A,L=function(e){var n=e.children;return e.size===b.loading?l.a.createElement("div",{className:"menuouter menuouterloading"},n):l.a.createElement("div",{className:"menuouter"},n)},D=function(e){var n=e.location;return Object(r.useEffect)((function(){!function(){var e=n+" | Null CTF";document.title=e}()})),l.a.createElement("div",null)},H=function(){return l.a.createElement("div",null,l.a.createElement("div",{className:"menubottomborder"}))},W=function(){return l.a.createElement("div",null,l.a.createElement("div",{className:"menutopborder"}))},z=function(){return l.a.createElement("div",null,l.a.createElement(W,null),l.a.createElement(H,null))},G=function(e){var n=e.title;return l.a.createElement("div",{className:"menuheader"},l.a.createElement("span",null,n),l.a.createElement(z,null))},q=function(e){var n=e.text;return l.a.createElement("div",{className:"menutoptext"},n)};!function(e){e[e.big=0]="big",e[e.small=1]="small",e[e.mobile=2]="mobile",e[e.loading=3]="loading"}(A||(A={}));var J=function(e){var n=e.menuTitle,t=e.pageTitle,a=e.topText,r=e.size,i=e.children,o="menudialog";return r===A.big&&(o+=" menudialogbig"),r===A.mobile&&(o+=" menudialogmobile"),r===A.loading&&(o+=" menudialogloading"),l.a.createElement("div",{className:o},l.a.createElement(L,{size:r},l.a.createElement(G,{title:n}),l.a.createElement(q,{text:a}),l.a.createElement(D,{location:t}),i))},M=function(){return l.a.createElement(J,{size:A.mobile,menuTitle:"Error",topText:"Mobile devices are not supported. We regret the inconvenience caused.",pageTitle:"Error"},l.a.createElement(F,null,l.a.createElement("p",null,"This CTF relies heavily on the functionality provided by browsers on desktop computers so please visit us from a desktop computer")))},U=function(){try{var e=localStorage.getItem("state");if(null==e)return;return JSON.parse(e)}catch(n){return}}(),K=Object(m.a)({reducer:{authenticator:h.reducer,startTimesetter:T.reducer},preloadedState:U}),P=function(){return Object(s.b)()},V=t(18),Y=t.n(V),Q=t(23),Z={GET_CHALLENGES:"/api/get-challenges",LOGIN:"/api/login",REGISTER:"/api/register",LOGOUT:"/api/logout",CHECK_RESPONSE:"/api/check-response",LEADERBOARD:"/api/leaderboard",GET_STATE:"/api/get-state"},$=function(){var e=Object(Q.a)(Y.a.mark((function e(n){return Y.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.abrupt("return",fetch(Z.LOGIN,{method:"post",headers:{"Content-type":"application/json; charset=UTF-8"},body:JSON.stringify(n)}).then((function(e){return e.json()})));case 1:case"end":return e.stop()}}),e)})));return function(n){return e.apply(this,arguments)}}(),X=function(e){return!e.replace(/\s/g,"").length},ee=(t(21),function(e){var n=e.label,t=e.required,a=e.autoFocus,r=e.autoComplete,i=e.name,o=e.input_type,c=e.placeholder,s=e.value,u=e.onChange;return l.a.createElement("div",{className:"formGroup"},l.a.createElement("label",{className:"label"},n),l.a.createElement("input",{className:"textInput",required:t,autoFocus:a,autoComplete:r,name:i,type:o,placeholder:c,value:s,onChange:u}))}),ne=function(e){var n=e.children,t=e.onClick;return l.a.createElement("div",{className:"btnSmallOuter"},l.a.createElement("div",{className:"btnSmall",onClick:t},n))},te=function(e){var n=e.children,t=e.onClick;return l.a.createElement("div",{className:"btn",onClick:t},n)},ae=function(){var e=Object(r.useState)(""),n=Object(u.a)(e,2),t=n[0],i=n[1],o=P(),c=Object(s.c)(y),m=Object(p.g)();return c===a.loggedOut?l.a.createElement(J,{size:A.small,menuTitle:"Log in",pageTitle:"Log in",topText:"Please Log in to continue"},l.a.createElement(F,null,l.a.createElement(ee,{label:"User ID: ",autoFocus:!0,autoComplete:"username",name:"userId",input_type:"text",placeholder:"User ID",value:t,onChange:function(e){return i(e.target.value)},required:!0})),l.a.createElement(te,{onClick:function(e){(e.preventDefault(),X(t))?alert("uer ID cant be empty"):$({userID:t}).then((function(e){return e.json()})).then((function(e){var n=e.body;switch(e.status){case 200:alert("Logged in"),n.map((function(e){o(w(parseInt(e.startTime))),o(f()),m.push("/register")}));break;case 401:alert("invalid credentials");break;case 403:n.map((function(e){o(w(parseInt(e.startTime)))})),alert("you are early");break;case 501:alert("Internal server error");break;default:console.log("Error while trying to login, server returned : ".concat(e.status))}})).catch((function(e){return console.log("Error occoured: ".concat(e))}))}},l.a.createElement("span",null,"L"),"og in")):l.a.createElement(p.a,{to:"/"})},re=function(){return Object(s.c)(y)===a.challengeReady?l.a.createElement(J,{size:A.big,menuTitle:"Leaderboard",pageTitle:"Leaderboard",topText:"Top 10 scores: "},l.a.createElement(F,null,l.a.createElement("p",null," Scorecard goes here "))):l.a.createElement(p.a,{to:"/"})},le=function(){var e=Object(r.useState)(""),n=Object(u.a)(e,2),t=n[0],i=n[1],o=Object(r.useState)(""),c=Object(u.a)(o,2),m=c[0],b=c[1],d=P(),h=Object(p.g)();return Object(s.c)(y)===a.loggedIn?l.a.createElement(J,{size:A.small,pageTitle:"Register",menuTitle:"Register",topText:"Please enter the following details to continue"},l.a.createElement(F,null,l.a.createElement(ee,{label:"Nickname: ",required:!0,autoFocus:!0,autoComplete:"name",name:"nickname",input_type:"text",placeholder:"Nickname",value:t,onChange:function(e){i(e.target.value)}}),l.a.createElement(ee,{label:"Email: ",required:!0,autoFocus:!1,autoComplete:"email",name:"email",input_type:"email",placeholder:"Email",value:m,onChange:function(e){b(e.target.value)}})),l.a.createElement(te,{onClick:function(e){(e.preventDefault(),X(m)||X(t))?alert("email and nickname cant be empty"):function(e){return fetch(Z.REGISTER,{method:"post",headers:{"Content-type":"application/json; charset=UTF-8"},body:JSON.stringify(e)}).then((function(e){return e.json()}))}({nickname:t,email:m}).then((function(e){switch(e.status){case 200:d(E()),h.push("/challenges");break;case 400:null!==e.body&&e.body.map((function(e){alert("Somthing happened"),e.error.map((function(e){return alert("looks like ".concat(e," is already taken"))}))}))}}))}},"Save")):l.a.createElement(p.a,{to:"/"})},ie=document.getElementById("falcon");if(null!==ie)ie.getAttribute("onclick");document.getElementById("yoda");var oe,ce=new RegExp("^[0-1]+$");!function(e){e[e.noHint=0]="noHint",e[e.showFlag=1]="showFlag",e[e.showBinToDecHint=2]="showBinToDecHint"}(oe||(oe={}));var se=function(e){var n=e.hintState,t=e.onClick;return n===oe.showFlag?l.a.createElement("p",null,"Computer: 000111000111011110100100110110100001"):n===oe.noHint?l.a.createElement(ne,{onClick:t},"Aks computer nicely"):l.a.createElement("p",null,"Well, the flag that you have entered might be correct but there's no way of verifying it. We, unlike computers, only speak decimal.So can you please present the flag in a way that we understand?"," ")},ue=function(e){var n,t=e.id,a=e.body,i=Object(r.useState)(""),o=Object(u.a)(i,2),c=o[0],s=o[1],p=Object(r.useState)(oe.noHint),m=Object(u.a)(p,2),b=m[0],d=m[1];return n=1===t?l.a.createElement("div",null,l.a.createElement("div",{dangerouslySetInnerHTML:{__html:a}}),l.a.createElement(se,{hintState:b,onClick:function(e){e.preventDefault(),b===oe.noHint&&d(oe.showFlag)}})):l.a.createElement("div",{dangerouslySetInnerHTML:{__html:a}}),l.a.createElement("div",{className:"challengeBody"},n,l.a.createElement("div",{className:"btnSmallOuter"},l.a.createElement(ee,{label:"Flag:",autoFocus:!0,name:"userAnswer",input_type:"text",autoComplete:"flag",placeholder:"Flag",value:c,onChange:function(e){return s(e.target.value)},required:!0}),l.a.createElement(te,{onClick:function(e){e.preventDefault(),X(c)?alert("can't be empty"):ce.test(c)?1===t&&ce.test(c)&&d(oe.showBinToDecHint):alert("submit")}},"submit")))},pe={challenges:[{id:1,challengeTitle:"be3p_b00p_be3p_b00p",challengeBody:'\n  <p id="Obi-Wan-Kenobi">\n  To a computer, everything is either a 1 or a 0. Even if you feed it an image\n  of pizzas, it\'ll still be a bunch of 0s and 1s. This way of representing\n  things is called base-2 numeral system or binary numeral system. The flag for\n  this challenge is a 10-digit number in the decimal number system(ranges from 0\n  to 9). <br /> <br />We have given your computer special instructions to hide it away from\n  you. But you know how computers are,they are high maintenance. So maybe if you\n  ask it nicely, it might tell you what the flag is!\n</p>\n<br />\n<style>\n  .hidden {\n    visibility: hidden;\n  }\n  .show {\n    visibility: visible;\n  }\n</style>\n',challengeAnswer:"7641648545",score:10},{id:2,challengeTitle:"iam_uniqu3",challengeBody:'\n<p>\nDid you know that files have fingerprints just like us? They <em>are</em> just 0s and\n  1s but they too have fingerprints. The entropy(randomness) lies is how their\n  contents(0s and 1s) are arranged. This fingerprint is often calculated using\n  hash functions, such as SHA256.\n</p>\n<p>\n  The flag for this challenge is the SHA256 checksum(fingerprint) of this\n  <a href="/file.txt" target="_blank">file</a> calculated using SHA256 hash function.\n  <br />\n  <br />\n  File: \n  <a href="/file.txt" target="_blank">file.txt</a>\n</p> ',challengeAnswer:"db108f489f3b14e228b3b35f365b3b6d4f64a6f653287347ad3bde203c70cae7",score:10},{id:3,challengeTitle:".... --- .-- -.. -.--",challengeBody:'\n<p>\n  What does "RTV3Jld3djFqUlZSdUhUXnlmOCYl" mean? I think it has something to do\n  with a binary-to-text encoding that\'s popular on the World Wide Web\n</p> ',challengeAnswer:"E5w&Wwv1jRVRuHT^yf8&%",score:20},{id:4,challengeTitle:"catch_m3_if_u_can",challengeAnswer:"qEizblnviY2fBtApKgQjf08Wdr9S",challengeBody:"\n<p>\n  We wanted this challenge to be very easy. In fact, we wanted to just give the\n  flag away without any ceremony. Because of how simple this challenge is, we\n  figured we'll put our laziest server up to this task. We asked it to just give\n  it away when you ask for it. But it looks like it is feeling particularly\n  energetic tonight and is pulling tricks on us. We are sorry but you are going\n  to have to retrieve this flag to clear this challenge.\n  <br />\n  <br />\n  Misbehaving server:\n  <a href='".concat("https://anxiousturtle.herokuapp.com/","'>").concat("https://anxiousturtle.herokuapp.com/","</a>\n  <br />\n  <br />\n  P.S: Apparently, Firefox is fast enough to talk to the misbehaving server!\n</p> "),score:20},{id:5,challengeTitle:"hidd3n_in_p1ain_sight",challengeAnswer:" SC2INcxcddmV2",challengeBody:'\n<p>\n  The flag is hidden in this <a href="./voodo.txt" target="_blank">file</a>. It\n  would be very tedious if you look for it manually, something tells me there\'s\n  a better way.\n</p>\n  <br />\n  <br />\n  File:\n<a href="./voodo.txt" target="_blank">voodo.txt</a>.\n',score:10},{id:6,challengeTitle:"hack_the_gibson",challengeAnswer:"zlZ6QPh97sg16ds856RK0DIK1waNJwy",challengeBody:"\n<script>\n  sessionStorage.setItem(\n    'FR5tcymRp1n2VKz9BxWhrstyjyof17z',\n    'nCTF{".concat("zlZ6QPh97sg16ds856RK0DIK1waNJwy","}',\n  );\n<\/script>\n<p>Hack this page to retrieve the flag for this challenge</p>\n"),score:20},{id:7,challengeTitle:"R2^_^3_&s%",challengeAnswer:"nCTF{91QGh7kJxl0bBYt3tu96GnfqN8i9oSI}",challengeBody:'\n<p>\n  You are are R2-D2, a cute robot from the Star Wars franchise are R2-D2. You\n  are tasked with hacking into the most secure valut of the Death Star to\n  retrive it\'s blueprints. The vault is controlled by a computer and requires a\n  password to open. The source code for the vault was obtained by one of our\n  comrades.\n  <br />\n  Please find the password(the flag) hidden in the source code and help the\n  rebels live to fight another day.\n  <br />\n  <br />\n  <br />\n  Vault:\n  <a href="https://deathstarvault.herokuapp.com/" target="_blank"\n    >https://deathstarvault.herokuapp.com/</a\n  >\n  <br />\n  Source code:\n  <a href="https://github.com/realaravinth/death-star" target="_blank">https://github.com/realaravinth/death-star</a>\n</p>\n',score:70}]},me=function(e){var n=e.onClick,t=pe.challenges.map((function(e){return l.a.createElement("li",{key:e.id,className:"ctitle",onClick:function(e){function n(){return e.apply(this,arguments)}return n.toString=function(){return e.toString()},n}((function(){return n(e.id)}))}," ",e.challengeTitle," ")}));return l.a.createElement("ul",{id:"challenges"}," ",t," ")},be=function(e){var n=e.children;return l.a.createElement("div",{className:"contentcolumned"},n)},de=function(e){var n=e.children;return l.a.createElement("div",{className:"menuoptions menusidebar"},n)},he=function(e){var n=e.children;return l.a.createElement("div",{className:"menusidebarbody menuoptions"},n)},ge=document.getElementById("falcon");if(null!==ge)ge.getAttribute("onclick");document.getElementById("yoda"),new RegExp("^[0-1]+$");var fe=function(){var e=Object(s.c)(y),n=Object(r.useState)(1),t=Object(u.a)(n,2),i=t[0],o=t[1],c=Object(r.useState)(pe.challenges[0].challengeBody),m=Object(u.a)(c,2),b=m[0],d=m[1];console.log(b);return e===a.challengeReady?l.a.createElement(J,{size:A.big,menuTitle:"Challenges",topText:"Solve these challges to find flags. Flags will be in the form nCTF{secret-goes-in-here}",pageTitle:"Challenges"},l.a.createElement(be,null,l.a.createElement(de,null,l.a.createElement(me,{onClick:function(e){o(e),function(e){pe.challenges.forEach((function(n){n.id===e&&d(n.challengeBody)}))}(e)}})),l.a.createElement(he,null,l.a.createElement(ue,{body:b,id:i})))):l.a.createElement(p.a,{to:"/"})},ve=function(){switch(Object(s.c)(y)){case a.loggedIn:return l.a.createElement(p.a,{to:"/register"});case a.registered:case a.challengeReady:return l.a.createElement(p.a,{to:"/challenges"});case a.loggedOut:return l.a.createElement(p.a,{to:"/login"})}},Ee=function(){return fetch(Z.LOGOUT,{credentials:"include"}).then((function(e){return e.json()}))},ye=function(){var e=Object(s.c)(y),n=P();return e===a.challengeReady&&(n(v()),Ee()),l.a.createElement(p.a,{to:"/login"})},Te=function(e){e.completed;for(var n={height:"100%",width:"10%",border:"0.1px solid",borderColor:"#fff",textAlign:"right"},t=[],a=0;a<30;a++)t.push(a);var r=t.map((function(e){return l.a.createElement("div",{style:n})}));return l.a.createElement("div",{style:{height:13,width:"100%",display:"flex"}},r)},we=function(){return l.a.createElement(J,{size:A.loading,menuTitle:"Loading",topText:"",pageTitle:"Loading"},l.a.createElement(Te,{completed:100}))},ke=function(){return l.a.createElement(p.d,null,l.a.createElement(p.b,{component:re,path:"/leaderboard"}),l.a.createElement(p.b,{component:le,path:"/register"}),l.a.createElement(p.b,{component:fe,path:"/challenges"}),l.a.createElement(p.b,{component:ae,path:"/login"}),l.a.createElement(p.b,{component:ye,path:"/logout"}),l.a.createElement(p.b,{component:we,exact:!0,path:"/loading"}),l.a.createElement(p.b,{component:ve,exact:!0,path:"/"}))},Oe=function(){return window.matchMedia("(max-width: 1000px)").matches?l.a.createElement("div",{className:"App"},l.a.createElement("div",{className:"container"},l.a.createElement(M,null))):l.a.createElement("div",{className:"App"},l.a.createElement(B,null),l.a.createElement(C,null),l.a.createElement("div",{className:"container"},l.a.createElement(ke,null)),"// End of container div")};Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));K.subscribe((function(){return function(e){try{var n=JSON.stringify(e);localStorage.setItem("state",n)}catch(t){console.log("error from saveState: ".concat(t))}}(K.getState())})),o.a.render(l.a.createElement(l.a.StrictMode,null,l.a.createElement(c.a,null,l.a.createElement(s.a,{store:K},l.a.createElement(Oe,null)))),document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then((function(e){e.unregister()})).catch((function(e){console.error(e.message)}))}},[[24,1,2]]]);
//# sourceMappingURL=main.a7c0a088.chunk.js.map