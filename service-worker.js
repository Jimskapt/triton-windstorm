const APP_NAME="triton-windstorm",CACHE_VERSION="1.12.0",CACHE_PREFIX=APP_NAME+"-1.12.0";self.addEventListener("install",(function(t){t.waitUntil(caches.open(CACHE_PREFIX).then((function(t){return t.addAll(["./","./index.html","./index.css","./pkg/package.js","./pkg/package_bg.wasm","./icons/icon-32.png"])})).catch((function(t){throw t})))})),self.addEventListener("activate",(function(t){t.waitUntil(caches.keys().then((function(t){return Promise.all(t.map((function(t){t!=CACHE_PREFIX&&caches.delete(t)})))})).catch((function(t){throw t})))})),self.addEventListener("fetch",(function(t){t.respondWith(caches.match(t.request).then((function(n){return n||fetch(t.request).then((function(n){return caches.open(CACHE_PREFIX).then((function(e){return e.put(t.request,n.clone()),n})).catch((function(t){throw t}))})).catch((function(t){throw t}))})).catch((function(t){throw t})))}));
