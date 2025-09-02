# Backend Requests

Source: https://data-star.dev/guide/backend_requests

---

# Backend Requests

Between [attributes](/reference/attributes) and [actions](/reference/actions), Datastar provides you with everything you need to build hypermedia-driven applications. Using this approach, the backend drives state to the frontend and acts as the single source of truth, determining what actions the user can take next.

## Sending Signals [#](#sending-signals)

By default, all signals (except for local signals whose keys begin with an underscore) are sent in an object with every backend request. When using a `GET` request, the signals are sent as a `datastar` query parameter, otherwise they are sent as a JSON body.

By sending **all** signals in every request, the backend has full access to the frontend state. This is by design. It is **not** recommended to send partial signals, but if you must, you can use the [`filterSignals`](/reference/actions#filterSignals) option to filter the signals sent to the backend.

### Nesting Signals [#](#nesting-signals)

Signals can be nested, making it easier to target signals in a more granular way on the backend.

Using dot-notation:

```html
1<div data-signals-foo.bar="1"></div>
```

Using object syntax:

```html
1<div data-signals="{foo: {bar: 1}}"></div>
```

Using two-way binding:

```html
1<input data-bind-foo.bar />
```

A practical use-case of nested signals is when you have repetition of state on a page. The following example tracks the open/closed state of a menu on both desktop and mobile devices, and the [toggleAll()](/reference/actions#toggleAll) action to toggle the state of all menus at once.

```html
1<div data-signals="{menu: {isOpen: {desktop: false, mobile: false}}}">
2    <button data-on-click="@toggleAll({include: /^menu\.isOpen\./})">
3        Open/close menu
4    </button>
5</div>
```

## Reading Signals [#](#reading-signals)

To read signals from the backend, JSON decode the `datastar` query param for `GET` requests, and the request body for all other methods.

All [SDKs](/reference/sdks) provide a helper function to read signals. Here's how you would read the nested signal `foo.bar` from an incoming request.

No example found for Clojure

```html
 1using StarFederation.Datastar.DependencyInjection;
 2
 3// Adds Datastar as a service
 4builder.Services.AddDatastar();
 5
 6public record Signals
 7{
 8    [JsonPropertyName("foo")] [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
 9    public FooSignals? Foo { get; set; } = null;
10
11    public record FooSignals
12    {
13        [JsonPropertyName("bar")] [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
14        public string? Bar { get; set; }
15    }
16}
17
18app.MapGet("/read-signals", async (IDatastarService datastarService) =>
19{
20    Signals? mySignals = await datastarService.ReadSignalsAsync<Signals>();
21    var bar = mySignals?.Foo?.Bar;
22});
```

```html
 1import ("github.com/starfederation/datastar-go/datastar")
 2
 3type Signals struct {
 4    Foo struct {
 5        Bar string `json:"bar"`
 6    } `json:"foo"`
 7}
 8
 9signals := &Signals{}
10if err := datastar.ReadSignals(request, signals); err != nil {
11    http.Error(w, err.Error(), http.StatusBadRequest)
12    return
13}
```

No example found for Java

```html
1use starfederation\datastar\ServerSentEventGenerator;
2
3// Reads all signals from the request.
4$signals = ServerSentEventGenerator::readSignals();
```

```html
1from datastar_py.fastapi import datastar_response, read_signals
2
3@app.get("/updates")
4@datastar_response
5async def updates(request: Request):
6    # Retrieve a dictionary with the current state of the signals from the frontend
7    signals = await read_signals(request)
```

```html
1# Setup with request
2datastar = Datastar.new(request:, response:)
3
4# Read signals
5some_signal = datastar.signals[:some_signal]
```

No example found for Rust

No example found for TypeScript

## SSE Events [#](#sse-events)

Datastar can stream zero or more [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE) from the web server to the browser. There's no special backend plumbing required to use SSE, just some special syntax. Fortunately, SSE is straightforward and [provides us with some advantages](/essays/event_streams_all_the_way_down), in addition to allowing us to send multiple events in a single response (in contrast to sending `text/html` or `application/json` responses).

First, set up your backend in the language of your choice. Familiarize yourself with [sending SSE events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#sending_events_from_the_server), or use one of the backend [SDKs](/reference/sdks) to get up and running even faster. We're going to use the SDKs in the examples below, which set the appropriate headers and format the events for us.

The following code would exist in a controller action endpoint in your backend.

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
14                                         "<div id=\"question\">What do you put in a toaster?</div>")
15
16                     ;; Patches signals
17                     (d*/patch-signals! sse "{response: '', answer: 'bread'}"))}))
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
 9    await datastarService.PatchElementsAsync(@"<div id=""question"">What do you put in a toaster?</div>");
10
11    // Patches signals.
12    await datastarService.PatchSignalsAsync(new { response = "", answer = "bread" });
13});
```

```html
 1import ("github.com/starfederation/datastar-go/datastar")
 2
 3// Creates a new `ServerSentEventGenerator` instance.
 4sse := datastar.NewSSE(w,r)
 5
 6// Patches elements into the DOM.
 7sse.PatchElements(
 8    `<div id="question">What do you put in a toaster?</div>`
 9)
10
11// Patches signals.
12sse.PatchSignals([]byte(`{response: '', answer: 'bread'}`))
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
 9    .data("<div id=\"question\">What do you put in a toaster?</div>")
10    .build()
11);
12
13// Patches signals.
14generator.send(PatchSignals.builder()
15    .data("{\"response\": \"\", \"answer\": \"\"}")
16    .build()
17);
```

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3// Creates a new `ServerSentEventGenerator` instance.
 4$sse = new ServerSentEventGenerator();
 5
 6// Patches elements into the DOM.
 7$sse->patchElements(
 8    '<div id="question">What do you put in a toaster?</div>'
 9);
10
11// Patches signals.
12$sse->patchSignals(['response' => '', 'answer' => 'bread']);
```

```html
1from datastar_py import ServerSentEventGenerator as SSE
2from datastar_py.litestar import DatastarResponse
3
4async def endpoint():
5    return DatastarResponse([
6        SSE.patch_elements('<div id="question">What do you put in a toaster?</div>'),
7        SSE.patch_signals({"response": "", "answer": "bread"})
8    ])
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
12  # Patches elements into the DOM
13  sse.patch_elements %(<div id="question">What do you put in a toaster?</div>)
14
15  # Patches signals
16  sse.patch_signals(response: '', answer: 'bread')
17end
```

```html
 1use datastar::prelude::*;
 2use async_stream::stream;
 3
 4Sse(stream! {
 5    // Patches elements into the DOM.
 6    yield PatchElements::new("<div id='question'>What do you put in a toaster?</div>").into();
 7
 8    // Patches signals.
 9    yield PatchSignals::new("{response: '', answer: 'bread'}").into();
10})
```

```html
1// Creates a new `ServerSentEventGenerator` instance (this also sends required headers)
2ServerSentEventGenerator.stream(req, res, (stream) => {
3      // Patches elements into the DOM.
4     stream.patchElements(`<div id="question">What do you put in a toaster?</div>`);
5
6     // Patches signals.
7     stream.patchSignals({'response':  '', 'answer': 'bread'});
8});
```

The `PatchElements()` function updates the provided HTML element into the DOM, replacing the element with `id="question"`. An element with the ID `question` must *already* exist in the DOM.

The `PatchSignals()` function updates the `response` and `answer` signals into the frontend signals.

With our backend in place, we can now use the `data-on-click` attribute to trigger the [`@get()`](/reference/actions#get) action, which sends a `GET` request to the `/actions/quiz` endpoint on the server when a button is clicked.

```html
 1<div
 2    data-signals="{response: '', answer: ''}"
 3    data-computed-correct="$response.toLowerCase() == $answer"
 4>
 5    <div id="question"></div>
 6    <button data-on-click="@get('/actions/quiz')">Fetch a question</button>
 7    <button
 8        data-show="$answer != ''"
 9        data-on-click="$response = prompt('Answer:') ?? ''"
10    >
11        BUZZ
12    </button>
13    <div data-show="$response != ''">
14        You answered "<span data-text="$response"></span>".
15        <span data-show="$correct">That is correct â</span>
16        <span data-show="!$correct">
17        The correct answer is "<span data-text="$answer"></span>" ð¤·
18        </span>
19    </div>
20</div>
```

Now when the `Fetch a question` button is clicked, the server will respond with an event to modify the `question` element in the DOM and an event to modify the `response` and `answer` signals. We're driving state from the backend!

Demo

...

Fetch a question BUZZ

You answered "". That is correct â The correct answer is "" ð¤·

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-computed-correct2="$response1.toLowerCase() == $answer1" data-signals="{response1: '', answer1: ''}" style="display: none">
  </div>
  <p id="question1">
   ...
  </p>
  <div style="display: flex; gap: var(--size-4)">
   <button class="warning large" data-on-click="@get('/guide/backend_requests/question')">
    Fetch a question
   </button>
   <button class="success large" data-on-click="$response1 = prompt('Answer:') ?? ''" data-show="$answer1 != ''">
    BUZZ
   </button>
  </div>
  <p data-show="$response1 != ''">
   You answered "
   <span data-text="$response1">
   </span>
   ".
   <span data-show="$correct2">
    That is correct â
   </span>
   <span data-show="!$correct2">
    The correct answer is "
    <span data-text="$answer1">
    </span>
    " ð¤·
   </span>
  </p>
 </div>
</fieldset>
```

### `data-indicator` [#](#data-indicator)

The [`data-indicator`](/reference/attributes#data-indicator) attribute sets the value of a signal to `true` while the request is in flight, otherwise `false`. We can use this signal to show a loading indicator, which may be desirable for slower responses.

```html
1<div id="question"></div>
2<button
3    data-on-click="@get('/actions/quiz')"
4    data-indicator-fetching
5>
6    Fetch a question
7</button>
8<div data-class-loading="$fetching" class="indicator"></div>
```

Demo

...

Fetch a question

![Indicator](/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif)

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <p id="question2">
   ...
  </p>
  <div style="display: flex; align-items: center; gap: var(--size-4)">
   <button class="warning large" data-indicator-fetching="" data-on-click="@get('/guide/backend_requests/question/slow')">
    Fetch a question
   </button>
   <div class="indicator" data-class-loading="$fetching">
    <img alt="Indicator" height="32" loading="" src="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif" srcset="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 1x, /cdn-cgi/image/format=auto,width=64/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 2x" width="32"/>
   </div>
  </div>
 </div>
</fieldset>
```

## Backend Actions [#](#backend-actions)

We're not limited to sending just `GET` requests. Datastar provides [backend actions](/reference/actions#backend-actions) for each of the methods available: `@get()`, `@post()`, `@put()`, `@patch()` and `@delete()`.

Here's how we can send an answer to the server for processing, using a `POST` request.

```html
1<button data-on-click="@post('/actions/quiz')">
2    Submit answer
3</button>
```

One of the benefits of using SSE is that we can send multiple events (patch elements and patch signals) in a single response.

```html
1(d*/patch-elements! sse "<div id=\"question\">...</div>")
2(d*/patch-elements! sse "<div id=\"instructions\">...</div>")
3(d*/patch-signals! sse "{answer: '...', prize: '...'}")
```

```html
1datastarService.PatchElementsAsync(@"<div id=""question"">...</div>");
2datastarService.PatchElementsAsync(@"<div id=""instructions"">...</div>");
3datastarService.PatchSignalsAsync(new { answer = "...", prize = "..." } );
```

```html
1sse.PatchElements(`<div id="question">...</div>`)
2sse.PatchElements(`<div id="instructions">...</div>`)
3sse.PatchSignals([]byte(`{answer: '...', prize: '...'}`))
```

```html
 1generator.send(PatchElements.builder()
 2    .data("<div id=\"question\">...</div>")
 3    .build()
 4);
 5generator.send(PatchElements.builder()
 6    .data("<div id=\"instructions\">...</div>")
 7    .build()
 8);
 9generator.send(PatchSignals.builder()
10    .data("{\"answer\": \"...\", \"prize\": \"...\"}")
11    .build()
12);
```

```html
1$sse->patchElements('<div id="question">...</div>');
2$sse->patchElements('<div id="instructions">...</div>');
3$sse->patchSignals(['answer' => '...', 'prize' => '...']);
```

```html
1return DatastarResponse([
2    SSE.patch_elements('<div id="question">...</div>'),
3    SSE.patch_elements('<div id="instructions">...</div>'),
4    SSE.patch_signals({"answer": "...", "prize": "..."})
5])
```

```html
1datastar.stream do |sse|
2  sse.patch_elements('<div id="question">...</div>')
3  sse.patch_elements('<div id="instructions">...</div>')
4  sse.patch_signals(answer: '...', prize: '...')
5end
```

```html
1yield PatchElements::new("<div id='question'>...</div>").into()
2yield PatchElements::new("<div id='instructions'>...</div>").into()
3yield PatchSignals::new("{answer: '...', prize: '...'}").into()
```

```html
1stream.patchElements('<div id="question">...</div>');
2stream.patchElements('<div id="instructions">...</div>');
3stream.patchSignals({'answer': '...', 'prize': '...'});
```

> In addition to your browser's dev tools, the [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to monitor and inspect SSE events received by Datastar.

Read more about SSE events in the [reference](/reference/sse_events).

## Congratulations [#](#congratulations)

You've actually read the entire guide! You should now know how to use Datastar to build reactive applications that communicate with the backend using backend requests and SSE events.

Feel free to dive into the [reference](/reference) and explore the [examples](/examples) next, to learn more about what you can do with Datastar.