const APP_NAME = "triton-windstorm";
const CACHE_VERSION = "1.7.0";

const CACHE_PREFIX = APP_NAME + '-' + CACHE_VERSION;

self.addEventListener('install', function(event) {
	event.waitUntil(
		caches
			.open(CACHE_PREFIX)
			.then(function(cache) {
				return cache.addAll([
					'./',
					'./index.html',
					'./index.css',
					'./pkg/package.js',
					'./pkg/package_bg.wasm',
					'./icons/icon-32.png',
				]);
			})
			.catch(function(error) {
				throw error;
			})
	)
});

self.addEventListener('activate', function(event) {
	event.waitUntil(
		caches
			.keys()
			.then(function(keyList) {
				return Promise.all(
					keyList
						.map(function(key) {
							if(key != CACHE_PREFIX) {
								caches.delete(key)
							}
						})
				);
			})
			.catch(function(error) {
				throw error;
			})
	);
});

self.addEventListener('fetch', function(event) {
	event.respondWith(
		caches
			.match(event.request)
			.then(function(r) {
				return r || fetch(event.request).then(function(response) {
					return caches
						.open(CACHE_PREFIX)
						.then(function(cache) {
							cache.put(event.request, response.clone());
							return response;
						})
						.catch(function(error) {
							throw error;
						});
				})
				.catch(function(error) {
					throw error;
				});
			})
			.catch(function(error) {
				throw error;
			})
	);
});
