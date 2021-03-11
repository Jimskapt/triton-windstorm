let wasm,cachedTextDecoder=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});cachedTextDecoder.decode();let cachegetUint8Memory0=null;function getUint8Memory0(){return null!==cachegetUint8Memory0&&cachegetUint8Memory0.buffer===wasm.memory.buffer||(cachegetUint8Memory0=new Uint8Array(wasm.memory.buffer)),cachegetUint8Memory0}function getStringFromWasm0(e,t){return cachedTextDecoder.decode(getUint8Memory0().subarray(e,e+t))}const heap=new Array(32).fill(void 0);heap.push(void 0,null,!0,!1);let heap_next=heap.length;function addHeapObject(e){heap_next===heap.length&&heap.push(heap.length+1);const t=heap_next;return heap_next=heap[t],heap[t]=e,t}function getObject(e){return heap[e]}function dropObject(e){e<36||(heap[e]=heap_next,heap_next=e)}function takeObject(e){const t=getObject(e);return dropObject(e),t}let WASM_VECTOR_LEN=0,cachedTextEncoder=new TextEncoder("utf-8");const encodeString="function"==typeof cachedTextEncoder.encodeInto?function(e,t){return cachedTextEncoder.encodeInto(e,t)}:function(e,t){const n=cachedTextEncoder.encode(e);return t.set(n),{read:e.length,written:n.length}};function passStringToWasm0(e,t,n){if(void 0===n){const n=cachedTextEncoder.encode(e),_=t(n.length);return getUint8Memory0().subarray(_,_+n.length).set(n),WASM_VECTOR_LEN=n.length,_}let _=e.length,a=t(_);const r=getUint8Memory0();let c=0;for(;c<_;c++){const t=e.charCodeAt(c);if(t>127)break;r[a+c]=t}if(c!==_){0!==c&&(e=e.slice(c)),a=n(a,_,_=c+3*e.length);const t=getUint8Memory0().subarray(a+c,a+_);c+=encodeString(e,t).written}return WASM_VECTOR_LEN=c,a}function isLikeNone(e){return null==e}let cachegetInt32Memory0=null;function getInt32Memory0(){return null!==cachegetInt32Memory0&&cachegetInt32Memory0.buffer===wasm.memory.buffer||(cachegetInt32Memory0=new Int32Array(wasm.memory.buffer)),cachegetInt32Memory0}function debugString(e){const t=typeof e;if("number"==t||"boolean"==t||null==e)return`${e}`;if("string"==t)return`"${e}"`;if("symbol"==t){const t=e.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==t){const t=e.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(e)){const t=e.length;let n="[";t>0&&(n+=debugString(e[0]));for(let _=1;_<t;_++)n+=", "+debugString(e[_]);return n+="]",n}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let _;if(!(n.length>1))return toString.call(e);if(_=n[1],"Object"==_)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:_}function makeMutClosure(e,t,n,_){const a={a:e,b:t,cnt:1,dtor:n},r=(...e)=>{a.cnt++;const t=a.a;a.a=0;try{return _(t,a.b,...e)}finally{0==--a.cnt?wasm.__wbindgen_export_2.get(a.dtor)(t,a.b):a.a=t}};return r.original=a,r}function __wbg_adapter_20(e,t,n){wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4aea35cf3888e05e(e,t,addHeapObject(n))}function __wbg_adapter_23(e,t,n){wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h01a16b4c876dba7b(e,t,n)}export function start(){wasm.start()}function handleError(e){return function(){try{return e.apply(this,arguments)}catch(e){wasm.__wbindgen_exn_store(addHeapObject(e))}}}function getArrayU8FromWasm0(e,t){return getUint8Memory0().subarray(e/1,e/1+t)}function __wbg_adapter_53(e,t,n,_,a){wasm.wasm_bindgen__convert__closures__invoke3_mut__hd46559a6267a7e9e(e,t,addHeapObject(n),_,addHeapObject(a))}async function load(e,t){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,t)}catch(t){if("application/wasm"==e.headers.get("Content-Type"))throw t;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",t)}const n=await e.arrayBuffer();return await WebAssembly.instantiate(n,t)}{const n=await WebAssembly.instantiate(e,t);return n instanceof WebAssembly.Instance?{instance:n,module:e}:n}}async function init(e){void 0===e&&(e=new URL("package_bg.wasm",import.meta.url));const t={wbg:{}};t.wbg.__wbg_preventDefault_2a53c6dce5093030=function(e){getObject(e).preventDefault()},t.wbg.__wbindgen_string_new=function(e,t){return addHeapObject(getStringFromWasm0(e,t))},t.wbg.__wbg_history_ea580f62f8cb1285=handleError((function(e){return addHeapObject(getObject(e).history)})),t.wbg.__wbg_pushState_10c51a6fb23b182f=handleError((function(e,t,n,_,a,r){getObject(e).pushState(getObject(t),getStringFromWasm0(n,_),0===a?void 0:getStringFromWasm0(a,r))})),t.wbg.__wbindgen_object_drop_ref=function(e){takeObject(e)},t.wbg.__wbg_cancelAnimationFrame_8df3d6d00824cedc=handleError((function(e,t){getObject(e).cancelAnimationFrame(t)})),t.wbg.__wbindgen_cb_drop=function(e){const t=takeObject(e).original;if(1==t.cnt--)return t.a=0,!0;return!1},t.wbg.__wbg_removeEventListener_e24bf386768929ee=handleError((function(e,t,n,_){getObject(e).removeEventListener(getStringFromWasm0(t,n),getObject(_))})),t.wbg.__wbg_value_022eba9b8dd4fb1e=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_23394860ba8926e6=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_c7efdd4c63bbb477=function(e){return getObject(e).value},t.wbg.__wbg_value_c8e2020625cf5c35=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_9fa10b764120d5cc=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_7465718cc9380e2a=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_656265bd80ad6d7f=function(e){return getObject(e).value},t.wbg.__wbg_value_f8f8540c26e951ad=function(e){return getObject(e).value},t.wbg.__wbg_value_0571cc1f2436b22a=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_23b744c40d881fd5=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbindgen_object_clone_ref=function(e){return addHeapObject(getObject(e))},t.wbg.__wbg_setinnerHTML_a8b2c66f141a2b24=function(e,t,n){getObject(e).innerHTML=getStringFromWasm0(t,n)},t.wbg.__wbg_childNodes_66f8f91fb4fa09d2=function(e){return addHeapObject(getObject(e).childNodes)},t.wbg.__wbg_length_ea5ea6ba35d76abd=function(e){return getObject(e).length},t.wbg.__wbg_settextContent_00a0c562129ed7b9=function(e,t,n){getObject(e).textContent=0===t?void 0:getStringFromWasm0(t,n)},t.wbg.__wbg_performance_800ff37c906b5f3b=function(e){var t=getObject(e).performance;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_now_9f22124bc74da886=function(e){return getObject(e).now()},t.wbg.__wbg_requestAnimationFrame_ef037dc409649fbf=handleError((function(e,t){return getObject(e).requestAnimationFrame(getObject(t))})),t.wbg.__wbg_getElementById_aeb1b7331ed88a97=function(e,t,n){var _=getObject(e).getElementById(getStringFromWasm0(t,n));return isLikeNone(_)?0:addHeapObject(_)},t.wbg.__wbg_firstChild_f1ab96bca1141d1f=function(e){var t=getObject(e).firstChild;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_navigator_fbf745329c64378b=function(e){return addHeapObject(getObject(e).navigator)},t.wbg.__wbg_language_909032f7bae83e7a=function(e,t){var n=getObject(t).language,_=isLikeNone(n)?0:passStringToWasm0(n,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),a=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=a,getInt32Memory0()[e/4+0]=_},t.wbg.__wbg_matchMedia_156d2b064b134a7a=handleError((function(e,t,n){var _=getObject(e).matchMedia(getStringFromWasm0(t,n));return isLikeNone(_)?0:addHeapObject(_)})),t.wbg.__wbg_matches_b8e0518e921710cd=function(e){return getObject(e).matches},t.wbg.__wbg_length_58ac6b9addb3a0a4=handleError((function(e){return getObject(e).length})),t.wbg.__wbg_key_db29b931d6a174cb=handleError((function(e,t,n){var _=getObject(t).key(n>>>0),a=isLikeNone(_)?0:passStringToWasm0(_,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=a})),t.wbg.__wbg_get_2b9ca76f8a2a9808=handleError((function(e,t,n,_){var a=getObject(t)[getStringFromWasm0(n,_)],r=isLikeNone(a)?0:passStringToWasm0(a,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),c=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=c,getInt32Memory0()[e/4+0]=r})),t.wbg.__wbg_checked_1596e4b21132859e=function(e){return getObject(e).checked},t.wbg.__wbg_error_c81c8d172df3cb18=function(e){console.error(getObject(e))},t.wbg.__wbg_textContent_04ca15fd48e65efe=function(e,t){var n=getObject(t).textContent,_=isLikeNone(n)?0:passStringToWasm0(n,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),a=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=a,getInt32Memory0()[e/4+0]=_},t.wbg.__wbg_namespaceURI_e79b3e59fe6e51eb=function(e,t){var n=getObject(t).namespaceURI,_=isLikeNone(n)?0:passStringToWasm0(n,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),a=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=a,getInt32Memory0()[e/4+0]=_},t.wbg.__wbg_getAttributeNames_cead3b75a84b43fc=function(e){return addHeapObject(getObject(e).getAttributeNames())},t.wbg.__wbg_forEach_249f930fcdfb1158=function(e,t,n){try{var _={a:t,b:n};getObject(e).forEach(((e,t,n)=>{const a=_.a;_.a=0;try{return __wbg_adapter_53(a,_.b,e,t,n)}finally{_.a=a}}))}finally{_.a=_.b=0}},t.wbg.__wbg_createElementNS_9e443d140d5b4a33=handleError((function(e,t,n,_,a){return addHeapObject(getObject(e).createElementNS(0===t?void 0:getStringFromWasm0(t,n),getStringFromWasm0(_,a)))})),t.wbg.__wbg_instanceof_HtmlElement_1557ca12085328d3=function(e){return getObject(e)instanceof HTMLElement},t.wbg.__wbg_focus_67ec816d6e2fd9da=handleError((function(e){getObject(e).focus()})),t.wbg.__wbg_closest_82f6c64c21f5b056=handleError((function(e,t,n){var _=getObject(e).closest(getStringFromWasm0(t,n));return isLikeNone(_)?0:addHeapObject(_)})),t.wbg.__wbg_instanceof_PopStateEvent_a695582c8f771c6d=function(e){return getObject(e)instanceof PopStateEvent},t.wbg.__wbg_state_2705fc307f614d01=function(e){return addHeapObject(getObject(e).state)},t.wbg.__wbg_log_8485ead621ceded9=function(e){console.log(getObject(e))},t.wbg.__wbg_documentElement_3feb6acab6237cb5=function(e){var t=getObject(e).documentElement;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_classList_8fccfe6c14d5fe6a=function(e){return addHeapObject(getObject(e).classList)},t.wbg.__wbg_remove_3c2cbdfc12d7b8ac=handleError((function(e,t,n){getObject(e).remove(getStringFromWasm0(t,n))})),t.wbg.__wbg_add_5317f65067cde517=handleError((function(e,t,n){getObject(e).add(getStringFromWasm0(t,n))})),t.wbg.__wbg_new0_8c7faee4e4e8144d=function(){return addHeapObject(new Date)},t.wbg.__wbg_getTime_da8516632da73fe1=function(e){return getObject(e).getTime()},t.wbg.__wbg_getTimezoneOffset_f1cc5fd62d20ac9a=function(e){return getObject(e).getTimezoneOffset()},t.wbg.__wbg_new_59cb74e423758ede=function(){return addHeapObject(new Error)},t.wbg.__wbg_stack_558ba5917b466edd=function(e,t){var n=passStringToWasm0(getObject(t).stack,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_error_4bb6c2a97407129a=function(e,t){try{console.error(getStringFromWasm0(e,t))}finally{wasm.__wbindgen_free(e,t)}},t.wbg.__wbg_self_f865985e662246aa=handleError((function(){return addHeapObject(self.self)})),t.wbg.__wbg_require_c59851dfa0dc7e78=handleError((function(e,t,n){return addHeapObject(getObject(e).require(getStringFromWasm0(t,n)))})),t.wbg.__wbg_crypto_bfb05100db79193b=function(e){return addHeapObject(getObject(e).crypto)},t.wbg.__wbg_msCrypto_f6dddc6ae048b7e2=function(e){return addHeapObject(getObject(e).msCrypto)},t.wbg.__wbg_newwithlength_e0c461e90217842c=function(e){return addHeapObject(new Uint8Array(e>>>0))},t.wbg.__wbindgen_is_undefined=function(e){return void 0===getObject(e)},t.wbg.__wbg_static_accessor_MODULE_39947eb3fe77895f=function(){return addHeapObject(_)},t.wbg.__wbg_length_8f15bbb4ecbf7e33=function(e){return getObject(e).length},t.wbg.__wbg_get_40375c2067f479fc=function(e,t){return addHeapObject(getObject(e)[t>>>0])},t.wbg.__wbg_self_eeabd9085c04fc17=handleError((function(){return addHeapObject(self.self)})),t.wbg.__wbg_window_f110c13310da2c8f=handleError((function(){return addHeapObject(window.window)})),t.wbg.__wbg_globalThis_a2669bee93faee43=handleError((function(){return addHeapObject(globalThis.globalThis)})),t.wbg.__wbg_global_a5584d717f4d6761=handleError((function(){return addHeapObject(global.global)})),t.wbg.__wbg_newnoargs_179d393e4626fcf7=function(e,t){return addHeapObject(new Function(getStringFromWasm0(e,t)))},t.wbg.__wbg_call_8487a9f580e47219=handleError((function(e,t){return addHeapObject(getObject(e).call(getObject(t)))})),t.wbg.__wbg_decodeURIComponent_d8e4320575c84fdb=handleError((function(e,t){return addHeapObject(decodeURIComponent(getStringFromWasm0(e,t)))})),t.wbg.__wbg_instanceof_HtmlProgressElement_099dd97139f55d04=function(e){return getObject(e)instanceof HTMLProgressElement},t.wbg.__wbg_instanceof_HtmlMeterElement_06595377c1159795=function(e){return getObject(e)instanceof HTMLMeterElement},t.wbg.__wbg_instanceof_HtmlLiElement_64c2eed46a738eb6=function(e){return getObject(e)instanceof HTMLLIElement},t.wbg.__wbg_instanceof_HtmlSelectElement_3d87bb1722a1aab7=function(e){return getObject(e)instanceof HTMLSelectElement},t.wbg.__wbg_instanceof_HtmlParamElement_4e70870350227791=function(e){return getObject(e)instanceof HTMLParamElement},t.wbg.__wbg_instanceof_HtmlOutputElement_e8f509036332b55f=function(e){return getObject(e)instanceof HTMLOutputElement},t.wbg.__wbg_instanceof_HtmlOptionElement_2d3bf8eac2e57ff6=function(e){return getObject(e)instanceof HTMLOptionElement},t.wbg.__wbg_instanceof_HtmlDataElement_d75c15e37f8e2330=function(e){return getObject(e)instanceof HTMLDataElement},t.wbg.__wbg_instanceof_HtmlButtonElement_3d160e69f22feabe=function(e){return getObject(e)instanceof HTMLButtonElement},t.wbg.__wbg_setsearch_52f207e366951e50=function(e,t,n){getObject(e).search=getStringFromWasm0(t,n)},t.wbg.__wbg_sethash_37821d172b510ae5=function(e,t,n){getObject(e).hash=getStringFromWasm0(t,n)},t.wbg.__wbg_href_2a6b9a755811ce1d=function(e,t){var n=passStringToWasm0(getObject(t).href,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_encodeURIComponent_6b3b34852c605918=function(e,t){return addHeapObject(encodeURIComponent(getStringFromWasm0(e,t)))},t.wbg.__wbg_new_fc60630260444d58=handleError((function(){return addHeapObject(new URLSearchParams)})),t.wbg.__wbg_append_08a087b5e1385fbb=function(e,t,n,_,a){getObject(e).append(getStringFromWasm0(t,n),getStringFromWasm0(_,a))},t.wbg.__wbg_toString_af6e7d79417c14d7=function(e){return addHeapObject(getObject(e).toString())},t.wbg.__wbg_createTextNode_6f0cfbce3a2487fb=function(e,t,n){return addHeapObject(getObject(e).createTextNode(getStringFromWasm0(t,n)))},t.wbg.__wbg_instanceof_Node_ddb79e680ddc3e56=function(e){return getObject(e)instanceof Node},t.wbg.__wbg_insertBefore_9ddb982d7f5d6824=handleError((function(e,t,n){return addHeapObject(getObject(e).insertBefore(getObject(t),getObject(n)))})),t.wbg.__wbg_replaceChild_a0f1c01252ffdc19=handleError((function(e,t,n){return addHeapObject(getObject(e).replaceChild(getObject(t),getObject(n)))})),t.wbg.__wbg_location_f8de588551329bf4=function(e){return addHeapObject(getObject(e).location)},t.wbg.__wbg_href_e45e989cb96ea8ec=handleError((function(e,t){var n=passStringToWasm0(getObject(t).href,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n})),t.wbg.__wbg_searchParams_77b66db69c0654a8=function(e){return addHeapObject(getObject(e).searchParams)},t.wbg.__wbg_from_87d2b71ada3c04df=function(e){return addHeapObject(Array.from(getObject(e)))},t.wbg.__wbg_activeElement_685445be4e13409a=function(e){var t=getObject(e).activeElement;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_is_736e7316d8240b8e=function(e,t){return Object.is(getObject(e),getObject(t))},t.wbg.__wbg_selectionStart_b489c31ded0be066=handleError((function(e,t){var n=getObject(t).selectionStart;getInt32Memory0()[e/4+1]=isLikeNone(n)?0:n,getInt32Memory0()[e/4+0]=!isLikeNone(n)})),t.wbg.__wbg_selectionEnd_44c065f6e75e8d0b=handleError((function(e,t){var n=getObject(t).selectionEnd;getInt32Memory0()[e/4+1]=isLikeNone(n)?0:n,getInt32Memory0()[e/4+0]=!isLikeNone(n)})),t.wbg.__wbg_setselectionStart_ba13947859e643bf=handleError((function(e,t,n){getObject(e).selectionStart=0===t?void 0:n>>>0})),t.wbg.__wbg_setselectionEnd_e82e09dad24796a2=handleError((function(e,t,n){getObject(e).selectionEnd=0===t?void 0:n>>>0})),t.wbg.__wbg_setvalue_db0a29ffda090b36=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_setvalue_56f53a1867a02211=function(e,t){getObject(e).value=t},t.wbg.__wbg_setvalue_6a24dd66b2682cf1=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_setvalue_0d978a515f3700e7=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_setvalue_b9aa240e8a4460ed=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_setvalue_2ab5b4baf8ab7ea4=function(e,t){getObject(e).value=t},t.wbg.__wbg_setvalue_4caf229f3d676809=function(e,t){getObject(e).value=t},t.wbg.__wbg_setvalue_caedeff380df343e=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_setvalue_d16d04b58702a5de=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_instanceof_HtmlMenuItemElement_e9e88b481e64a32b=function(e){return getObject(e)instanceof HTMLMenuItemElement},t.wbg.__wbg_setchecked_c59c4e3455f1971c=function(e,t){getObject(e).checked=0!==t},t.wbg.__wbg_setchecked_e661a3683e5ae32a=function(e,t){getObject(e).checked=0!==t},t.wbg.__wbg_subarray_8a52f1c1a11c02a8=function(e,t,n){return addHeapObject(getObject(e).subarray(t>>>0,n>>>0))},t.wbg.__wbg_getRandomValues_57e4008f45f0e105=handleError((function(e,t){getObject(e).getRandomValues(getObject(t))})),t.wbg.__wbg_length_2cfa674c2a529bc1=function(e){return getObject(e).length},t.wbg.__wbindgen_memory=function(){return addHeapObject(wasm.memory)},t.wbg.__wbg_buffer_e35e010c3ba9f945=function(e){return addHeapObject(getObject(e).buffer)},t.wbg.__wbg_new_139e70222494b1ff=function(e){return addHeapObject(new Uint8Array(getObject(e)))},t.wbg.__wbg_set_d771848e3c7935bb=function(e,t,n){getObject(e).set(getObject(t),n>>>0)},t.wbg.__wbg_randomFillSync_d90848a552cbd666=handleError((function(e,t,n){getObject(e).randomFillSync(getArrayU8FromWasm0(t,n))})),t.wbg.__wbindgen_string_get=function(e,t){const n=getObject(t);var _="string"==typeof n?n:void 0,a=isLikeNone(_)?0:passStringToWasm0(_,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=a},t.wbg.__wbindgen_debug_string=function(e,t){var n=passStringToWasm0(debugString(getObject(t)),wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbindgen_throw=function(e,t){throw new Error(getStringFromWasm0(e,t))},t.wbg.__wbg_instanceof_Window_fa4595281eb5ba83=function(e){return getObject(e)instanceof Window},t.wbg.__wbg_createElement_695120dd76150487=handleError((function(e,t,n){return addHeapObject(getObject(e).createElement(getStringFromWasm0(t,n)))})),t.wbg.__wbg_querySelector_5cba5f6b2ed68e05=handleError((function(e,t,n){var _=getObject(e).querySelector(getStringFromWasm0(t,n));return isLikeNone(_)?0:addHeapObject(_)})),t.wbg.__wbg_instanceof_Element_a4435f01fb4b890f=function(e){return getObject(e)instanceof Element},t.wbg.__wbg_tagName_1544173ec78f7f60=function(e,t){var n=passStringToWasm0(getObject(t).tagName,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_getAttribute_3d19bb0ff8904532=function(e,t,n,_){var a=getObject(t).getAttribute(getStringFromWasm0(n,_)),r=isLikeNone(a)?0:passStringToWasm0(a,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),c=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=c,getInt32Memory0()[e/4+0]=r},t.wbg.__wbg_removeAttribute_4bc290dbe2b7214f=handleError((function(e,t,n){getObject(e).removeAttribute(getStringFromWasm0(t,n))})),t.wbg.__wbg_setAttribute_fb8737b4573a65f8=handleError((function(e,t,n,_,a){getObject(e).setAttribute(getStringFromWasm0(t,n),getStringFromWasm0(_,a))})),t.wbg.__wbg_target_b5de46e2f66e3085=function(e){var t=getObject(e).target;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_addEventListener_9b66d58c2a9ba39a=handleError((function(e,t,n,_){getObject(e).addEventListener(getStringFromWasm0(t,n),getObject(_))})),t.wbg.__wbg_instanceof_HtmlInputElement_bcbf72cd9188bbf5=function(e){return getObject(e)instanceof HTMLInputElement},t.wbg.__wbg_type_4ba412665eff3f16=function(e,t){var n=passStringToWasm0(getObject(t).type,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_9c8f7cca5ded6671=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_setvalue_44d4bd0ac437904f=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_instanceof_HtmlTextAreaElement_1bd106832e7a2d85=function(e){return getObject(e)instanceof HTMLTextAreaElement},t.wbg.__wbg_setvalue_d3112c6f38991024=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_nodeType_4f9e1589e687efd3=function(e){return getObject(e).nodeType},t.wbg.__wbg_appendChild_9ba4c99688714f13=handleError((function(e,t){return addHeapObject(getObject(e).appendChild(getObject(t)))})),t.wbg.__wbg_removeChild_e02df31f6d70392a=handleError((function(e,t){return addHeapObject(getObject(e).removeChild(getObject(t)))})),t.wbg.__wbg_get_6b352442c688a85c=function(e,t){var n=getObject(e)[t>>>0];return isLikeNone(n)?0:addHeapObject(n)},t.wbg.__wbg_getItem_3fc9a85a5c86c097=handleError((function(e,t,n,_){var a=getObject(t).getItem(getStringFromWasm0(n,_)),r=isLikeNone(a)?0:passStringToWasm0(a,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),c=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=c,getInt32Memory0()[e/4+0]=r})),t.wbg.__wbg_setItem_99592651ffc703f6=handleError((function(e,t,n,_,a){getObject(e).setItem(getStringFromWasm0(t,n),getStringFromWasm0(_,a))})),t.wbg.__wbg_pathname_a3cd13b12f428948=function(e,t){var n=passStringToWasm0(getObject(t).pathname,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_hash_a323624f55087378=function(e,t){var n=passStringToWasm0(getObject(t).hash,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),_=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=_,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_newwithbase_c870f862e657414c=handleError((function(e,t,n,_){return addHeapObject(new URL(getStringFromWasm0(e,t),getStringFromWasm0(n,_)))})),t.wbg.__wbg_document_d8cce4c1031c64eb=function(e){var t=getObject(e).document;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_localStorage_a79a5d8ee7487fcb=handleError((function(e){var t=getObject(e).localStorage;return isLikeNone(t)?0:addHeapObject(t)})),t.wbg.__wbindgen_closure_wrapper275=function(e,t,n){return addHeapObject(makeMutClosure(e,t,14,__wbg_adapter_20))},t.wbg.__wbindgen_closure_wrapper277=function(e,t,n){return addHeapObject(makeMutClosure(e,t,18,__wbg_adapter_23))},("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:_}=await load(await e,t);return wasm=n.exports,init.__wbindgen_wasm_module=_,wasm.__wbindgen_start(),wasm}export default init;
