self.addEventListener("install", (event) => {
    event.waitUntil(
        caches.open("pwa-cache").then((cache) => {
            return cache.addAll(["/", "/index.html", "/main.js", "/img/logo_192x192", "/img/logo_512x512", "/pkg/index.js", "/pkg/index_bg.wasm"]);
        })
    );
});

self.addEventListener("fetch", (event) => {
    event.respondWith(
        caches.match(event.request).then((response) => {
            return response || fetch(event.request);
        })
    );
});
