(function(e){function t(t){for(var r,o,s=t[0],c=t[1],u=t[2],m=0,p=[];m<s.length;m++)o=s[m],Object.prototype.hasOwnProperty.call(a,o)&&a[o]&&p.push(a[o][0]),a[o]=0;for(r in c)Object.prototype.hasOwnProperty.call(c,r)&&(e[r]=c[r]);l&&l(t);while(p.length)p.shift()();return i.push.apply(i,u||[]),n()}function n(){for(var e,t=0;t<i.length;t++){for(var n=i[t],r=!0,s=1;s<n.length;s++){var c=n[s];0!==a[c]&&(r=!1)}r&&(i.splice(t--,1),e=o(o.s=n[0]))}return e}var r={},a={app:0},i=[];function o(t){if(r[t])return r[t].exports;var n=r[t]={i:t,l:!1,exports:{}};return e[t].call(n.exports,n,n.exports,o),n.l=!0,n.exports}o.m=e,o.c=r,o.d=function(e,t,n){o.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},o.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},o.t=function(e,t){if(1&t&&(e=o(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(o.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)o.d(n,r,function(t){return e[t]}.bind(null,r));return n},o.n=function(e){var t=e&&e.__esModule?function(){return e["default"]}:function(){return e};return o.d(t,"a",t),t},o.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},o.p="/";var s=window["webpackJsonp"]=window["webpackJsonp"]||[],c=s.push.bind(s);s.push=t,s=s.slice();for(var u=0;u<s.length;u++)t(s[u]);var l=c;i.push([0,"chunk-vendors"]),n()})({0:function(e,t,n){e.exports=n("56d7")},"1d1b":function(e,t,n){},2635:function(e,t,n){},"47bc":function(e,t,n){"use strict";n("2635")},"56d7":function(e,t,n){"use strict";n.r(t);n("e260"),n("e6cf"),n("cca6"),n("a79d");var r=n("2b0e"),a=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{attrs:{id:"gameshow"}},[n("cookie-consent",{attrs:{lang:e.lang},on:{consent:e.got_consent}}),n("div",{staticClass:"mainWindow"},[n("div",{staticClass:"sidebar"},[""!=e.nickname&&""!=e.lobby?[n("transition",{attrs:{name:"transition",mode:"out-in",appear:""}},[n("div",{staticClass:"compWindow",staticStyle:{"text-align":"center"}},[n("span",[e._v(e._s(e.lang["Question"])+" "+e._s(e.current_question.id))])])])]:e._e()],2),n("div",{staticClass:"mainStage"},[n("transition",{attrs:{name:"transition",mode:"out-in",appear:""}},["loading"==e.selectedWindow?[n("div",{staticClass:"compWindow"},[e._v(" "+e._s(e.lang["Loading"])+".. ")])]:"login-window"==e.selectedWindow?[n("login-window",{attrs:{lang:e.lang},on:{"set-name":e.set_name}})]:[n("div",{staticClass:"compWindow"},[e._v(" "+e._s(e.lang["Waiting for players"])+".. ")])]],2)],1)])],1)},i=[],o=n("1da1"),s=(n("96cf"),{name:"lang",en:{Accept:"Accept","This site uses (only functional) cookies!":"This site uses (only functional) cookies!",Question:"Question","Waiting for players":"Waiting for players",Name:"Name",Submit:"Submit","Name must not be empty!":"Name must not be empty!",Loading:"Loading","Connection to server failed!":"Connection to server failed!"},de:{Accept:"Akzeptieren","This site uses (only functional) cookies!":"Diese Seite benutzt (nur funktionale) Cookies!",Question:"Frage","Waiting for players":"Warte auf Mitspieler",Name:"Name",Submit:"Absenden","Name must not be empty!":"Name darf nicht leer sein!",Loading:"Lädt","Connection to server failed!":"Verbindung zum Server fehlgeschlagen!"}}),c=(n("d3b7"),n("99af"),"./api/"),u={name:"api",lang:s.en,set_name:function(){var e=Object(o["a"])(regeneratorRuntime.mark((function e(t){var n,r;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.next=2,fetch(c+"set_name?name="+encodeURIComponent(t));case 2:if(n=e.sent,n.ok){e.next=11;break}return e.next=6,n.text();case 6:return r=e.sent,alert("".concat(this.lang["Connection to server failed!"]," \n ").concat(n.status," ").concat(n.statusText," \n ").concat(r)),e.abrupt("return",!1);case 11:return e.abrupt("return",!0);case 12:case"end":return e.stop()}}),e,this)})));function t(t){return e.apply(this,arguments)}return t}(),get_name:function(){var e=Object(o["a"])(regeneratorRuntime.mark((function e(){var t,n;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return e.next=2,fetch(c+"get_name");case 2:if(t=e.sent,t.ok){e.next=13;break}if(404!=t.status){e.next=6;break}return e.abrupt("return","");case 6:return e.next=8,t.text();case 8:return n=e.sent,alert("".concat(this.lang["Connection to server failed!"]," \n ").concat(t.status," ").concat(t.statusText," \n ").concat(n)),e.abrupt("return","");case 13:return e.next=15,t.text();case 15:return e.abrupt("return",e.sent);case 16:case"end":return e.stop()}}),e,this)})));function t(){return e.apply(this,arguments)}return t}()},l=function(){var e=this,t=e.$createElement,n=e._self._c||t;return e.visible?n("div",{staticClass:"background"},[n("div",{staticClass:"window"},[n("p",{staticClass:"text"},[e._v(e._s(e.lang["This site uses (only functional) cookies!"]))]),n("div",{staticClass:"button-accept",on:{click:e.accept}},[e._v(e._s(e.lang["Accept"]))])])]):e._e()},m=[],p=(n("1276"),n("ac1f"),n("498a"),{name:"global",getCookie:function(e){for(var t=document.cookie.split(";"),n=0;n<t.length;n++){var r=t[n].split("=");if(e==r[0].trim())return decodeURIComponent(r[1])}return null}}),d={name:"CookieConsent",props:["lang"],data:function(){return{visible:!1}},methods:{accept:function(){document.cookie="CONSENT=1",this.visible=!1,this.$emit("consent")},show:function(){this.visible=!0}},mounted:function(){var e=p.getCookie("CONSENT");"1"!=e?this.show():this.$emit("consent")}},f=d,g=(n("47bc"),n("2877")),b=Object(g["a"])(f,l,m,!1,null,"854b6210",null),h=b.exports,v=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{staticClass:"compWindow"},[n("form",{on:{submit:function(t){return t.preventDefault(),e.submit_name(t)}}},[n("label",{attrs:{for:"name"}},[e._v(e._s(e.lang["Name"])+":")]),n("input",{directives:[{name:"model",rawName:"v-model.trim",value:e.nickname,expression:"nickname",modifiers:{trim:!0}}],staticClass:"input",attrs:{type:"text",name:"name",placeholder:"<"+e.lang["Name"]+">",autofocus:"",id:"login-input"},domProps:{value:e.nickname},on:{input:function(t){t.target.composing||(e.nickname=t.target.value.trim())},blur:function(t){return e.$forceUpdate()}}}),n("br"),e.error?[n("span",{staticClass:"error"},[e._v(e._s(e.error_msg))]),n("br")]:e._e(),n("input",{staticClass:"button",attrs:{type:"submit",id:"login-submit"},domProps:{value:e.lang["Submit"]}})],2)])},_=[],w={name:"LoginWindow",props:["lang"],data:function(){return{nickname:"",error:!1,error_msg:""}},methods:{submit_name:function(){if(""==this.nickname)this.error=!0,this.error_msg=this.lang["Name must not be empty!"];else{this.error=!1;var e=document.getElementById("login-submit");e.setAttribute("disabled","disabled"),this.$emit("set-name",this.nickname),setTimeout((function(){e.removeAttribute("disabled")}),2e3)}}},mounted:function(){document.getElementById("login-input").focus()}},y=w,k=(n("780e"),Object(g["a"])(y,v,_,!1,null,"2d795efd",null)),x=k.exports,C={name:"Gameshow",components:{CookieConsent:h,LoginWindow:x},data:function(){return{lang:s.en,consent:!1,selectedWindow:"loading",nickname:"",lobby:"",money:1,jokers:0,players:[],results_players_prev:[],results_players_new:[],current_question:{id:0,type:"",category:"",question:"",answers:[],correct_answer:0,wrong_answers:[]},last_event_id:-1}},methods:{switchLanguage:function(e){switch(e){case"de":return this.lang=s.de,u.lang=s.de,!0;case"en":return this.lang=s.en,u.lang=s.en,!0;default:return!1}},got_consent:function(){var e=Object(o["a"])(regeneratorRuntime.mark((function e(){var t;return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:return this.consent=!0,e.next=3,u.get_name();case 3:t=e.sent,""!=t?this.nickname=t:this.selectedWindow="login-window";case 5:case"end":return e.stop()}}),e,this)})));function t(){return e.apply(this,arguments)}return t}(),set_name:function(){var e=Object(o["a"])(regeneratorRuntime.mark((function e(t){return regeneratorRuntime.wrap((function(e){while(1)switch(e.prev=e.next){case 0:if(this.consent){e.next=2;break}return e.abrupt("return");case 2:return e.next=4,u.set_name(t);case 4:if(!e.sent){e.next=6;break}this.nickname=t;case 6:case"end":return e.stop()}}),e,this)})));function t(t){return e.apply(this,arguments)}return t}()},mounted:function(){}},O=C,j=(n("99a6"),Object(g["a"])(O,a,i,!1,null,null,null)),N=j.exports;r["a"].config.productionTip=!1,new r["a"]({render:function(e){return e(N)}}).$mount("#app")},"5ca0":function(e,t,n){},"780e":function(e,t,n){"use strict";n("1d1b")},"99a6":function(e,t,n){"use strict";n("5ca0")}});
//# sourceMappingURL=app.eddc9002.js.map