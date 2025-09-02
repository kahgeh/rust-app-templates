# Reactive Signals

Source: https://data-star.dev/guide/reactive_signals

---

# Reactive Signals

In a hypermedia approach, the backend drives state to the frontend and acts as the primary source of truth. It's up to the backend to determine what actions the user can take next by patching appropriate elements in the DOM.

Sometimes, however, you may need access to frontend state that's driven by user interactions. Click, input and keydown events are some of the more common user events that you'll want your frontend to be able to react to.

Datastar uses *signals* to manage frontend state. You can think of signals as reactive variables that automatically track and propagate changes in and to [Datastar expressions](/guide/datastar_expressions). Signals are denoted using the `$` prefix.

## Data Attributes [#](#data-attributes)

Datastar allows you to add reactivity to your frontend and interact with your backend in a declarative way using [data-\*](https://developer.mozilla.org/en-US/docs/Web/HTML/How_to/Use_data_attributes) attributes.

> The Datastar [VSCode extension](https://marketplace.visualstudio.com/items?itemName=starfederation.datastar-vscode) and [IntelliJ plugin](https://plugins.jetbrains.com/plugin/26072-datastar-support) provide autocompletion for all `data-*` attributes.

### `data-bind` [#](#data-bind)

The [`data-bind`](/reference/attributes#data-bind) attribute sets up two-way data binding on any HTML element that receives user input or selections. These include `input`, `textarea`, `select`, `checkbox` and `radio` elements, as well as web components whose value can be made reactive.

```html
1<input data-bind-foo />
```

This creates a new signal that can be called using `$foo`, and binds it to the element's value. If either is changed, the other automatically updates.

You can accomplish the same thing passing the signal name as a *value*, an alternate syntax that might be more useful for some templating languages:

```html
1<input data-bind="foo" />
```

### `data-text` [#](#data-text)

The [`data-text`](/reference/attributes#data-text) attribute sets the text content of an element to the value of a signal. The `$` prefix is required to denote a signal.

```html
1<input data-bind-foo />
2<div data-text="$foo"></div>
```

Demo

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo1="" placeholder="Enter some input..." type="text"/>
  <pre><code data-text="$foo1"></code></pre>
 </div>
</fieldset>
```

The value of the `data-text` attribute is a [Datastar expression](/guide/datastar_expressions) that is evaluated, meaning that we can use JavaScript in it.

```html
1<input data-bind-foo />
2<div data-text="$foo.toUpperCase()"></div>
```

Demo

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo2="" placeholder="Enter some input..." type="text"/>
  <pre><code data-text="$foo2.toUpperCase()"></code></pre>
 </div>
</fieldset>
```

### `data-computed` [#](#data-computed)

The [`data-computed`](/reference/attributes#data-computed) attribute creates a new signal that is derived from a reactive expression. The computed signal is read-only, and its value is automatically updated when any signals in the expression are updated.

```html
1<input data-bind-foo />
2<div data-computed-repeated="$foo.repeat(2)" data-text="$repeated"></div>
```

This results in the `$repeated` signal's value always being equal to the value of the `$foo` signal repeated twice. Computed signals are useful for memoizing expressions containing other signals.

Demo

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo3="" placeholder="Enter some input..." type="text"/>
  <pre><code data-computed-repeated="$foo3.repeat(2)" data-text="$repeated"></code></pre>
 </div>
</fieldset>
```

### `data-show` [#](#data-show)

The [`data-show`](/reference/attributes#data-show) attribute can be used to show or hide an element based on whether an expression evaluates to `true` or `false`.

```html
1<input data-bind-foo />
2<button data-show="$foo != ''">Save</button>
```

This results in the button being visible only when the input value is *not* an empty string. This could also be shortened to `data-show="!$foo"`.

Demo

Save

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo4="" placeholder="Enter some input..." type="text"/>
  <button data-show="$foo4 != ''">
   Save
  </button>
 </div>
</fieldset>
```

### `data-class` [#](#data-class)

The [`data-class`](/reference/attributes#data-class) attribute allows us to add or remove an element's class based on an expression.

```html
1<input data-bind-foo />
2<button data-class-success="$foo != ''">
3    Save
4</button>
```

If the expression evaluates to `true`, the `success` class is added to the element; otherwise, it is removed.

Demo

Save

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo5="" placeholder="Enter some input..." type="text"/>
  <button data-class-success="$foo5 != ''">
   Save
  </button>
 </div>
</fieldset>
```

The `data-class` attribute can also be used to add or remove multiple classes from an element using a set of key-value pairs, where the keys represent class names and the values represent expressions.

```html
1<button data-class="{success: $foo != '', 'font-bold': $foo == 'bar'}">
2    Save
3</button>
```

### `data-attr` [#](#data-attr)

The [`data-attr`](/reference/attributes#data-attr) attribute can be used to bind the value of any HTML attribute to an expression.

```html
1<input data-bind-foo />
2<button data-attr-disabled="$foo == ''">
3    Save
4</button>
```

This results in a `disabled` attribute being given the value `true` whenever the input is an empty string.

Demo

Save

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo6="" placeholder="Enter some input..." type="text"/>
  <button data-attr-disabled="$foo6 == ''">
   Save
  </button>
 </div>
</fieldset>
```

The `data-attr` attribute can also be used to set the values of multiple attributes on an element using a set of key-value pairs, where the keys represent attribute names and the values represent expressions.

```html
1<button data-attr="{disabled: $foo == '', title: $foo}">Save</button>
```

### `data-signals` [#](#data-signals)

Signals are globally accessible from anywhere in the DOM. So far, we've created signals on the fly using `data-bind` and `data-computed`. If a signal is used without having been created, it will be created automatically and its value set to an empty string.

Another way to create signals is using the [`data-signals`](/reference/attributes#data-signals) attribute, which patches (adds, updates or removes) one or more signals into the existing signals.

```html
1<div data-signals-foo="1"></div>
```

Signals can be nested using dot-notation.

```html
1<div data-signals-form.foo="2"></div>
```

The `data-signals` attribute can also be used to patch multiple signals using a set of key-value pairs, where the keys represent signal names and the values represent expressions.

```html
1<div data-signals="{foo: 1, form: {foo: 2}}"></div>
```

### `data-on` [#](#data-on)

The [`data-on`](/reference/attributes#data-on) attribute can be used to attach an event listener to an element and run an expression whenever the event is triggered.

```html
1<input data-bind-foo />
2<button data-on-click="$foo = ''">
3    Reset
4</button>
```

This results in the `$foo` signal's value being set to an empty string whenever the button element is clicked. This can be used with any valid event name such as `data-on-keydown`, `data-on-mouseover`, etc. Custom events may also be used.

Demo

Reset

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <input data-bind-foo7="" placeholder="Enter some input..." type="text"/>
  <button data-on-click="$foo7 = ''">
   Reset
  </button>
 </div>
</fieldset>
```

These are just *some* of the attributes available in Datastar. For a complete list, see the [attribute reference](/reference/attributes).

## Frontend Reactivity [#](#frontend-reactivity)

Datastar's data attributes enable declarative signals and expressions, providing a simple yet powerful way to add reactivity to the frontend.

Datastar expressions are strings that are evaluated by Datastar [attributes](/reference/attributes) and [actions](/reference/actions). While they are similar to JavaScript, there are some important differences that are explained in the [next section of the guide](/guide/datastar_expressions).

```html
1<div data-signals-hal="'...'">
2    <button data-on-click="$hal = 'Affirmative, Dave. I read you.'">
3        HAL, do you read me?
4    </button>
5    <div data-text="$hal"></div>
6</div>
```

Demo

HAL, do you read me?

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-signals-hal="'...'" style="display: none">
  </div>
  <button class="warning large" data-on-click="$hal = 'Affirmative, Dave. I read you.'">
   HAL, do you read me?
  </button>
  <pre><code data-text="$hal"></code></pre>
 </div>
</fieldset>
```

See if you can figure out what the code below does based on what you've learned so far, *before* trying the demo below it.

```html
 1<div
 2    data-signals="{response: '', answer: 'bread'}"
 3    data-computed-correct="$response.toLowerCase() == $answer"
 4>
 5    <div id="question">What do you put in a toaster?</div>
 6    <button data-on-click="$response = prompt('Answer:') ?? ''">BUZZ</button>
 7    <div data-show="$response != ''">
 8        You answered "<span data-text="$response"></span>".
 9        <span data-show="$correct">That is correct â</span>
10        <span data-show="!$correct">
11        The correct answer is "
12        <span data-text="$answer"></span>
13        " ð¤·
14        </span>
15    </div>
16</div>
```

Demo

What do you put in a toaster?

BUZZ

You answered "". That is correct â The correct answer is "bread" ð¤·

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-computed-correct="$response.toLowerCase() == $answer" data-signals="{response: '', answer: 'bread'}" style="display: none">
  </div>
  <p id="question">
   What do you put in a toaster?
  </p>
  <button class="warning large" data-on-click="$response = prompt('Answer:') ?? ''">
   BUZZ
  </button>
  <p data-show="$response != ''">
   You answered "
   <span data-text="$response">
   </span>
   ".
   <span data-show="$correct">
    That is correct â
   </span>
   <span data-show="!$correct">
    The correct answer is "
    <span data-text="$answer">
     bread
    </span>
    " ð¤·
   </span>
  </p>
 </div>
</fieldset>
```

> The [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to inspect and filter current signals and view signal patch events in real-time.

## Patching Signals [#](#patching-signals)

Remember that in a hypermedia approach, the backend drives state to the frontend. Just like with elements, frontend signals can be **patched** (added, updated and removed) from the backend using [backend actions](/reference/actions#backend-actions).

```html
1<div data-signals-hal="'...'">
2    <button data-on-click="@get('/endpoint')">
3        HAL, do you read me?
4    </button>
5    <div data-text="$hal"></div>
6</div>
```

If a response has a `content-type` of `application/json`, the signal values are patched into the frontend signals.

We call this a "Patch Signals" event because multiple signals can be patched (using [JSON Merge Patch RFC 7396](https://datatracker.ietf.org/doc/rfc7396/)) into the existing signals.

```html
1{"hal": "Affirmative, Dave. I read you."}
```

Demo

HAL, do you read me? 

Reset

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="@get('/guide/reactive_signals/do_you_read_me_json')" data-signals-hal_json="'...'">
   HAL, do you read me?
  </button>
  <code data-text="$hal_json">
  </code>
  <div data-show="$show_hal_reset">
   <button data-on-click="$hal_json = '...'; $show_hal_reset = false">
    Reset
   </button>
  </div>
 </div>
</fieldset>
```

If the response has a `content-type` of `text/event-stream`, it can contain zero or more [SSE events](/reference/sse_events). The example above can be replicated using a `datastar-patch-signals` SSE event.

```html
1event: datastar-patch-signals
2data: signals {hal: 'Affirmative, Dave. I read you.'}
```

Because we can send as many events as we want in a stream, and because it can be a long-lived connection, we can extend the example above to first set the `hal` signal to an "affirmative" response and then, after a second, reset the signal.

```html
1event: datastar-patch-signals
2data: signals {hal: 'Affirmative, Dave. I read you.'}
3
4// Wait 1 second
5
6event: datastar-patch-signals
7data: signals {hal: '...'}
```

Demo

HAL, do you read me?

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="@get('/guide/reactive_signals/do_you_read_me_sse')" data-signals-hal_sse="'...'">
   HAL, do you read me?
  </button>
  <code data-text="$hal_sse">
  </code>
 </div>
</fieldset>
```

Here's the code to generate the SSE events above using the SDKs.

```html
 1;; Import the SDK's api and your adapter
 2(require
 3  '[starfederation.datastar.clojure.api :as d*]
 4  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]])
 5
 6;; in a ring handler
 7(defn handler [request]
 8  ;; Create an SSE response
 9  (->sse-response request
10                  {on-open
11                   (fn [sse]
12                     ;; Patches signal.
13                     (d*/patch-signals! sse "{hal: 'Affirmative, Dave. I read you.'}")
14                     (Thread/sleep 1000)
15                     (d*/patch-signals! sse "{hal: '...'}"))}))
```

```html
 1using StarFederation.Datastar.DependencyInjection;
 2
 3// Adds Datastar as a service
 4builder.Services.AddDatastar();
 5
 6app.MapGet("/hal", async (IDatastarService datastarService) =>
 7{
 8    // Patches signals.
 9    await datastarService.PatchSignalsAsync(new { hal = "Affirmative, Dave. I read you" });
10
11    await Task.Delay(TimeSpan.FromSeconds(3));
12
13    await datastarService.PatchSignalsAsync(new { hal = "..." });
14});
```

```html
 1import (
 2    "github.com/starfederation/datastar-go/datastar"
 3)
 4
 5// Creates a new `ServerSentEventGenerator` instance.
 6sse := datastar.NewSSE(w, r)
 7
 8// Patches signals
 9sse.PatchSignals([]byte(`{hal: 'Affirmative, Dave. I read you.'}`))
10
11time.Sleep(1 * time.Second)
12
13sse.PatchSignals([]byte(`{hal: '...'}`))
```

```html
 1import starfederation.datastar.utils.ServerSentEventGenerator;
 2
 3// Creates a new `ServerSentEventGenerator` instance.
 4AbstractResponseAdapter responseAdapter = new HttpServletResponseAdapter(response);
 5ServerSentEventGenerator generator = new ServerSentEventGenerator(responseAdapter);
 6
 7// Patches signals.
 8generator.send(PatchSignals.builder()
 9    .data("{\"hal\": \"Affirmative, Dave. I read you.\"}")
10    .build()
11);
12
13Thread.sleep(1000);
14
15generator.send(PatchSignals.builder()
16    .data("{\"hal\": \"...\"}")
17    .build()
18);
```

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3// Creates a new `ServerSentEventGenerator` instance.
 4$sse = new ServerSentEventGenerator();
 5
 6// Patches signals.
 7$sse->patchSignals(['hal' => 'Affirmative, Dave. I read you.']);
 8
 9sleep(1)
10
11$sse->patchSignals(['hal' => '...']);
```

```html
1from datastar_py import ServerSentEventGenerator as SSE
2from datastar_py.sanic import datastar_response
3
4@app.get('/do-you-read-me')
5@datastar_response
6async def open_doors(request):
7    yield SSE.patch_signals({"hal": "Affirmative, Dave. I read you."})
8    await asyncio.sleep(1)
9    yield SSE.patch_signals({"hal": "..."})
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
12  # Patches signals
13  sse.patch_signals(hal: 'Affirmative, Dave. I read you.')
14
15  sleep 1
16  
17  sse.patch_signals(hal: '...')
18end
```

```html
 1use async_stream::stream;
 2use datastar::prelude::*;
 3use std::thread;
 4use std::time::Duration;
 5
 6Sse(stream! {
 7    // Patches signals.
 8    yield PatchSignals::new("{hal: 'Affirmative, Dave. I read you.'}").into();
 9
10    thread::sleep(Duration::from_secs(1));
11    
12    yield PatchSignals::new("{hal: '...'}").into();
13})
```

```html
1// Creates a new `ServerSentEventGenerator` instance (this also sends required headers)
2ServerSentEventGenerator.stream(req, res, (stream) => {
3    // Patches signals.
4    stream.patchSignals({'hal': 'Affirmative, Dave. I read you.'});
5
6    setTimeout(() => {
7        stream.patchSignals({'hal': '...'});
8    }, 1000);
9});
```

> In addition to your browser's dev tools, the [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to monitor and inspect SSE events received by Datastar.

We'll cover event streams and [SSE events](/reference/sse_events) in more detail [later in the guide](/guide/backend_requests), but as you can see, they are just plain text events with a special syntax, made simpler by the [SDKs](/reference/sdks).