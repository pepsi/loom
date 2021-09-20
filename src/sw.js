var CACHE_NAME = 'my-site-cache-v1';
var urlsToCache = [
  '/',
  '/index-f6df5e15a749cff6_bg.wasm',
  '/index-f6df5e15a749cff6.js',
  '/index.html'

];

self.addEventListener('install', function(event) {
  // Perform install steps
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(function(cache) {
        console.log('Opened cache');
        return cache.addAll(urlsToCache);
      })
  );
});

// self.addEventListener('fetch', function(event) {
//     event.respondWith(
//       caches.match(event.request)
//         .then(function(response) {
//           // Cache hit - return response
//           if (response) {
//               console.log("Using sw  cache!")
//             return response;
//           }
//           console.log("Fetching!!!", event.request.url)
//           return fetch(event.request);
//         }
//       )
//     );
//   });
  
self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.open(CACHE_NAME).then(function(cache) {
      return cache.match(event.request).then(function (response) {
        return response || fetch(event.request).then(function(response) {
          cache.put(event.request, response.clone());
          return response;
        });
      });
    })
  );
});

