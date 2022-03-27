const addResourcesToCache = async (resources) => {
    const cache = await caches.open('c6dca8bc0a728116523cbfcd1c3a40f0');
    await cache.addAll(resources);
};

const putInCache = async (request, response) => {
    const cache = await caches.open('c6dca8bc0a728116523cbfcd1c3a40f0');
    await cache.put(request, response);
};

const cacheFirst = async ({ request, preloadResponsePromise, fallbackUrl }) => {
    // First try to get the resource from the cache
    const responseFromCache = await caches.match(request);
    if (responseFromCache) {
        return responseFromCache;
    }

    // Next try to use the preloaded response, if it's there
    const preloadResponse = await preloadResponsePromise;
    if (preloadResponse) {
        console.info('using preload response', preloadResponse);
        putInCache(request, preloadResponse.clone());
        return preloadResponse;
    }

    // Next try to get the resource from the network
    try {
        const responseFromNetwork = await fetch(request);
        // response may be used only once
        // we need to save clone to put one copy in cache
        // and serve second one
        putInCache(request, responseFromNetwork.clone());
        return responseFromNetwork;
    } catch (error) {
        const fallbackResponse = await caches.match(fallbackUrl);
        if (fallbackResponse) {
            return fallbackResponse;
        }
        // when even the fallback response is not available,
        // there is nothing we can do, but we must always
        // return a Response object
        return new Response('Network error happened', {
            status: 408,
            headers: { 'Content-Type': 'text/plain' },
        });
    }
};

const enableNavigationPreload = async () => {
    if (self.registration.navigationPreload) {
        // Enable navigation preloads!
        await self.registration.navigationPreload.enable();
    }
};

self.addEventListener('activate', (event) => {
    event.waitUntil(enableNavigationPreload());
});

self.addEventListener('install', (event) => {
    event.waitUntil(
        addResourcesToCache([
            '/img/slider/1.webp','/img/slider/1.jpg','/img/svg/testimonials/quote.svg','/img/svg/skills/rust.png','/img/svg/skills/vscode.webp','/img/svg/skills/vscode.png','/img/svg/skills/rust.webp','/img/svg/skills/tensorflow.png','/img/svg/skills/tensorflow.webp','/img/svg/process/brush.svg','/img/svg/process/energy.svg','/img/svg/process/target-2.svg','/img/svg/process/energy-2.svg','/img/svg/process/brush-2.svg','/img/svg/process/target.svg','/img/svg/service/web.svg','/img/svg/service/physics.svg','/img/svg/service/contact.svg','/img/svg/service/anchor.svg','/img/me-sqr.webp','/img/me-sqr.png','/img/subscribe/dots.jpg','/img/subscribe/dots.webp','/img/brushes/about/1.webp','/img/brushes/about/2.webp','/img/brushes/about/1.png','/img/brushes/about/2.png','/img/brushes/process/1.png','/img/brushes/process/3.png','/img/brushes/process/dark/1.png','/img/brushes/process/dark/3.png','/img/brushes/process/dark/2.png','/img/brushes/process/2.png','/img/brushes/news/1.png','/img/brushes/news/2.png','/img/brushes/partners/1.png','/img/brushes/service/1.webp','/img/brushes/service/5.webp','/img/brushes/service/2.webp','/img/brushes/service/6.webp','/img/brushes/service/1.png','/img/brushes/service/3.png','/img/brushes/service/4.webp','/img/brushes/service/3.webp','/img/brushes/service/5.png','/img/brushes/service/4.png','/img/brushes/service/dark/1.png','/img/brushes/service/dark/3.png','/img/brushes/service/dark/4.png','/img/brushes/service/dark/2.png','/img/brushes/service/2.png','/img/brushes/service/6.png','/img/service/1.webp','/img/service/2.webp','/img/service/4.jpg','/img/service/2.jpg','/img/service/1.jpg','/img/service/4.webp','/img/service/3.webp','/img/service/3.jpg','/sw.js','/font/fontello.woff2','/font/fontello.eot','/font/fontello.ttf','/font/fontello.svg','/font/fontello.woff','/index-ee5a00cef9645f0.js','/index.html','/manifest.webmanifest','/index-ee5a00cef9645f0_bg.wasm','/icons/favicon.ico','/icons/favicon-16x16.png','/icons/android-chrome-192x192.png','/icons/favicon-32x32.png','/icons/apple-touch-icon.png','/icons/android-chrome-512x512.png',
        ])
    );
});

self.addEventListener('fetch', (event) => {
    event.respondWith(
        cacheFirst({
            request: event.request,
            preloadResponsePromise: event.preloadResponse,
            fallbackUrl: '/',
        })
    );
});
