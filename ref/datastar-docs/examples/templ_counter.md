# Templ Counter

Source: https://data-star.dev/examples/templ_counter

---

# Templ Counter

Demo

Increment Global: 10Increment User: 0

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-on-load="@get('/examples/templ_counter/updates')" style="display: flex; gap: var(--size-6)">
   <button class="info" data-on-click="@patch('/examples/templ_counter/global')" id="global">
    Increment Global: 10
   </button>
   <button class="success" data-on-click="@patch('/examples/templ_counter/user')" id="user">
    Increment User: 0
   </button>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div
 2    style="display: flex; gap: var(--size-6)"
 3    data-on-load="@get('/examples/templ_counter/updates')"
 4>
 5    <!-- Global Counter -->
 6    <button
 7        id="global"
 8        class="info"
 9        data-on-click="@patch('/examples/templ_counter/global')"
10    >
11        Global Clicks: 0
12    </button>
13
14    <!-- User Counter -->
15    <button
16        id="user"
17        class="success"
18        data-on-click="@patch('/examples/templ_counter/user')"
19    >
20        User Clicks: 0
21    </button>
22</div>
```

## Explanation [#](#explanation)

This example demonstrates two counters - a global counter shared across all users and a user-specific counter. The counters are updated via server-sent events (SSE) and increment when clicked.