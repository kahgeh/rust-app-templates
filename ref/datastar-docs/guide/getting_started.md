# Getting Started

Source: https://data-star.dev/guide/getting_started

---

# Getting Started

Datastar simplifies frontend development — allowing you to build backend-driven, interactive UIs using a **hypermedia-first** approach.

> Hypermedia refers to linked content like images, audio, and video — an extension of hypertext (you know, the "H" in HTML and HTTP).

Datastar offers backend-driven reactivity like [htmx](https://htmx.org/), and frontend-driven reactivity like [Alpine.js](https://alpinejs.dev/), in a lightweight framework that doesn't require any npm packages or other dependencies. It provides two primary functions:

1. Modify the DOM and state by sending events from your backend.
2. Build reactivity into your frontend using HTML attributes.

## Installation [#](#installation)

The quickest way to use Datastar is to include it using a `script` tag that fetches it from a CDN.

```html
1<script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@main/bundles/datastar.js"></script>
```

If you prefer to host the file yourself, download the [script](https://cdn.jsdelivr.net/gh/starfederation/datastar@main/bundles/datastar.js) or create your own bundle using the [bundler](/bundler), then include it from the appropriate path.

```html
1<script type="module" src="/path/to/datastar.js"></script>
```

## `data-*` [#](#data-*)

At the core of Datastar are [data-\*](https://developer.mozilla.org/en-US/docs/Web/HTML/How_to/Use_data_attributes) attributes (hence the name). They allow you to add reactivity to your frontend and interact with your backend in a declarative way.

> The Datastar [VSCode extension](https://marketplace.visualstudio.com/items?itemName=starfederation.datastar-vscode) and [IntelliJ plugin](https://plugins.jetbrains.com/plugin/26072-datastar-support) provide autocompletion for all `data-*` attributes.

The [`data-on`](/reference/attributes#data-on) attribute can be used to attach an event listener to an element and execute an expression whenever the event is triggered. The value of the attribute is a [Datastar expression](/guide/datastar_expressions) in which JavaScript can be used.

```html
1<button data-on-click="alert('I'm sorry, Dave. I'm afraid I can't do that.')">
2    Open the pod bay doors, HAL.
3</button>
```

Demo

Open the pod bay doors, HAL.

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="alert(`I'm sorry, Dave. I'm afraid I can't do that.`)">
   Open the pod bay doors, HAL.
  </button>
 </div>
</fieldset>
```

We'll explore more data attributes in the [next section of the guide](/guide/reactive_signals).

## Patching Elements [#](#patching-elements)

With Datastar, the backend *drives* the frontend by **patching** (adding, updating and removing) HTML elements in the DOM.

Datastar receives elements from the backend and manipulates the DOM using a morphing strategy (by default). Morphing ensures that only modified parts of the DOM are updated, preserving state and improving performance.

Datastar provides [actions](/reference/actions#backend-actions) for sending requests to the backend. The [`@get()`](/reference/actions#get) action sends a `GET` request to the provided URL using a [fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) request.

```html
1<button data-on-click="@get('/endpoint')">
2    Open the pod bay doors, HAL.
3</button>
4<div id="hal"></div>
```

> Actions in Datastar are helper functions that have the syntax `@actionName()`. Read more about actions in the [reference](/reference/actions).

If the response has a `content-type` of `text/html`, the top-level HTML elements will be morphed into the existing DOM based on the element IDs.

```html
1<div id="hal">
2    I'm sorry, Dave. I'm afraid I can't do that.
3</div>
```

We call this a "Patch Elements" event because multiple elements can be patched into the DOM at once.

Demo

Open the pod bay doors, HAL. `Waiting for an order...`

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="@get('/guide/getting_started/open_the_bay_doors_html')">
   Open the pod bay doors, HAL.
  </button>
  <code id="hal_html">
   Waiting for an order...
  </code>
  <div id="hal_html_reset">
  </div>
 </div>
</fieldset>
```

In the example above, the DOM must contain an element with a `hal` ID in order for morphing to work. Other [patching strategies](/reference/sse_events#datastar-patch-elements) are available, but morph is the best and simplest choice in most scenarios.

If the response has a `content-type` of `text/event-stream`, it can contain zero or more [SSE events](/reference/sse_events). The example above can be replicated using a `datastar-patch-elements` SSE event.

```html
1event: datastar-patch-elements
2data: elements <div id="hal">
3data: elements     I'm sorry, Dave. I'm afraid I can't do that.
4data: elements </div>
```

Because we can send as many events as we want in a stream, and because it can be a long-lived connection, we can extend the example above to first send HAL's response and then, after a few seconds, reset the text.

```html
1event: datastar-patch-elements
2data: elements <div id="hal">
3data: elements     I'm sorry, Dave. I'm afraid I can't do that.
4data: elements </div>
5
6event: datastar-patch-elements
7data: elements <div id="hal">
8data: elements     Waiting for an order...
9data: elements </div>
```

Demo

Open the pod bay doors, HAL. `Waiting for an order...`

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="@get('/guide/getting_started/open_the_bay_doors_sse')">
   Open the pod bay doors, HAL.
  </button>
  <code id="hal_sse">
   Waiting for an order...
  </code>
 </div>
</fieldset>
```

Here's the code to generate the SSE events above using the SDKs.

```html
 1;; Import the SDK's api and your adapter
 2(require
 3 '[starfederation.datastar.clojure.api :as d*]
 4 '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]])
 5
 6;; in a ring handler
 7(defn handler [request]
 8  ;; Create an SSE response
 9  (->sse-response request
10                  {on-open
11                   (fn [sse]
12                     ;; Patches elements into the DOM
13                     (d*/patch-elements! sse
14                                         "<div id=\"hal\">I'm sorry, Dave. I'm afraid I can't do that.</div>")
15                     (Thread/sleep 1000)
16                     (d*/patch-elements! sse
17                                         "<div id=\"hal\">Waiting for an order...</div>"))}))
```

```html
 1using StarFederation.Datastar.DependencyInjection;
 2
 3// Adds Datastar as a service
 4builder.Services.AddDatastar();
 5
 6app.MapGet("/", async (IDatastarService datastarService) =>
 7{
 8    // Patches elements into the DOM.
 9    await datastarService.PatchElementsAsync(@"<div id=""hal"">I'm sorry, Dave. I'm afraid I can't do that.</div>");
10
11    await Task.Delay(TimeSpan.FromSeconds(1));
12
13    await datastarService.PatchElementsAsync(@"<div id=""hal"">Waiting for an order...</div>");
14});
```

```html
 1import (
 2    "github.com/starfederation/datastar-go/datastar"
 3    time
 4)
 5
 6// Creates a new `ServerSentEventGenerator` instance.
 7sse := datastar.NewSSE(w,r)
 8
 9// Patches elements into the DOM.
10sse.PatchElements(
11    `<div id="hal">I'm sorry, Dave. I'm afraid I can't do that.</div>`
12)
13
14time.Sleep(1 * time.Second)
15
16sse.PatchElements(
17    `<div id="hal">Waiting for an order...</div>`
18)
```

```html
 1import starfederation.datastar.utils.ServerSentEventGenerator;
 2
 3// Creates a new `ServerSentEventGenerator` instance.
 4AbstractResponseAdapter responseAdapter = new HttpServletResponseAdapter(response);
 5ServerSentEventGenerator generator = new ServerSentEventGenerator(responseAdapter);
 6
 7// Patches elements into the DOM.
 8generator.send(PatchElements.builder()
 9    .data("<div id=\"hal\">I'm sorry, Dave. I'm afraid I can't do that.</div>")
10    .build()
11);
12
13Thread.sleep(1000);
14
15generator.send(PatchElements.builder()
16    .data("<div id=\"hal\">Waiting for an order...</div>")
17    .build()
18);
```

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3// Creates a new `ServerSentEventGenerator` instance.
 4$sse = new ServerSentEventGenerator();
 5
 6// Patches elements into the DOM.
 7$sse->patchElements(
 8    '<div id="hal">I'm sorry, Dave. I'm afraid I can't do that.</div>'
 9);
10
11sleep(1)
12
13$sse->patchElements(
14    '<div id="hal">Waiting for an order...</div>'
15);
```

```html
1from datastar_py import ServerSentEventGenerator as SSE
2from datastar_py.sanic import datastar_response
3
4@app.get('/open-the-bay-doors')
5@datastar_response
6async def open_doors(request):
7    yield SSE.patch_elements('<div id="hal">I'm sorry, Dave. I'm afraid I can't do that.</div>')
8    await asyncio.sleep(1)
9    yield SSE.patch_elements('<div id="hal">Waiting for an order...</div>')
```

```html
 1require 'datastar'
 2
 3# Create a Datastar::Dispatcher instance
 4
 5datastar = Datastar.new(request:, response:)
 6
 7# In a Rack handler, you can instantiate from the Rack env
 8# datastar = Datastar.from_rack_env(env)
 9
10# Start a streaming response
11datastar.stream do |sse|
12  # Patches elements into the DOM.
13  sse.patch_elements %(<div id="hal">I'm sorry, Dave. I'm afraid I can't do that.</div>)
14
15  sleep 1
16  
17  sse.patch_elements %(<div id="hal">Waiting for an order...</div>)
18end
```

```html
 1use async_stream::stream;
 2use datastar::prelude::*;
 3use std::thread;
 4use std::time::Duration;
 5
 6Sse(stream! {
 7    // Patches elements into the DOM.
 8    yield PatchElements::new("<div id='hal'>I'm sorry, Dave. I'm afraid I can't do that.</div>").into();
 9
10    thread::sleep(Duration::from_secs(1));
11    
12    yield PatchElements::new("<div id='hal'>Waiting for an order...</div>").into();
13})
```

```html
1// Creates a new `ServerSentEventGenerator` instance (this also sends required headers)
2ServerSentEventGenerator.stream(req, res, (stream) => {
3    // Patches elements into the DOM.
4    stream.patchElements(`<div id="hal">I'm sorry, Dave. I'm afraid I can't do that.</div>`);
5
6    setTimeout(() => {
7        stream.patchElements(`<div id="hal">Waiting for an order...</div>`);
8    }, 1000);
9});
```

> In addition to your browser's dev tools, the [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to monitor and inspect SSE events received by Datastar.

We'll cover event streams and [SSE events](/reference/sse_events) in more detail [later in the guide](/guide/backend_requests), but as you can see, they are just plain text events with a special syntax, made simpler by the [SDKs](/reference/sdks).