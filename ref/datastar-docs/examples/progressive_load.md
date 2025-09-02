# Progressive Load

Source: https://data-star.dev/examples/progressive_load

---

# Progressive Load

Demonstrates how to progressively load different sections of a page using SSE events.

Demo

Load

![Indicator](/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif)

Each part is loaded randomly and progressively.

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div class="actions">
   <button data-attr-disabled="$loadDisabled" data-indicator-progressive-load="" data-on-click="$loadDisabled=true; @get('/examples/progressive_load/updates')" data-signals-load-disabled="false" id="load-button">
    Load
   </button>
   <div class="indicator" data-class-loading="$progressiveLoad">
    <img alt="Indicator" height="32" loading="" src="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif" srcset="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 1x, /cdn-cgi/image/format=auto,width=64/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 2x" width="32"/>
   </div>
  </div>
  <p>
   Each part is loaded randomly and progressively.
  </p>
  <div id="Load">
   <section id="article">
   </section>
   <section id="comments">
   </section>
   <div id="footer">
   </div>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div>
 2    <div class="actions">
 3        <button
 4            id="load-button"
 5            data-signals-load-disabled="false"
 6            data-on-click="$loadDisabled=true; @get('/examples/progressive_load/updates')"
 7            data-attr-disabled="$loadDisabled"
 8            data-indicator-progressive-Load
 9        >
10            Load
11        </button>
12        <!-- Indicator element -->
13    </div>
14    <p>
15        Each part is loaded randomly and progressively.
16    </p>
17</div>
18<div id="Load">
19    <header id="header">Welcome to my blog</header>
20    <section id="article">
21        <h4>This is my article</h4>
22        <section id="articleBody">
23            <p>
24                Lorem ipsum dolor sit amet...
25            </p>
26        </section>
27    </section>
28    <section id="comments">
29        <h5>Comments</h5>
30        <p>
31            This is the comments section. It will also be progressively loaded as you scroll down.
32        </p>
33        <ul id="comments-list">
34            <li id="1">
35                <img src="https://avatar.iran.liara.run/username?username=example" alt="Avatar" class="avatar"/>
36                This is a comment...
37            </li>
38            <!-- More comments loaded progressively -->
39        </ul>
40    </section>
41    <div id="footer">Hope you like it</div>
42</div>
```

## Explanation [#](#explanation)

This is a response to [Dan Abramov's article on progressive JSON](https://overreacted.io/progressive-json/). I think it's overcomplicated and shows a lack of understanding of how powerful native hypermedia is.

### Note [#](#note)

This example shows how to progressively load a page using Datastar. The page is divided into sections. We already have examples of [infinite scroll](/examples/infinite_scroll) and [progress bar](/examples/progress_bar), but this example shows how to progressively load a page in a more structured way.

It's truly baffling to me the amount of complexity that React developers tend to introduce. Hypermedia is a powerful tool that allows you to progressively load content in a way that is simple and efficient. This example shows how to use Datastar's server-sent events (SSE) to progressively load a page in a way that is easy to understand and maintain.

Nothing is faster than direct HTML morphing without a virtual DOM. — let the browser do the heavy lifting. This example shows how to use Datastar to progressively load a page in a way that is simple and efficient while only using a one-time cost CDN shim.