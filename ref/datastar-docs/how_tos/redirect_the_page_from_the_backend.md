# How to redirect the page from the backend

Source: https://data-star.dev/how_tos/redirect_the_page_from_the_backend

---

# How to redirect the page from the backend

Redirecting to another page is a common task that can be done from the backend by patching a `script` tag into the DOM using a [`datastar-patch-elements`](/reference/sse_events#datastar-patch-elements) SSE event. Since this results in a browser redirect, existing signals will *not* persist to the new page.

## Goal [#](#goal)

Our goal is to indicate to the user that they will be redirected, wait 3 seconds, and then redirect them to `/guide`, all from the backend.

Demo

Click to be redirected from the backend

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button data-on-click="@get('/how_tos/redirect_the_page_from_the_backend/redirect')">
   Click to be redirected from the backend
  </button>
  <p id="indicator">
  </p>
 </div>
</fieldset>
```

## Steps [#](#steps)

We'll place a `data-on-click` attribute on a button and use the `get` action to send a `GET` request to the backend. We'll include an empty indicator `div` to show the user that they will be redirected.

```html
1<button data-on-click="@get('/endpoint')">
2    Click to be redirected from the backend
3</button>
4<div id="indicator"></div>
```

We'll set up our backend to first send a `datastar-patch-elements` event with a populated indicator fragment, then wait 3 seconds, and then send another `datastar-patch-elements` SSE event to append a `script` tag that redirects the page.

```html
1event: datastar-patch-elements
2data: elements <div id="indicator">Redirecting in 3 seconds...</div>
3
4// Wait 3 seconds
5
6event: datastar-patch-elements
7data: selector body
8data: mode append
9data: elements <script>window.location.href = "/guide"</script>
```

All SDKs provide an `ExecuteScript` helper function that wraps the provided code in a `script` tag and patches it into the DOM.

```html
 1(require
 2  '[starfederation.datastar.clojure.api :as d*]
 3  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]]
 4  '[some.hiccup.library :refer [html]])
 5
 6
 7(defn handle [ring-request]
 8  (->sse-response ring-request
 9    {on-open
10      (fn [sse]
11        (d*/patch-elements! sse
12          (html [:div#indicator "Redirecting in 3 seconds..."]))
13        (Thread/sleep 3000)
14        (d*/execute-script! sse "window.location = \"/guide\"")
15        (d*/close-sse! sse)}))
```

```html
1using StarFederation.Datastar.DependencyInjection;
2
3app.MapGet("/redirect", async (IDatastarService datastarService) =>
4{
5    await datastarService.PatchElementsAsync("""<div id="indicator">Redirecting in 3 seconds...</div>""");
6    await Task.Delay(TimeSpan.FromSeconds(3));
7    await datastarService.ExecuteScriptAsync("""window.location = "/guide";""");
8});
```

```html
 1import (
 2    "time"
 3    "github.com/starfederation/datastar-go/datastar"
 4)
 5
 6sse := datastar.NewSSE(w, r)
 7sse.PatchElements(`
 8    <div id="indicator">Redirecting in 3 seconds...</div>
 9`)
10time.Sleep(3 * time.Second)
11sse.ExecuteScript(`
12    window.location = "/guide"
13`)
```

No example found for Java

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3$sse = new ServerSentEventGenerator();
 4$sse->patchElements(`
 5    <div id="indicator">Redirecting in 3 seconds...</div>
 6`);
 7sleep(3);
 8$sse->executeScript(`
 9    window.location = "/guide"
10`);
```

```html
1from datastar_py import ServerSentEventGenerator as SSE
2from datastar_py.sanic import datastar_response
3
4@app.get("/redirect")
5@datastar_response
6async def redirect_from_backend():
7    yield SSE.patch_elements('<div id="indicator">Redirecting in 3 seconds...</div>')
8    await asyncio.sleep(3)
9    yield SSE.execute_script('window.location = "/guide"')
```

```html
1datastar = Datastar.new(request:, response:)
2
3datastar.stream do |sse|
4  sse.patch_elements '<div id="indicator">Redirecting in 3 seconds...</div>'
5  sleep 3
6  sse.execute_script 'window.location = "/guide"'
7end
```

```html
1use datastar::prelude::*;
2use async_stream::stream;
3use core::time::Duration;
4
5Sse(stream! {
6    yield PatchElements::new("<div id='indicator'>Redirecting in 3 seconds...</div>").into();
7    tokio::time::sleep(core::time::Duration::from_secs(3)).await;
8    yield ExecuteScript::new("window.location = '/guide'").into();
9});
```

```html
 1import { createServer } from "node:http";
 2import { ServerSentEventGenerator } from "../npm/esm/node/serverSentEventGenerator.js";
 3
 4const server = createServer(async (req, res) => {
 5
 6  ServerSentEventGenerator.stream(req, res, async (sse) => {
 7    sse.patchElements(`
 8      <div id="indicator">Redirecting in 3 seconds...</div>
 9    `);
10
11    setTimeout(() => {
12      sse.executeScript(`window.location = "/guide"`);
13    }, 3000);
14  });
15});
```

Note that in Firefox, if a redirect happens within a `script` tag then the URL is *replaced*, rather than *pushed*, meaning that the previous URL won't show up in the back history (or back/forward navigation).

To work around this, you can wrap the redirect in a `setTimeout` function call. See [issue #529](https://github.com/starfederation/datastar/issues/529) for reference.

```html
 1(require
 2  '[starfederation.datastar.clojure.api :as d*]
 3  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]]
 4  '[some.hiccup.library :refer [html]])
 5
 6
 7(defn handle [ring-request]
 8  (->sse-response ring-request
 9    {on-open
10      (fn [sse]
11        (d*/patch-elements! sse
12          (html [:div#indicator "Redirecting in 3 seconds..."]))
13        (Thread/sleep 3000)
14        (d*/execute-script! sse
15          "setTimeout(() => window.location = \"/guide\")"
16        (d*/close-sse! sse))}))
```

```html
1using StarFederation.Datastar.DependencyInjection;
2
3app.MapGet("/redirect", async (IDatastarService datastarService) =>
4{
5    await datastarService.PatchElementsAsync("""<div id="indicator">Redirecting in 3 seconds...</div>""");
6    await Task.Delay(TimeSpan.FromSeconds(3));
7    await datastarService.ExecuteScriptAsync("""setTimeout(() => window.location = "/guide");""");
8});
```

```html
 1import (
 2    "time"
 3    "github.com/starfederation/datastar-go/datastar"
 4)
 5
 6sse := datastar.NewSSE(w, r)
 7sse.PatchElements(`
 8    <div id="indicator">Redirecting in 3 seconds...</div>
 9`)
10time.Sleep(3 * time.Second)
11sse.ExecuteScript(`
12    setTimeout(() => window.location = "/guide")
13`)
```

No example found for Java

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3$sse = new ServerSentEventGenerator();
 4$sse->patchElements(`
 5    <div id="indicator">Redirecting in 3 seconds...</div>
 6`);
 7sleep(3);
 8$sse->executeScript(`
 9    setTimeout(() => window.location = "/guide")
10`);
```

```html
1from datastar_py import ServerSentEventGenerator as SSE
2from datastar_py.sanic import datastar_response
3
4@app.get("/redirect")
5@datastar_response
6async def redirect_from_backend():
7    yield SSE.patch_elements('<div id="indicator">Redirecting in 3 seconds...</div>')
8    await asyncio.sleep(3)
9    yield SSE.execute_script('setTimeout(() => window.location = "/guide")')
```

```html
 1datastar = Datastar.new(request:, response:)
 2
 3datastar.stream do |sse|
 4  sse.patch_elements '<div id="indicator">Redirecting in 3 seconds...</div>'
 5
 6  sleep 3
 7
 8  sse.execute_script <<~JS
 9    setTimeout(() => {
10      window.location = '/guide'
11    })
12  JS
13end
```

```html
1use datastar::prelude::*;
2use async_stream::stream;
3use core::time::Duration;
4
5Sse(stream! {
6    yield PatchElements::new("<div id='indicator'>Redirecting in 3 seconds...</div>").into();
7    tokio::time::sleep(core::time::Duration::from_secs(3)).await;
8    yield ExecuteScript::new("setTimeout(() => window.location = '/guide')").into();
9});
```

```html
 1import { createServer } from "node:http";
 2import { ServerSentEventGenerator } from "../npm/esm/node/serverSentEventGenerator.js";
 3
 4const server = createServer(async (req, res) => {
 5
 6  ServerSentEventGenerator.stream(req, res, async (sse) => {
 7    sse.patchElements(`
 8      <div id="indicator">Redirecting in 3 seconds...</div>
 9    `);
10
11    setTimeout(() => {
12      sse.executeScript(`setTimeout(() => window.location = "/guide")`);
13    }, 3000);
14  });
15});
```

Some SDKs provide a helper method that automatically wraps the statement in a `setTimeout` function call, so you don't have to worry about doing so (you're welcome!).

```html
 1(require
 2  '[starfederation.datastar.clojure.api :as d*]
 3  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]]
 4  '[some.hiccup.library :refer [html]])
 5
 6
 7(defn handler [ring-request]
 8  (->sse-response ring-request
 9    {on-open
10      (fn [sse]
11        (d*/patch-elements! sse
12          (html [:div#indicator "Redirecting in 3 seconds..."]))
13        (Thread/sleep 3000)
14        (d*/redirect! sse "/guide")
15        (d*/close-sse! sse))}))
```

```html
1using StarFederation.Datastar.DependencyInjection;
2using StarFederation.Datastar.Scripts;
3
4app.MapGet("/redirect", async (IDatastarService datastarService) =>
5{
6    await datastarService.PatchElementsAsync("""<div id="indicator">Redirecting in 3 seconds...</div>""");
7    await Task.Delay(TimeSpan.FromSeconds(3));
8    await datastarService.Redirect("/guide");
9});
```

```html
 1import (
 2    "time"
 3    "github.com/starfederation/datastar-go/datastar"
 4)
 5
 6sse := datastar.NewSSE(w, r)
 7sse.PatchElements(`
 8    <div id="indicator">Redirecting in 3 seconds...</div>
 9`)
10time.Sleep(3 * time.Second)
11sse.Redirect("/guide")
```

No example found for Java

```html
1use starfederation\datastar\ServerSentEventGenerator;
2
3$sse = new ServerSentEventGenerator();
4$sse->patchElements(`
5    <div id="indicator">Redirecting in 3 seconds...</div>
6`);
7sleep(3);
8$sse->location('/guide');
```

```html
1from datastar_py import ServerSentEventGenerator as SSE
2from datastar_py.sanic import datastar_response
3
4@app.get("/redirect")
5@datastar_response
6async def redirect_from_backend():
7    yield SSE.patch_elements('<div id="indicator">Redirecting in 3 seconds...</div>')
8    await asyncio.sleep(3)
9    yield SSE.redirect("/guide")
```

```html
1datastar = Datastar.new(request:, response:)
2
3datastar.stream do |sse|
4  sse.patch_elements '<div id="indicator">Redirecting in 3 seconds...</div>'
5
6  sleep 3
7
8  sse.redirect '/guide'
9end
```

No example found for Rust

No example found for TypeScript

## Conclusion [#](#conclusion)

Redirecting to another page can be done from the backend thanks to the ability to patch `script` tags into the DOM using the [`datastar-patch-elements`](/reference/sse_events#datastar-patch-elements) SSE event, or to execute JavaScript using an SDK.