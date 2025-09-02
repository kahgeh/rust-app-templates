# Lazy Load

Source: https://data-star.dev/examples/lazy_load

---

# Lazy Load

Demo

Loading...![Indicator](/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif)

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-on-load="@get('/examples/lazy_load/graph')" id="lazy-load">
   Loading...
   <img alt="Indicator" height="32" loading="" src="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif" srcset="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 1x, /cdn-cgi/image/format=auto,width=64/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 2x" width="32"/>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This example shows how to lazily load an element on a page. We start with an initial state that looks like this:

```html
1<div id="graph" data-on-load="@get('/examples/lazy_load/graph')">
2    Loading...
3</div>
```

Which shows a progress indicator as we are loading the graph. The graph is loaded by patching an element with the same ID.

```html
1<div id="graph">
2    <img src="/images/examples/tokyo.png" />
3</div>
```