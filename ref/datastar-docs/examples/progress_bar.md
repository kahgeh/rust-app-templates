# Progress Bar

Source: https://data-star.dev/examples/progress_bar

---

# Progress Bar

Demo

0%

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-on-load="@get('/examples/progress_bar/updates', {openWhenHidden: true})" id="progress-bar">
   <svg height="200" style="transform: rotate(-90deg)" viewbox="-25 -25 250 250" width="200">
    <circle cx="100" cy="100" fill="transparent" r="90" stroke="#e0e0e0" stroke-dasharray="565.48px" stroke-dashoffset="565px" stroke-width="16px">
    </circle>
    <circle cx="100" cy="100" fill="transparent" r="90" stroke="#6bdba7" stroke-dasharray="565.48px" stroke-dashoffset="565px" stroke-linecap="round" stroke-width="16px">
    </circle>
    <text fill="#6bdba7" font-size="52px" font-weight="bold" style="transform:rotate(90deg) translate(0px, -196px)" x="44px" y="115px">
     0%
    </text>
   </svg>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div
 2    id="progress-bar"
 3    data-on-load="@get('/examples/progress_bar/updates', {openWhenHidden: true})"
 4>
 5    <!-- When progress is less than 100% -->
 6    <svg
 7        width="200"
 8        height="200"
 9        viewbox="-25 -25 250 250"
10        style="transform: rotate(-90deg)"
11    >
12        <circle
13            r="90"
14            cx="100"
15            cy="100"
16            fill="transparent"
17            stroke="#e0e0e0"
18            stroke-width="16px"
19            stroke-dasharray="565.48px"
20            stroke-dashoffset="565px"
21        ></circle>
22        <circle
23            r="90"
24            cx="100"
25            cy="100"
26            fill="transparent"
27            stroke="#6bdba7"
28            stroke-width="16px"
29            stroke-linecap="round"
30            stroke-dashoffset="282px"
31            stroke-dasharray="565.48px"
32        ></circle>
33        <text
34            x="44px"
35            y="115px"
36            fill="#6bdba7"
37            font-size="52px"
38            font-weight="bold"
39            style="transform:rotate(90deg) translate(0px, -196px)"
40        >50%</text>
41    </svg>
42    
43    <!-- When progress is 100% -->
44    <button
45        data-indicator-_fetching
46        data-attr-aria-disabled="`${$_fetching}`"
47        data-on-click="
48            !$_fetching && @get('/examples/progress_bar/updates', {openWhenHidden: true})
49        "
50    >
51        <i class="material-symbols:check-circle"></i>
52        Completed! Try again?
53    </button>
54</div>
```

## Explanation [#](#explanation)

This example shows an updating progress graphic. Since Datastar supports SSE, this is very easy to implement. The server sends down a new progress bar svg every 500 milliseconds causing the client to update. After the progress is complete, the server sends down a button allowing the user to restart the progress bar.

### Note [#](#note)

The `openWhenHidden` option is used to keep the connection open even when the progress bar is not visible. This is useful for when the user navigates away from the page and then returns. This will use more resources, so use it judiciously.