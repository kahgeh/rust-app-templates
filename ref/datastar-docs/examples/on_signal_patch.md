# On Signal Patch

Source: https://data-star.dev/examples/on_signal_patch

---

# On Signal Patch

Demo

Update Message Increment Counter Clear All Changes

### Current Values

Counter:

Message:

### Counter Changes Only

### All Signal Changes

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-signals="{counter: 0, message: 'Hello World', allChanges: [], counterChanges: []}">
   <div class="actions">
    <button data-on-click="$message = `Updated: ${performance.now().toFixed(2)}`">
     Update Message
    </button>
    <button data-on-click="$counter++">
     Increment Counter
    </button>
    <button class="error" data-on-click="$allChanges.length = 0; $counterChanges.length = 0">
     Clear All Changes
    </button>
   </div>
   <div>
    <h3>
     Current Values
    </h3>
    <p>
     Counter:
     <span data-text="$counter">
     </span>
    </p>
    <p>
     Message:
     <span data-text="$message">
     </span>
    </p>
   </div>
   <div data-on-signal-patch="$counterChanges.push(patch)" data-on-signal-patch-filter="{include: /^counter$/}">
    <h3>
     Counter Changes Only
    </h3>
    <pre data-json-signals__terse="{include: /^counterChanges/}"></pre>
   </div>
   <div data-on-signal-patch="$allChanges.push(patch)" data-on-signal-patch-filter="{exclude: /(^|\.)_|allChanges|counterChanges/}">
    <h3>
     All Signal Changes
    </h3>
    <pre data-json-signals__terse="{include: /^allChanges/}"></pre>
   </div>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

```html
 1<div data-signals="{counter: 0, message: 'Hello World', allChanges: [], counterChanges: []}">
 2    <div class="actions">
 3        <button data-on-click="$message = `Updated: ${performance.now().toFixed(2)}`">
 4            Update Message
 5        </button>
 6        <button data-on-click="$counter++">
 7            Increment Counter
 8        </button>
 9        <button
10            class="error"
11            data-on-click="$allChanges.length = 0; $counterChanges.length = 0"
12        >
13            Clear All Changes
14        </button>
15    </div>
16    <div>
17        <h3>Current Values</h3>
18        <p>Counter: <span data-text="$counter"></span></p>
19        <p>Message: <span data-text="$message"></span></p>
20    </div>
21    <div
22        data-on-signal-patch="$counterChanges.push(patch)"
23        data-on-signal-patch-filter="{include: /^counter$/}"
24    >
25        <h3>Counter Changes Only</h3>
26        <pre data-json-signals__terse="{include: /^counterChanges/}"></pre>
27    </div>
28    <div
29        data-on-signal-patch="$allChanges.push(patch)"
30        data-on-signal-patch-filter="{exclude: /allChanges|counterChanges/}"
31    >
32        <h3>All Signal Changes</h3>
33        <pre data-json-signals__terse="{include: /^allChanges/}"></pre>
34    </div>
35</div>
```

The [`data-on-signal-patch`](/reference/attributes#data-on-signal-patch) plugin allows you to execute an expression whenever signals are patched. This is useful for tracking changes, updating dependent values, or triggering side effects.

You can filter which signals to watch using the `data-on-signal-patch-filter` attribute with include/exclude patterns, as seen above.