const addResourcesToCache=async(resources)=>{const cache=await caches.open('1e867cb5fb4b121bd048edb377a68e50');await cache.addAll(resources)};const putInCache=async(request,response)=>{const cache=await caches.open('1e867cb5fb4b121bd048edb377a68e50');await cache.put(request,response)};const deleteCache=async key=>{await caches.delete(key)};const deleteOldCaches=async()=>{const cacheKeepList=['1e867cb5fb4b121bd048edb377a68e50'];const keyList=await caches.keys();const cachesToDelete=keyList.filter(key=>!cacheKeepList.includes(key));await Promise.all(cachesToDelete.map(deleteCache))};const cacheFirst=async({request,preloadResponsePromise,fallbackUrl})=>{const responseFromCache=await caches.match(request);if(responseFromCache){return responseFromCache}const preloadResponse=await preloadResponsePromise;if(preloadResponse){console.info('using preload response',preloadResponse);putInCache(request,preloadResponse.clone());return preloadResponse}try{const responseFromNetwork=await fetch(request);putInCache(request,responseFromNetwork.clone());return responseFromNetwork}catch(error){const fallbackResponse=await caches.match(fallbackUrl);if(fallbackResponse){return fallbackResponse}return new Response('Network error happened',{status:408,headers:{'Content-Type':'text/plain'},})}};const enableNavigationPreload=async()=>{if(self.registration.navigationPreload){await self.registration.navigationPreload.enable()}};self.addEventListener('activate',(event)=>{event.waitUntil(enableNavigationPreload());event.waitUntil(deleteOldCaches())});self.addEventListener('install',(event)=>{event.waitUntil(addResourcesToCache(['/img/github.svg','/img/slider/1.webp','/img/slider/1.jpg','/img/svg/testimonials/quote.svg','/img/svg/skills/rust.png','/img/svg/skills/vscode.webp','/img/svg/skills/vscode.png','/img/svg/skills/rust.webp','/img/svg/skills/tensorflow.png','/img/svg/skills/tensorflow.webp','/img/svg/process/brush.svg','/img/svg/process/energy.svg','/img/svg/process/target-2.svg','/img/svg/process/energy-2.svg','/img/svg/process/brush-2.svg','/img/svg/process/target.svg','/img/svg/service/web.svg','/img/svg/service/physics.svg','/img/svg/service/contact.svg','/img/svg/service/anchor.svg','/img/me-sqr.webp','/img/me-sqr.png','/img/subscribe/dots.jpg','/img/subscribe/dots.webp','/img/brushes/about/1.webp','/img/brushes/about/2.webp','/img/brushes/about/1.png','/img/brushes/about/2.png','/img/brushes/process/1.png','/img/brushes/process/3.png','/img/brushes/process/dark/1.png','/img/brushes/process/dark/3.png','/img/brushes/process/dark/2.png','/img/brushes/process/2.png','/img/brushes/news/1.png','/img/brushes/news/2.png','/img/brushes/partners/1.png','/img/brushes/service/1.webp','/img/brushes/service/5.webp','/img/brushes/service/2.webp','/img/brushes/service/6.webp','/img/brushes/service/1.png','/img/brushes/service/3.png','/img/brushes/service/4.webp','/img/brushes/service/3.webp','/img/brushes/service/5.png','/img/brushes/service/4.png','/img/brushes/service/dark/1.png','/img/brushes/service/dark/3.png','/img/brushes/service/dark/4.png','/img/brushes/service/dark/2.png','/img/brushes/service/2.png','/img/brushes/service/6.png','/img/service/1.webp','/img/service/2.webp','/img/service/4.jpg','/img/service/2.jpg','/img/service/1.jpg','/img/service/4.webp','/img/service/3.webp','/img/service/3.jpg','/sw.js','/font/fontello.woff2','/font/fontello.eot','/font/fontello.ttf','/font/fontello.svg','/font/fontello.woff','/service-worker.js','/service-worker_bg.wasm','/index.html','/manifest.webmanifest','/icons/favicon.ico','/icons/favicon-16x16.png','/icons/android-chrome-192x192.png','/icons/favicon-32x32.png','/icons/apple-touch-icon.png','/icons/android-chrome-512x512.png','/personal-webpage-2887c6fafe123014_bg.wasm','/personal-webpage-2887c6fafe123014.js',]))});self.addEventListener('fetch',(event)=>{event.respondWith(cacheFirst({request:event.request,preloadResponsePromise:event.preloadResponse,fallbackUrl:'/',}))})