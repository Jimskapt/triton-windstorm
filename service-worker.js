const APP_NAME = "triton-windstorm";
const CACHE_VERSION = "1.5.3";

const CACHE_PREFIX = APP_NAME + '-' + CACHE_VERSION;

self.addEventListener('activate', function(event) {
	event.waitUntil(
		caches
			.open(CACHE_PREFIX)
			.then(function(cache) {
				return cache.addAll([
					'index.html',
					'index.css',
					'pkg/package.js',
					'pkg/package_bg.wasm',
					'icons/icon-32.png',
				]);
			})
			.catch(function(error) {
				console.error('SW install error :', error);

				throw error;
			})
	)
});

self.addEventListener('fetch', function(event) {
	event.respondWith(
		caches
			.open(CACHE_PREFIX)
			.then(function(cache) {
				return cache.match(event.request);
			})
			.catch(function(error) {
				console.error('SW open error :', error);

				throw error;
			})
	);
});
