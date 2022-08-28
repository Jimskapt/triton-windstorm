const APP_NAME = "triton-windstorm";
const CACHE_VERSION = "1.13.3";

const CACHE_PREFIX = APP_NAME + "-" + CACHE_VERSION;

const CONTENT_TO_CACHE = [
	"./",
	"./index.html",
	"./index.css",
	"./favicon.ico",
	"./pkg/package.js",
	"./pkg/package_bg.wasm",
	"./icons/icon-32.png",
];

self.addEventListener("install", function(event) {
	console.log("service-worker", "install");

	event.waitUntil(
		async function() {
			const cache = await caches.open(CACHE_PREFIX);

			console.log("service-worker", "install", CACHE_PREFIX, CONTENT_TO_CACHE);

			await cache.addAll(CONTENT_TO_CACHE);
		}()
	);
});

self.addEventListener("activate", function(event) {
	console.log("service-worker", "activate");

	event.waitUntil(
		caches
			.keys()
			.then(function(keyList) {
				return Promise.all(
					keyList.map(function(key) {
						if(key === CACHE_PREFIX) {
							console.log("service-worker", "activate", "keep", key);
							return;
						} else {
							console.log("service-worker", "activate", "delete", key);
							return caches.delete(key);
						}
					})
				);
			})
	);
});

self.addEventListener("fetch", function(event) {
	const request = event.request;
	console.log("service-worker", "fetch", request.method, request.url);

	event.respondWith(
		async function() {
			const found_request = await caches.match(request);

			if(found_request) {
				console.log("service-worker", "fetch", request.method, request.url, "found in cache");

				return found_request;
			} else {
				console.log("service-worker", "fetch", request.method, request.url, "not found in cache");

				const response = await fetch(request);

				const cache_control = function() {
					const value = response.headers.get("Cache-Control");

					if(value !== null && value !== undefined) {
						return value.toLowerCase();
					} else {
						return "";
					}
				}();

				if(request.method.toUpperCase() !== "HEAD" && cache_control === "no-cache") {
					const cache = await caches.open(CACHE_PREFIX);

					console.log("service-worker", "fetch", "caching", request.method, request.url, "Cache-Control : " + cache_control);

					cache.put(request, response.clone());
				} else {
					console.log("service-worker", "fetch", "no caching", request.method, request.url, "Cache-Control : " + cache_control);
				}

				return response;
			}
		}()
	);
});
