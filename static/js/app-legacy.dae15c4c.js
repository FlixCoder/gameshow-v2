(function(e){function t(t){for(var r,a,s=t[0],c=t[1],u=t[2],b=0,d=[];b<s.length;b++)a=s[b],Object.prototype.hasOwnProperty.call(i,a)&&i[a]&&d.push(i[a][0]),i[a]=0;for(r in c)Object.prototype.hasOwnProperty.call(c,r)&&(e[r]=c[r]);l&&l(t);while(d.length)d.shift()();return o.push.apply(o,u||[]),n()}function n(){for(var e,t=0;t<o.length;t++){for(var n=o[t],r=!0,s=1;s<n.length;s++){var c=n[s];0!==i[c]&&(r=!1)}r&&(o.splice(t--,1),e=a(a.s=n[0]))}return e}var r={},i={app:0},o=[];function a(t){if(r[t])return r[t].exports;var n=r[t]={i:t,l:!1,exports:{}};return e[t].call(n.exports,n,n.exports,a),n.l=!0,n.exports}a.m=e,a.c=r,a.d=function(e,t,n){a.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},a.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},a.t=function(e,t){if(1&t&&(e=a(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(a.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)a.d(n,r,function(t){return e[t]}.bind(null,r));return n},a.n=function(e){var t=e&&e.__esModule?function(){return e["default"]}:function(){return e};return a.d(t,"a",t),t},a.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},a.p="/";var s=window["webpackJsonp"]=window["webpackJsonp"]||[],c=s.push.bind(s);s.push=t,s=s.slice();for(var u=0;u<s.length;u++)t(s[u]);var l=c;o.push([0,"chunk-vendors"]),n()})({0:function(e,t,n){e.exports=n("56d7")},2635:function(e,t,n){},"47bc":function(e,t,n){"use strict";n("2635")},"4ee1":function(e,t,n){"use strict";n("cd32")},"56d7":function(e,t,n){"use strict";n.r(t);n("e260"),n("e6cf"),n("cca6"),n("a79d");var r=n("2b0e"),i=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{attrs:{id:"gameshow"}},[n("cookie-consent",{attrs:{lang:e.lang},on:{consent:e.got_consent}}),n("div",{staticClass:"mainWindow"},[n("div",{staticClass:"sidebar"},[""!=e.nickname&&""!=e.lobby?[n("transition",{attrs:{name:"transition",mode:"out-in",appear:""}},[n("div",{staticClass:"compWindow",staticStyle:{"text-align":"center"}},[n("span",[e._v(e._s(e.lang["Question"])+" "+e._s(e.current_question.id))])])])]:e._e()],2),n("div",{staticClass:"mainStage"},[n("transition",{attrs:{name:"transition",mode:"out-in",appear:""}},["loading"==e.selectedWindow?[n("div",{staticClass:"compWindow"},[e._v(" "+e._s(e.lang["Loading"])+".. ")])]:"login-window"==e.selectedWindow?[n("login-window",{attrs:{lang:e.lang},on:{"set-name":e.set_name}})]:"lobby-selection"==e.selectedWindow?[n("lobby-selection",{attrs:{lang:e.lang},on:{"create-lobby":e.create_lobby,"join-lobby":e.join_lobby}})]:[n("div",{staticClass:"compWindow"},[e._v(" "+e._s(e.lang["Waiting for players"])+".. ")])]],2)],1)])],1)},o=[],a=n("1da1"),s=(n("96cf"),n("1276"),n("ac1f"),n("498a"),{getCookie:function(e){for(var t=document.cookie.split(";"),n=0;n<t.length;n++){var r=t[n].split("=");if(e==r[0].trim())return decodeURIComponent(r[1])}return null},extract_lobby_id:function(){var e=window.location.href,t=e.indexOf("#");return-1==t?"":e.substr(t+1)}}),c={name:"lang",en:{Accept:"Accept","This site uses (only functional) cookies!":"This site uses (only functional) cookies!",Question:"Question","Waiting for players":"Waiting for players",Name:"Name",Submit:"Submit","Name must not be empty!":"Name must not be empty!",Loading:"Loading","Connection to server failed!":"Connection to server failed!","Lobby ID":"Lobby ID","Create lobby":"Create lobby",Join:"Join","Lobby ID must not be empty!":"Lobby ID must not be empty!","Join lobby":"Join lobby"},de:{Accept:"Akzeptieren","This site uses (only functional) cookies!":"Diese Seite benutzt (nur funktionale) Cookies!",Question:"Frage","Waiting for players":"Warte auf Mitspieler",Name:"Name",Submit:"Absenden","Name must not be empty!":"Name darf nicht leer sein!",Loading:"Lädt","Connection to server failed!":"Verbindung zum Server fehlgeschlagen!","Lobby ID":"Lobby ID","Create lobby":"Lobby erstellen",Join:"Beitreten","Lobby ID must not be empty!":"Lobby ID darf nicht leer sein!","Join lobby":"Lobby beitreten"}},u=(n("d3b7"),n("99af"),"./api/"),l={name:"api",lang:c.en,set_name:function(){var e=Object(a["a"])(regeneratorRuntime.mark((function e(t){var n,r;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.next=2,fetch(u+"set_name?name="+encodeURIComponent(t));case 2:if(n=e.sent,n.ok){e.next=11;break}return e.next=6,n.text();case 6:return r=e.sent,alert("".concat(this.lang["Connection to server failed!"]," \n ").concat(n.status," ").concat(n.statusText," \n ").concat(r)),e.abrupt("return",!1);case 11:return e.abrupt("return",!0);case 12:case"end":return e.stop()}}),e,this)})));function t(t){return e.apply(this,arguments)}return t}(),get_name:function(){var e=Object(a["a"])(regeneratorRuntime.mark((function e(){var t,n;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.next=2,fetch(u+"get_name");case 2:if(t=e.sent,t.ok){e.next=13;break}if(404!=t.status){e.next=6;break}return e.abrupt("return","");case 6:return e.next=8,t.text();case 8:return n=e.sent,alert("".concat(this.lang["Connection to server failed!"]," \n ").concat(t.status," ").concat(t.statusText," \n ").concat(n)),e.abrupt("return","");case 13:return e.next=15,t.text();case 15:return e.abrupt("return",e.sent);case 16:case"end":return e.stop()}}),e,this)})));function t(){return e.apply(this,arguments)}return t}()},b=function(){var e=this,t=e.$createElement,n=e._self._c||t;return e.visible?n("div",{staticClass:"background"},[n("div",{staticClass:"window"},[n("p",{staticClass:"text"},[e._v(e._s(e.lang["This site uses (only functional) cookies!"]))]),n("div",{staticClass:"button-accept",on:{click:e.accept}},[e._v(e._s(e.lang["Accept"]))])])]):e._e()},d=[],m={name:"CookieConsent",props:["lang"],data:function(){return{visible:!1}},methods:{accept:function(){document.cookie="CONSENT=1",this.visible=!1,this.$emit("consent")},show:function(){this.visible=!0}},mounted:function(){var e=s.getCookie("CONSENT");"1"!=e?this.show():this.$emit("consent")}},p=m,f=(n("47bc"),n("2877")),y=Object(f["a"])(p,b,d,!1,null,"854b6210",null),g=y.exports,h=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{staticClass:"compWindow"},[n("form",{on:{submit:function(t){return t.preventDefault(),e.submit_name(t)}}},[n("label",{attrs:{for:"name"}},[e._v(e._s(e.lang["Name"])+":")]),n("input",{directives:[{name:"model",rawName:"v-model.trim",value:e.nickname,expression:"nickname",modifiers:{trim:!0}}],staticClass:"input",attrs:{type:"text",name:"name",placeholder:"<"+e.lang["Name"]+">",autofocus:"",id:"login-input"},domProps:{value:e.nickname},on:{input:function(t){t.target.composing||(e.nickname=t.target.value.trim())},blur:function(t){return e.$forceUpdate()}}}),n("br"),e.error?[n("span",{staticClass:"error"},[e._v(e._s(e.error_msg))]),n("br")]:e._e(),n("input",{staticClass:"button",attrs:{type:"submit",id:"login-submit"},domProps:{value:e.lang["Submit"]}})],2)])},v=[],_={name:"LoginWindow",props:["lang"],data:function(){return{nickname:"",error:!1,error_msg:""}},methods:{submit_name:function(){if(""==this.nickname)this.error=!0,this.error_msg=this.lang["Name must not be empty!"];else{this.error=!1;var e=document.getElementById("login-submit");e.setAttribute("disabled","disabled"),this.$emit("set-name",this.nickname),setTimeout((function(){e.removeAttribute("disabled")}),2e3)}}},mounted:function(){document.getElementById("login-input").focus()}},w=_,x=(n("4ee1"),Object(f["a"])(w,h,v,!1,null,"eac3244e",null)),k=x.exports,C=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{staticClass:"compWindow"},[n("form",{on:{submit:function(t){return t.preventDefault(),e.button_create_lobby(t)}}},[n("input",{staticClass:"button",attrs:{type:"submit",id:"lobby-create"},domProps:{value:e.lang["Create lobby"]}})]),n("br"),n("form",{on:{submit:function(t){return t.preventDefault(),e.button_join_lobby(t)}}},[n("label",{attrs:{for:"lobbyID"}},[e._v(e._s(e.lang["Join lobby"])+":")]),n("input",{directives:[{name:"model",rawName:"v-model.trim",value:e.lobby_id,expression:"lobby_id",modifiers:{trim:!0}}],staticClass:"input",attrs:{type:"text",name:"lobbyID",placeholder:"<"+e.lang["Lobby ID"]+">",autofocus:"",id:"lobby-input"},domProps:{value:e.lobby_id},on:{input:function(t){t.target.composing||(e.lobby_id=t.target.value.trim())},blur:function(t){return e.$forceUpdate()}}}),n("br"),e.error?[n("span",{staticClass:"error"},[e._v(e._s(e.error_msg))]),n("br")]:e._e(),n("input",{staticClass:"button",attrs:{type:"submit",id:"lobby-join"},domProps:{value:e.lang["Join"]}})],2)])},j=[],O={name:"LobbySelection",props:["lang"],data:function(){return{lobby_id:"",error:!1,error_msg:""}},methods:{button_create_lobby:function(){var e=document.getElementById("lobby-create"),t=document.getElementById("lobby-join");e.setAttribute("disabled","disabled"),t.setAttribute("disabled","disabled"),this.$emit("create-lobby"),setTimeout((function(){e.removeAttribute("disabled"),t.removeAttribute("disabled")}),2e3)},button_join_lobby:function(){if(""==this.lobby_id)this.error=!0,this.error_msg=this.lang["Lobby ID must not be empty!"];else{this.error=!1;var e=document.getElementById("lobby-create"),t=document.getElementById("lobby-join");e.setAttribute("disabled","disabled"),t.setAttribute("disabled","disabled"),this.$emit("join-lobby",this.lobby_id),setTimeout((function(){e.removeAttribute("disabled"),t.removeAttribute("disabled")}),2e3)}}},mounted:function(){document.getElementById("lobby-input").focus()}},L=O,W=(n("f839"),Object(f["a"])(L,C,j,!1,null,"53cc3b4c",null)),I=W.exports,N={name:"Gameshow",components:{CookieConsent:g,LoginWindow:k,LobbySelection:I},data:function(){return{lang:c.en,consent:!1,selectedWindow:"loading",nickname:"",lobby:"",admin:"",money:1,jokers:0,players:[],results_players_prev:[],results_players_new:[],current_question:{id:0,type:"",category:"",question:"",answers:[],correct_answer:0,wrong_answers:[]},last_event_id:-1}},methods:{switchLanguage:function(e){switch(e){case"de":return this.lang=c.de,l.lang=c.de,!0;case"en":return this.lang=c.en,l.lang=c.en,!0;default:return!1}},got_consent:function(){var e=Object(a["a"])(regeneratorRuntime.mark((function e(){var t,n;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return this.consent=!0,e.next=3,l.get_name();case 3:if(t=e.sent,""==t){e.next=17;break}if(this.nickname=t,n=s.extract_lobby_id(),""==n){e.next=14;break}return e.next=10,this.join_lobby(n);case 10:if(e.sent){e.next=12;break}this.selectedWindow="lobby-selection";case 12:e.next=15;break;case 14:this.selectedWindow="lobby-selection";case 15:e.next=18;break;case 17:this.selectedWindow="login-window";case 18:case"end":return e.stop()}}),e,this)})));function t(){return e.apply(this,arguments)}return t}(),set_name:function(){var e=Object(a["a"])(regeneratorRuntime.mark((function e(t){var n;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:if(this.consent){e.next=2;break}return e.abrupt("return");case 2:return e.next=4,l.set_name(t);case 4:if(!e.sent){e.next=15;break}if(this.nickname=t,n=s.extract_lobby_id(),""==n){e.next=14;break}return e.next=10,this.join_lobby(n);case 10:if(e.sent){e.next=12;break}this.selectedWindow="lobby-selection";case 12:e.next=15;break;case 14:this.selectedWindow="lobby-selection";case 15:case"end":return e.stop()}}),e,this)})));function t(t){return e.apply(this,arguments)}return t}(),create_lobby:function(){var e=Object(a["a"])(regeneratorRuntime.mark((function e(){return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:if(this.consent){e.next=2;break}return e.abrupt("return");case 2:case"end":return e.stop()}}),e,this)})));function t(){return e.apply(this,arguments)}return t}(),join_lobby:function(){var e=Object(a["a"])(regeneratorRuntime.mark((function e(t){return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:if(this.consent){e.next=2;break}return e.abrupt("return",!1);case 2:if(""!=t){e.next=4;break}return e.abrupt("return",!1);case 4:return e.abrupt("return",!1);case 5:case"end":return e.stop()}}),e,this)})));function t(t){return e.apply(this,arguments)}return t}()},mounted:function(){}},S=N,A=(n("99a6"),Object(f["a"])(S,i,o,!1,null,null,null)),D=A.exports;r["a"].config.productionTip=!1,new r["a"]({render:function(e){return e(D)}}).$mount("#app")},"5ca0":function(e,t,n){},"99a6":function(e,t,n){"use strict";n("5ca0")},abf6:function(e,t,n){},cd32:function(e,t,n){},f839:function(e,t,n){"use strict";n("abf6")}});
//# sourceMappingURL=app-legacy.dae15c4c.js.map