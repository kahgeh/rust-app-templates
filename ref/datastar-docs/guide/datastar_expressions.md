# Datastar Expressions

Source: https://data-star.dev/guide/datastar_expressions

---

# Datastar Expressions

Datastar expressions are strings that are evaluated by `data-*` attributes. While they are similar to JavaScript, there are some important differences that make them more powerful for declarative hypermedia applications.

## Datastar Expressions [#](#datastar-expressions)

The following example outputs `1` because we've defined `foo` as a signal with the initial value `1`, and are using `$foo` in a `data-*` attribute.

```html
1<div data-signals-foo="1">
2    <div data-text="$foo"></div>
3</div>
```

A variable `el` is available in every Datastar expression, representing the element that the attribute is attached to.

```html
1<div id="foo" data-text="el.id"></div>
```

When Datastar evaluates the expression `$foo`, it first converts it to the signal value, and then evaluates that expression in a sandboxed context. This means that JavaScript can be used in Datastar expressions.

```html
1<div data-text="$foo.length"></div>
```

JavaScript operators are also available in Datastar expressions. This includes (but is not limited to) the ternary operator `?:`, the logical OR operator `||`, and the logical AND operator `&&`. These operators are helpful in keeping Datastar expressions terse.

```html
 1// Output one of two values, depending on the truthiness of a signal
 2<div data-text="$landingGearRetracted ? 'Ready' : 'Waiting'"></div>
 3
 4// Show a countdown if the signal is truthy or the time remaining is less than 10 seconds
 5<div data-show="$landingGearRetracted || $timeRemaining < 10">
 6    Countdown
 7</div>
 8
 9// Only send a request if the signal is truthy
10<button data-on-click="$landingGearRetracted && @post('/launch')">
11    Launch
12</button>
```

Multiple statements can be used in a single expression by separating them with a semicolon.

```html
1<div data-signals-foo="1">
2    <button data-on-click="$landingGearRetracted = true; @post('/launch')">
3        Force launch
4    </button>
5</div>
```

Expressions may span multiple lines, but a semicolon must be used to separate statements. Unlike JavaScript, line breaks alone are not sufficient to separate statements.

```html
1<div data-signals-foo="1">
2    <button data-on-click="
3        $landingGearRetracted = true; 
4        @post('/launch')
5    ">
6        Force launch
7    </button>
8</div>
```

## Using JavaScript [#](#using-javascript)

Most of your JavaScript logic should go in `data-*` attributes, since reactive signals and actions only work in [Datastar expressions](/guide/datastar_expressions).

> Caution: if you find yourself trying to do too much in Datastar expressions, **you are probably overcomplicating itâ¢**.

Any additional JavaScript functionality you require that cannot belong in `data-*` attributes should be extracted out into [external scripts](#external-scripts) or, better yet, [web components](#web-components).

> Always encapsulate state and send **props down, events up**.

### External Scripts [#](#external-scripts)

When using external scripts, pass data into functions via arguments and return a result *or* listen for custom events dispatched from them **props down, events up**.

In this way, the function is encapsulated — all it knows is that it receives input via an argument, acts on it, and optionally returns a result or dispatches a custom event — and `data-*` attributes can be used to drive reactivity.

```html
1<div data-signals-result>
2    <input data-bind-foo 
3        data-on-input="$result = myfunction($foo)"
4    >
5    <span data-text="$result"></span>
6</div>
```

```html
1function myfunction(data) {
2    return `You entered: ${data}`;
3}
```

If your function call is asynchronous then it will need to dispatch a custom event containing the result. While asynchronous code *can* be placed within Datastar expressions, Datastar will *not* await it.

```html
1<div data-signals-result>
2    <input data-bind-foo 
3           data-on-input="myfunction(el, $foo)"
4           data-on-mycustomevent__window="$result = evt.detail.value"
5    >
6    <span data-text="$result"></span>
7</div>
```

```html
1async function myfunction(element, data) {
2    const value = await new Promise((resolve) => {
3        setTimeout(() => resolve(`You entered: ${data}`), 1000);
4    });
5    element.dispatchEvent(
6        new CustomEvent('mycustomevent', {detail: {value}})
7    );
8}
```

See the [sortable example](/examples/sortable).

### Web Components [#](#web-components)

[Web components](https://developer.mozilla.org/en-US/docs/Web/API/Web_components) allow you create reusable, encapsulated, custom elements. They are native to the web and require no external libraries or frameworks. Web components unlock [custom elements](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements) — HTML tags with custom behavior and styling.

When using web components, pass data into them via attributes and listen for custom events dispatched from them (*props down, events up*).

In this way, the web component is encapsulated — all it knows is that it receives input via an attribute, acts on it, and optionally dispatches a custom event containing the result — and `data-*` attributes can be used to drive reactivity.

```html
1<div data-signals-result="''">
2    <input data-bind-foo />
3    <my-component
4        data-attr-src="$foo"
5        data-on-mycustomevent="$result = evt.detail.value"
6    ></my-component>
7    <span data-text="$result"></span>
8</div>
```

```html
 1class MyComponent extends HTMLElement {
 2    static get observedAttributes() {
 3        return ['src'];
 4    }
 5
 6    attributeChangedCallback(name, oldValue, newValue) {
 7        const value = `You entered: ${newValue}`;
 8        this.dispatchEvent(
 9            new CustomEvent('mycustomevent', {detail: {value}})
10        );
11    }
12}
13
14customElements.define('my-component', MyComponent);
```

Since the `value` attribute is allowed on web components, it is also possible to use `data-bind` to bind a signal to the web component's value. Note that a `change` event must be dispatched so that the event listener used by `data-bind` is triggered by the value change.

See the [web component example](/examples/web_component).

## Executing Scripts [#](#executing-scripts)

Just like elements and signals, the backend can also send JavaScript to be executed on the frontend using [backend actions](/reference/actions#backend-actions).

```html
1<button data-on-click="@get('/endpoint')">
2    What are you talking about, HAL?
3</button>
```

If a response has a `content-type` of `text/javascript`, the value will be executed as JavaScript in the browser.

```html
1alert('This mission is too important for me to allow you to jeopardize it.')
```

Demo

What are you talking about, HAL?

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="@get('/guide/datastar_expressions/what_are_you_talking_about_javascript')">
   What are you talking about, HAL?
  </button>
 </div>
</fieldset>
```

If the response has a `content-type` of `text/event-stream`, it can contain zero or more [SSE events](/reference/sse_events). The example above can be replicated by including a `script` tag inside of a `datastar-patch-elements` SSE event.

```html
1event: datastar-patch-elements
2data: elements <div id="hal">
3data: elements     <script>alert('This mission is too important for me to allow you to jeopardize it.')</script>
4data: elements </div>
```

If you *only* want to execute a script, you can `append` the script tag to the `body`.

```html
1event: datastar-patch-elements
2data: mode append
3data: selector body
4data: elements <script>alert('This mission is too important for me to allow you to jeopardize it.')</script>
```

Most SDKs have an `ExecuteScript` helper function for executing a script. Here's the code to generate the SSE event above using the Go SDK.

```html
1sse := datastar.NewSSE(writer, request)
2sse.ExecuteScript(`alert('This mission is too important for me to allow you to jeopardize it.')`)
```

Demo

What are you talking about, HAL?

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button class="warning large" data-on-click="@get('/guide/datastar_expressions/what_are_you_talking_about_sse')">
   What are you talking about, HAL?
  </button>
 </div>
</fieldset>
```

We'll cover event streams and [SSE events](/reference/sse_events) in more detail [later in the guide](/guide/backend_requests), but as you can see, they are just plain text events with a special syntax, made simpler by the [SDKs](/reference/sdks).