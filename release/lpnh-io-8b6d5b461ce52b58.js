let P=3,U=4,N=`function`,O=1,R=`string`,S=`Object`,J=`utf-8`,H=null,I=`undefined`,L=0,K=Error,Q=FinalizationRegistry,X=Object,W=Object.getPrototypeOf,T=Promise,M=Uint8Array,V=globalThis,G=undefined;var c=(b=>{const c=a.__externref_table_alloc();a.__wbindgen_export_1.set(c,b);return c});var C=((a,b)=>{});var b=(a=>a===G||a===H);var A=(async(a,b)=>{if(typeof Response===N&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===N){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var h=((a,b)=>{a=a>>>L;return e.decode(g().subarray(a,a+ b))});var E=(b=>{if(a!==G)return a;if(typeof b!==I){if(W(b)===X.prototype){({module:b}=b)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const c=B();C(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return D(d,b)});var n=(()=>{if(m===H||m.buffer.detached===!0||m.buffer.detached===G&&m.buffer!==a.memory.buffer){m=new DataView(a.memory.buffer)};return m});var q=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==H){return `${a}`};if(b==R){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==H){return `Symbol`}else{return `Symbol(${b})`}};if(b==N){const b=a.name;if(typeof b==R&&b.length>L){return `Function(${b})`}else{return `Function`}};if(Array.isArray(a)){const b=a.length;let c=`[`;if(b>L){c+=q(a[L])};for(let d=O;d<b;d++){c+=`, `+ q(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c&&c.length>O){d=c[O]}else{return toString.call(a)};if(d==S){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return S}};if(a instanceof K){return `${a.name}: ${a.message}\n${a.stack}`};return d});var p=((b,c,d,e)=>{const f={a:b,b:c,cnt:O,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=L;try{return e(c,f.b,...b)}finally{if(--f.cnt===L){a.__wbindgen_export_6.get(f.dtor)(c,f.b);o.unregister(f)}else{f.a=c}}};g.original=f;o.register(g,f,f);return g});var s=((b,c,d,e)=>{a.closure50_externref_shim(b,c,d,e)});var D=((b,c)=>{a=b.exports;F.__wbindgen_wasm_module=c;m=H;f=H;a.__wbindgen_start();return a});var r=((b,c,d)=>{a.closure33_externref_shim(b,c,d)});function d(b,d){try{return b.apply(this,d)}catch(b){const d=c(b);a.__wbindgen_exn_store(d)}}var l=((a,b,c)=>{if(c===G){const c=j.encode(a);const d=b(c.length,O)>>>L;g().subarray(d,d+ c.length).set(c);i=c.length;return d};let d=a.length;let e=b(d,O)>>>L;const f=g();let h=L;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;f[e+ h]=b};if(h!==d){if(h!==L){a=a.slice(h)};e=c(e,d,d=h+ a.length*P,O)>>>L;const b=g().subarray(e+ h,e+ d);const f=k(a,b);h+=f.written;e=c(e,d,h,O)>>>L};i=h;return e});var F=(async(b)=>{if(a!==G)return a;if(typeof b!==I){if(W(b)===X.prototype){({module_or_path:b}=b)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof b===I){b=new URL(`lpnh-io_bg.wasm`,import.meta.url)};const c=B();if(typeof b===R||typeof Request===N&&b instanceof Request||typeof URL===N&&b instanceof URL){b=fetch(b)};C(c);const {instance:d,module:e}=await A(await b,c);return D(d,e)});var g=(()=>{if(f===H||f.byteLength===L){f=new M(a.memory.buffer)};return f});var B=(()=>{const e={};e.wbg={};e.wbg.__wbg_body_942ea927546a04ba=(a=>{const d=a.body;return b(d)?L:c(d)});e.wbg.__wbg_buffer_09165b52af8c5237=(a=>{const b=a.buffer;return b});e.wbg.__wbg_buffer_609cc3eee51ed158=(a=>{const b=a.buffer;return b});e.wbg.__wbg_byobRequest_77d9adf63337edfb=(a=>{const d=a.byobRequest;return b(d)?L:c(d)});e.wbg.__wbg_byteLength_e674b853d9c77e1d=(a=>{const b=a.byteLength;return b});e.wbg.__wbg_byteOffset_fd862df290ef848d=(a=>{const b=a.byteOffset;return b});e.wbg.__wbg_call_672a4d21634d4a24=function(){return d(((a,b)=>{const c=a.call(b);return c}),arguments)};e.wbg.__wbg_call_7cccdd69e0791ae2=function(){return d(((a,b,c)=>{const d=a.call(b,c);return d}),arguments)};e.wbg.__wbg_cloneNode_a8ce4052a2c37536=function(){return d(((a,b)=>{const c=a.cloneNode(b!==L);return c}),arguments)};e.wbg.__wbg_close_304cc1fef3466669=function(){return d((a=>{a.close()}),arguments)};e.wbg.__wbg_close_5ce03e29be453811=function(){return d((a=>{a.close()}),arguments)};e.wbg.__wbg_content_537e4105afcd9cee=(a=>{const b=a.content;return b});e.wbg.__wbg_createElementNS_914d752e521987da=function(){return d(((a,b,c,d,e)=>{const f=a.createElementNS(b===L?G:h(b,c),h(d,e));return f}),arguments)};e.wbg.__wbg_createElement_8c9931a732ee2fea=function(){return d(((a,b,c)=>{const d=a.createElement(h(b,c));return d}),arguments)};e.wbg.__wbg_debug_3cb59063b29f58c1=(a=>{console.debug(a)});e.wbg.__wbg_document_d249400bd7bd996d=(a=>{const d=a.document;return b(d)?L:c(d)});e.wbg.__wbg_enqueue_bb16ba72f537dc9e=function(){return d(((a,b)=>{a.enqueue(b)}),arguments)};e.wbg.__wbg_error_524f506f44df1645=(a=>{console.error(a)});e.wbg.__wbg_error_7534b8e9a36f1ab4=((b,c)=>{let d;let e;try{d=b;e=c;console.error(h(b,c))}finally{a.__wbindgen_free(d,e,O)}});e.wbg.__wbg_firstElementChild_d75d385f5abd1414=(a=>{const d=a.firstElementChild;return b(d)?L:c(d)});e.wbg.__wbg_info_3daf2e093e091b66=(a=>{console.info(a)});e.wbg.__wbg_insertBefore_c181fb91844cd959=function(){return d(((a,b,c)=>{const d=a.insertBefore(b,c);return d}),arguments)};e.wbg.__wbg_instanceof_Window_def73ea0955fc569=(a=>{let b;try{b=a instanceof Window}catch(a){b=!1}const c=b;return c});e.wbg.__wbg_length_a446193dc22c12f8=(a=>{const b=a.length;return b});e.wbg.__wbg_log_c222819a41e063d3=(a=>{console.log(a)});e.wbg.__wbg_new_23a2665fac83c611=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=L;try{return s(d,c.b,a,b)}finally{c.a=d}};const e=new T(d);return e}finally{c.a=c.b=L}});e.wbg.__wbg_new_8a6f238a6ece86ea=(()=>{const a=new K();return a});e.wbg.__wbg_new_c68d7209be747379=((a,b)=>{const c=new K(h(a,b));return c});e.wbg.__wbg_newnoargs_105ed471475aaf50=((a,b)=>{const c=new Function(h(a,b));return c});e.wbg.__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a=((a,b,c)=>{const d=new M(a,b>>>L,c>>>L);return d});e.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5=(a=>{queueMicrotask(a)});e.wbg.__wbg_queueMicrotask_d3219def82552485=(a=>{const b=a.queueMicrotask;return b});e.wbg.__wbg_resolve_4851785c9c5f573d=(a=>{const b=T.resolve(a);return b});e.wbg.__wbg_respond_1f279fa9f8edcb1c=function(){return d(((a,b)=>{a.respond(b>>>L)}),arguments)};e.wbg.__wbg_setAttribute_2704501201f15687=function(){return d(((a,b,c,d,e)=>{a.setAttribute(h(b,c),h(d,e))}),arguments)};e.wbg.__wbg_set_65595bdd868b3009=((a,b,c)=>{a.set(b,c>>>L)});e.wbg.__wbg_setinnerHTML_31bde41f835786f7=((a,b,c)=>{a.innerHTML=h(b,c)});e.wbg.__wbg_stack_0ed75d68575b0f3c=((b,c)=>{const d=c.stack;const e=l(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const f=i;n().setInt32(b+ U*O,f,!0);n().setInt32(b+ U*L,e,!0)});e.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07=(()=>{const a=typeof global===I?H:global;return b(a)?L:c(a)});e.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0=(()=>{const a=typeof V===I?H:V;return b(a)?L:c(a)});e.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819=(()=>{const a=typeof self===I?H:self;return b(a)?L:c(a)});e.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40=(()=>{const a=typeof window===I?H:window;return b(a)?L:c(a)});e.wbg.__wbg_then_44b73946d2fb3e7d=((a,b)=>{const c=a.then(b);return c});e.wbg.__wbg_view_fd8a56e8983f448d=(a=>{const d=a.view;return b(d)?L:c(d)});e.wbg.__wbg_warn_4ca3906c248c47c4=(a=>{console.warn(a)});e.wbg.__wbindgen_cb_drop=(a=>{const b=a.original;if(b.cnt--==O){b.a=L;return !0};const c=!1;return c});e.wbg.__wbindgen_closure_wrapper1383=((a,b,c)=>{const d=p(a,b,34,r);return d});e.wbg.__wbindgen_debug_string=((b,c)=>{const d=q(c);const e=l(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const f=i;n().setInt32(b+ U*O,f,!0);n().setInt32(b+ U*L,e,!0)});e.wbg.__wbindgen_init_externref_table=(()=>{const b=a.__wbindgen_export_1;const c=b.grow(U);b.set(L,G);b.set(c+ L,G);b.set(c+ O,H);b.set(c+ 2,!0);b.set(c+ P,!1)});e.wbg.__wbindgen_is_function=(a=>{const b=typeof a===N;return b});e.wbg.__wbindgen_is_undefined=(a=>{const b=a===G;return b});e.wbg.__wbindgen_memory=(()=>{const b=a.memory;return b});e.wbg.__wbindgen_string_new=((a,b)=>{const c=h(a,b);return c});e.wbg.__wbindgen_throw=((a,b)=>{throw new K(h(a,b))});return e});let a;const e=typeof TextDecoder!==I?new TextDecoder(J,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw K(`TextDecoder not available`)}};if(typeof TextDecoder!==I){e.decode()};let f=H;let i=L;const j=typeof TextEncoder!==I?new TextEncoder(J):{encode:()=>{throw K(`TextEncoder not available`)}};const k=typeof j.encodeInto===N?((a,b)=>j.encodeInto(a,b)):((a,b)=>{const c=j.encode(a);b.set(c);return {read:a.length,written:c.length}});let m=H;const o=typeof Q===I?{register:()=>{},unregister:()=>{}}:new Q(b=>{a.__wbindgen_export_6.get(b.dtor)(b.a,b.b)});const t=[`bytes`];const u=typeof Q===I?{register:()=>{},unregister:()=>{}}:new Q(b=>a.__wbg_intounderlyingbytesource_free(b>>>L,O));class v{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=L;u.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b,L)}type(){const b=a.intounderlyingbytesource_type(this.__wbg_ptr);return t[b]}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>L}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,b)}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,b);return c}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const w=typeof Q===I?{register:()=>{},unregister:()=>{}}:new Q(b=>a.__wbg_intounderlyingsink_free(b>>>L,O));class x{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=L;w.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b,L)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,b);return c}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return c}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,b);return d}}const y=typeof Q===I?{register:()=>{},unregister:()=>{}}:new Q(b=>a.__wbg_intounderlyingsource_free(b>>>L,O));class z{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=L;y.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b,L)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,b);return c}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default F;export{v as IntoUnderlyingByteSource,x as IntoUnderlyingSink,z as IntoUnderlyingSource,E as initSync}