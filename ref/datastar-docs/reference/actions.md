# Actions

Source: https://data-star.dev/reference/actions

---

# Actions

Datastar provides actions that can be used in Datastar expressions.

> The `@` prefix designates actions that are safe to use in expressions. This is a security feature that prevents arbitrary JavaScript from being executed in the browser. Datastar uses [`Function()` constructors](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function) to create and execute these actions in a secure and controlled sandboxed environment.

### `@peek()` [#](#peek)

> `@peek(callable: () => any)`

Allows accessing signals without subscribing to their changes in expressions.

```html
1<div data-text="$foo + @peek(() => $bar)"></div>
```

In the example above, the expression in the `data-text` attribute will be re-evaluated whenever `$foo` changes, but it will *not* be re-evaluated when `$bar` changes, since it is evaluated inside the `@peek()` action.

### `@setAll()` [#](#setall)

> `@setAll(value: any, filter?: {include: RegExp, exclude?: RegExp})`

Sets the value of all matching signals (or all signals if no filter is used) to the expression provided in the first argument. The second argument is an optional filter object with an `include` property that accepts a regular expression to match signal paths. You can optionally provide an `exclude` property to exclude specific patterns.

> The [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to inspect and filter current signals and view signal patch events in real-time.

```html
 1<!-- Sets the `foo` signal only -->
 2<div data-signals-foo="false">
 3    <button data-on-click="@setAll(true, {include: /^foo$/})"></button>
 4</div>
 5
 6<!-- Sets all signals starting with `user.` -->
 7<div data-signals="{user: {name: '', nickname: ''}}">
 8    <button data-on-click="@setAll('johnny', {include: /^user\./})"></button>
 9</div>
10
11<!-- Sets all signals except those ending with `_temp` -->
12<div data-signals="{data: '', data_temp: '', info: '', info_temp: ''}">
13    <button data-on-click="@setAll('reset', {include: /.*/, exclude: /_temp$/})"></button>
14</div>
```

### `@toggleAll()` [#](#toggleall)

> `@toggleAll(filter?: {include: RegExp, exclude?: RegExp})`

Toggles the boolean value of all matching signals (or all signals if no filter is used). The argument is an optional filter object with an `include` property that accepts a regular expression to match signal paths. You can optionally provide an `exclude` property to exclude specific patterns.

> The [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to inspect and filter current signals and view signal patch events in real-time.

```html
 1<!-- Toggles the `foo` signal only -->
 2<div data-signals-foo="false">
 3    <button data-on-click="@toggleAll({include: /^foo$/})"></button>
 4</div>
 5
 6<!-- Toggles all signals starting with `is` -->
 7<div data-signals="{isOpen: false, isActive: true, isEnabled: false}">
 8    <button data-on-click="@toggleAll({include: /^is/})"></button>
 9</div>
10
11<!-- Toggles signals starting with `settings.` -->
12<div data-signals="{settings: {darkMode: false, autoSave: true}}">
13    <button data-on-click="@toggleAll({include: /^settings\./})"></button>
14</div>
```

## Backend Actions [#](#backend-actions)

### `@get()` [#](#get)

> `@get(uri: string, options={ })`

Sends a `GET` request to the backend using the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API). The URI can be any valid endpoint and the response must contain zero or more [Datastar SSE events](/reference/sse_events).

```html
1<button data-on-click="@get('/endpoint')"></button>
```

By default, requests are sent with a `Datastar-Request: true` header, and a `{datastar: *}` object containing all existing signals, except those beginning with an underscore. This behavior can be changed using the [`filterSignals`](#filterSignals) option, which allows you to include or exclude specific signals using regular expressions.

> When using a `get` request, the signals are sent as a query parameter, otherwise they are sent as a JSON body.

When a page is hidden (in a background tab, for example), the default behavior is for the SSE connection to be closed, and reopened when the page becomes visible again. To keep the connection open when the page is hidden, set the [`openWhenHidden`](#openWhenHidden) option to `true`.

```html
1<button data-on-click="@get('/endpoint', {openWhenHidden: true})"></button>
```

It's possible to send form encoded requests by setting the `contentType` option to `form`. This sends requests using `application/x-www-form-urlencoded` encoding.

```html
1<button data-on-click="@get('/endpoint', {contentType: 'form'})"></button>
```

It's also possible to send requests using `multipart/form-data` encoding by specifying it in the `form` element's [`enctype`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype) attribute. This should be used when uploading files. See the [form data example](/examples/form_data).

```html
1<form enctype="multipart/form-data">
2    <input type="file" name="file" />
3    <button data-on-click="@get('/endpoint', {contentType: 'form'})"></button>
4</form>
```

### `@post()` [#](#post)

> `@post(uri: string, options={ })`

Works the same as [`@get()`](#get) but sends a `POST` request to the backend.

```html
1<button data-on-click="@post('/endpoint')"></button>
```

### `@put()` [#](#put)

> `@put(uri: string, options={ })`

Works the same as [`@get()`](#get) but sends a `PUT` request to the backend.

```html
1<button data-on-click="@put('/endpoint')"></button>
```

### `@patch()` [#](#patch)

> `@patch(uri: string, options={ })`

Works the same as [`@get()`](#get) but sends a `PATCH` request to the backend.

```html
1<button data-on-click="@patch('/endpoint')"></button>
```

### `@delete()` [#](#delete)

> `@delete(uri: string, options={ })`

Works the same as [`@get()`](#get) but sends a `DELETE` request to the backend.

```html
1<button data-on-click="@delete('/endpoint')"></button>
```

### Options [#](#options)

All of the actions above take a second argument of options.

- `contentType` — The type of content to send. A value of `json` sends all signals in a JSON request. A value of `form` tells the action to look for the closest form to the element on which it is placed (unless a `selector` option is provided), perform validation on the form elements, and send them to the backend using a form request (no signals are sent). Defaults to `json`.
- `filterSignals` — A filter object with an `include` property that accepts a regular expression to match signal paths (defaults to all signals: `/.*/`), and an optional `exclude` property to exclude specific signal paths (defaults to all signals that do not have a `_` prefix: `/(^_|\._).*/`).
  > The [Datastar Inspector](/reference/datastar_pro#datastar-inspector) can be used to inspect and filter current signals and view signal patch events in real-time.
- `selector` — Optionally specifies a form to send when the `contentType` option is set to `form`. If the value is `null`, the closest form is used. Defaults to `null`.
- `headers` — An object containing headers to send with the request.
- `openWhenHidden` — Whether to keep the connection open when the page is hidden. Useful for dashboards but can cause a drain on battery life and other resources when enabled. Defaults to `false`.
- `retryInterval` — The retry interval in milliseconds. Defaults to `1000` (one second).
- `retryScaler` — A numeric multiplier applied to scale retry wait times. Defaults to `2`.
- `retryMaxWaitMs` — The maximum allowable wait time in milliseconds between retries. Defaults to `30000` (30 seconds).
- `retryMaxCount` — The maximum number of retry attempts. Defaults to `10`.
- `requestCancellation` — Controls request cancellation behavior. Can be `'auto'` (default, cancels existing requests on the same element), `'disabled'` (allows concurrent requests), or an `AbortController` instance for custom control. Defaults to `'auto'`.

```html
1<button data-on-click="@get('/endpoint', {
2    filterSignals: {include: /^foo\./},
3    headers: {
4        'X-Csrf-Token': 'JImikTbsoCYQ9oGOcvugov0Awc5LbqFsZW6ObRCxuq',
5    },
6    openWhenHidden: true,
7    requestCancellation: 'disabled',
8})"></button>
```

### Request Cancellation [#](#request-cancellation)

By default, when a new fetch request is initiated on an element, any existing request on that same element is automatically cancelled. This prevents multiple concurrent requests from conflicting with each other and ensures clean state management.

For example, if a user rapidly clicks a button that triggers a backend action, only the most recent request will be processed:

```html
1<!-- Clicking this button multiple times will cancel previous requests (default behavior) -->
2<button data-on-click="@get('/slow-endpoint')">Load Data</button>
```

This automatic cancellation happens at the element level, meaning requests on different elements can run concurrently without interfering with each other.

You can control this behavior using the [`requestCancellation`](#requestCancellation) option:

```html
1<!-- Allow concurrent requests (no automatic cancellation) -->
2<button data-on-click="@get('/endpoint', {requestCancellation: 'disabled'})">Allow Multiple</button>
3
4<!-- Custom abort controller for fine-grained control -->
5<div data-signals-controller="new AbortController()">
6    <button data-on-click="@get('/endpoint', {requestCancellation: $controller})">Start Request</button>
7    <button data-on-click="$controller.abort()">Cancel Request</button>
8</div>
```

### Response Handling [#](#response-handling)

Backend actions automatically handle different response content types:

- `text/event-stream` — Standard SSE responses with [Datastar SSE events](/reference/sse_events).
- `text/html` — HTML elements to patch into the DOM.
- `application/json` — JSON encoded signals to patch.
- `text/javascript` — JavaScript code to execute in the browser.

#### `text/html`

When returning HTML (`text/html`), the server can optionally include the following response headers:

- `datastar-selector` — A CSS selector for the target elements to patch
- `datastar-mode` — How to patch the elements (`outer`, `inner`, `remove`, `replace`, `prepend`, `append`, `before`, `after`). Defaults to `outer`.
- `datastar-use-view-transition` — Whether to use the [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transitions_API) when patching elements.

```html
1response.headers.set('Content-Type', 'text/html')
2response.headers.set('datastar-selector', '#my-element')
3response.headers.set('datastar-mode', 'inner')
4response.body = '<p>New content</p>'
```

#### `application/json`

When returning JSON (`application/json`), the server can optionally include the following response header:

- `datastar-only-if-missing` — If set to `true`, only patch signals that don't already exist.

```html
1response.headers.set('Content-Type', 'application/json')
2response.headers.set('datastar-only-if-missing', 'true')
3response.body = JSON.stringify({ foo: 'bar' })
```

#### `text/javascript`

When returning JavaScript (`text/javascript`), the server can optionally include the following response header:

- `datastar-script-attributes` — Sets the script element's attributes using a JSON encoded string.

```html
1response.headers.set('Content-Type', 'text/javascript')
2response.headers.set('datastar-script-attributes', JSON.stringify({ type: 'module' }))
3response.body = 'console.log("Hello from server!");'
```

### Events [#](#events)

All of the actions above trigger `datastar-fetch` events during the fetch request lifecycle. The event type determines the stage of the request.

- `started` — Triggered when the fetch request is started.
- `finished` — Triggered when the fetch request is finished.
- `error` — Triggered when the fetch request encounters an error.
- `retrying` — Triggered when the fetch request is retrying.
- `retries-failed` — Triggered when all fetch retries have failed.
- [`upload-progress`](#upload-progress) — Triggered during file uploads (see below).

```html
1<div data-on-datastar-fetch="
2    evt.detail.type === 'error' && console.log('Fetch error encountered')
3"></div>
```

### Upload Progress [#](#upload-progress)[Pro](/reference/datastar_pro)

All [backend actions](/reference/actions#backend-actions) (`@get()`, `@post()`, `@put()`, `@patch()`, `@delete()`) automatically support file upload progress monitoring when:

- Using [Datastar Pro](/reference/datastar_pro) (not available in the free version).
- The target URL uses the HTTPS protocol.
- The request body is FormData (multipart/form-data).

> The HTTPS requirement exists due to browser security restrictions. Browsers only allow the `duplex` option (required for ReadableStream uploads) on secure connections. For HTTP URLs or non-FormData requests, standard fetch is used without progress tracking.

When these conditions are met, the actions dispatch `upload-progress` fetch events with:

- `progress` — Upload percentage (0-100) as a string
- `loaded` — Bytes uploaded so far as a string
- `total` — Total bytes to upload as a string

```html
 1<form enctype="multipart/form-data"
 2    data-signals="{progress: 0, uploading: false}"
 3    data-on-submit__prevent="@post('https://example.com/upload', {contentType: 'form'})"
 4    data-on-datastar-fetch="
 5        if (evt.detail.type !== 'upload-progress') return;
 6
 7        const {progress, loaded, total} = evt.detail.argsRaw;
 8        $uploading = true;
 9        $progress = Number(progress);
10
11        if ($progress >= 100) {
12            $uploading = false;
13        }
14    "
15>
16    <input type="file" name="files" multiple />
17    <button type="submit">Upload</button>
18    <progress data-show="$uploading" data-attr-value="$progress" max="100"></progress>
19</form>
```

## Pro Actions [#](#pro-actions)

### `@clipboard()` [#](#clipboard)[Pro](/reference/datastar_pro)

> `@clipboard(text: string, isBase64?: boolean)`

Copies the provided text to the clipboard. If the second parameter is `true`, the text is treated as [Base64](https://developer.mozilla.org/en-US/docs/Glossary/Base64) encoded, and is decoded before copying.

> Base64 encoding is useful when copying content that contains special characters, quotes, or code fragments that might not be valid within HTML attributes. This prevents parsing errors and ensures the content is safely embedded in `data-*` attributes.

```html
1<!-- Copy plain text -->
2<button data-on-click="@clipboard('Hello, world!')"></button>
3
4<!-- Copy base64 encoded text (will decode before copying) -->
5<button data-on-click="@clipboard('SGVsbG8sIHdvcmxkIQ==', true)"></button>
```

### `@fit()` [#](#fit)[Pro](/reference/datastar_pro)

> `@fit(v: number, oldMin: number, oldMax: number, newMin: number, newMax: number, shouldClamp=false, shouldRound=false)`

Linearly interpolates a value from one range to another. This is useful for converting between different scales, such as mapping a slider value to a percentage or converting temperature units.

The optional `shouldClamp` parameter ensures the result stays within the new range, and `shouldRound` rounds the result to the nearest integer.

```html
 1<!-- Convert a 0-100 slider to 0-255 RGB value -->
 2<div>
 3    <input type="range" min="0" max="100" value="50" data-bind-slider-value>
 4    <div data-computed-rgb-value="@fit($sliderValue, 0, 100, 0, 255)">
 5        RGB Value: <span data-text="$rgbValue"></span>
 6    </div>
 7</div>
 8
 9<!-- Convert Celsius to Fahrenheit -->
10<div>
11    <input type="number" data-bind-celsius value="20" />
12    <div data-computed-fahrenheit="@fit($celsius, 0, 100, 32, 212)">
13        <span data-text="$celsius"></span>Â°C = <span data-text="$fahrenheit.toFixed(1)"></span>Â°F
14    </div>
15</div>
16
17<!-- Map mouse position to element opacity (clamped) -->
18<div
19    data-signals-mouse-x="0"
20    data-computed-opacity="@fit($mouseX, 0, window.innerWidth, 0, 1, true)"
21    data-on-mousemove__window="$mouseX = evt.clientX"
22    data-attr-style="'opacity: ' + $opacity"
23>
24    Move your mouse horizontally to change opacity
25</div>
```