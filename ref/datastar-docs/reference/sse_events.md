# SSE Events

Source: https://data-star.dev/reference/sse_events

---

# SSE Events

Responses to [backend actions](/reference/actions#backend-actions) with a content type of `text/event-stream` can contain zero or more Datastar [SSE](https://en.wikipedia.org/wiki/Server-sent_events) events.

> The backend [SDKs](/reference/sdks) can handle the formatting of SSE events for you, or you can format them yourself.

## Event Types [#](#event-types)

### `datastar-patch-elements` [#](#datastar-patch-elements)

Patches one or more elements in the DOM. By default, Datastar morphs elements by matching top-level elements based on their ID.

```html
1event: datastar-patch-elements
2data: elements <div id="foo">Hello world!</div>
```

In the example above, the element `<div id="foo">Hello world!</div>` will be morphed into the target element with ID `foo`.

> Be sure to place IDs on top-level elements to be morphed, as well as on elements within them that you'd like to preserve state on (event listeners, CSS transitions, etc.).

SVG morphing in Datastar requires special handling due to XML namespaces. See the [SVG morphing example](/examples/svg_morphing).

Additional `data` lines can be added to the response to override the default behavior.

| Key | Description |
| --- | --- |
| `data: mode outer` | Morphs the outer HTML of the elements. This is the default (and recommended) mode. |
| `data: mode inner` | Morphs the inner HTML of the elements. |
| `data: mode replace` | Replaces the outer HTML of the elements. |
| `data: mode prepend` | Prepends the elements to the target's children. |
| `data: mode append` | Appends the elements to the target's children. |
| `data: mode before` | Inserts the elements before the target as siblings. |
| `data: mode after` | Inserts the elements after the target as siblings. |
| `data: mode remove` | Removes the target elements from DOM. |
| `data: selector #foo` | Selects the target element of the patch using a CSS selector. Not required when using the `outer` or `inner` modes. |
| `data: useViewTransition true` | Whether to use view transitions when patching elements. Defaults to `false`. |
| `data: elements` | The HTML elements to patch. |

```html
1event: datastar-patch-elements
2data: elements <div id="foo">Hello world!</div>
```

Elements can be removed using the `remove` mode and providing either an explicit `selector` or elements with an ID.

```html
1event: datastar-patch-elements
2data: mode remove
3data: selector #foo
```

```html
1event: datastar-patch-elements
2data: mode remove
3data: elements <div id="foo"></div>
```

Elements can span multiple lines. Sample output showing non-default options:

```html
1event: datastar-patch-elements
2data: mode inner
3data: selector #foo
4data: useViewTransition true
5data: elements <div>
6data: elements        Hello world!
7data: elements </div>
```

### `datastar-patch-signals` [#](#datastar-patch-signals)

Patches signals into the existing signals on the page. The `onlyIfMissing` line determines whether to update each signal with the new value only if a signal with that name does not yet exist. The `signals` line should be a valid `data-signals` attribute.

```html
1event: datastar-patch-signals
2data: signals {foo: 1, bar: 2}
```

Signals can be removed by setting their values to `null`.

```html
1event: datastar-patch-signals
2data: signals {foo: null, bar: null}
```

Sample output showing non-default options:

```html
1event: datastar-patch-signals
2data: onlyIfMissing true
3data: signals {foo: 1, bar: 2}
```