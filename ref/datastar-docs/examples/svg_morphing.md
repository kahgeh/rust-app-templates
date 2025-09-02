# SVG Morphing

Source: https://data-star.dev/examples/svg_morphing

---

# SVG Morphing

SVG morphing in Datastar requires special handling because, as an XML dialect, SVG is [namespaced](https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/Namespaces_crash_course). This means that `<svg>` elements (as well as `<math>` elements) create their own namespace, separate from the HTML namespace.

To morph an SVG element, you must ensure that the target element is wrapped in an outer `<svg>` tag. This ensures that the inner SVG element is created under the correct namespace.

```html
1<svg>
2    <svg id="target">
3        <circle cx="50" cy="100" r="50" fill="red" />
4    </svg>
5    <circle cx="150" cy="100" r="50" fill="red" />
6</svg>
```

## Basic Circle Color Change [#](#basic-circle-color-change)

This example demonstrates morphing an SVG circle's color. Click the button to change the circle from red to blue.

Demo

Change Color

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <svg id="circle-demo">
   <circle cx="50" cy="50" fill="red" r="40">
   </circle>
  </svg>
  <div style="margin-top: 10px;">
   <button data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/svg_morphing/circle_color')">
    Change Color
   </button>
  </div>
 </div>
</fieldset>
```

```html
1svgMorphingRouter.Get("/circle_color", func(w http.ResponseWriter, r *http.Request) {
2    sse := datastar.NewSSE(w, r)
3    color := svgColors[rand.N(len(svgColors))]
4    sse.PatchElements(fmt.Sprintf(`<svg id="circle-demo"><circle cx="50" cy="50" r="40" fill="%s" /></svg>`, color))
5})
```

## Circle Radius Change [#](#circle-radius-change)

This example shows how to morph the size of an SVG element. The circle will change to a random radius when you click the button.

Demo

Change Radius

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <svg height="120" style="border: 1px solid #ccc;" width="120">
   <svg id="size-demo">
    <circle cx="50" cy="50" fill="green" r="30">
    </circle>
   </svg>
  </svg>
  <div style="margin-top: 10px;">
   <button data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/svg_morphing/circle_size')">
    Change Radius
   </button>
  </div>
 </div>
</fieldset>
```

```html
1svgMorphingRouter.Get("/circle_size", func(w http.ResponseWriter, r *http.Request) {
2    sse := datastar.NewSSE(w, r)
3    radius := 15 + rand.N(45) // Random radius between 15-60
4    sse.PatchElements(fmt.Sprintf(`<svg id="size-demo"><circle cx="50" cy="50" r="%d" fill="green" /></svg>`, radius))
5})
```

## Random Shape Transformation [#](#random-shape-transformation)

SVG morphing can handle changing between different shape types. This example morphs to a random shape each time you click.

Demo

Random Shape

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <svg height="120" style="border: 1px solid #ccc;" width="120">
   <svg id="shape-demo">
    <circle cx="50" cy="50" fill="purple" r="40">
    </circle>
   </svg>
  </svg>
  <div style="margin-top: 10px;">
   <button data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/svg_morphing/shape_transform')">
    Random Shape
   </button>
  </div>
 </div>
</fieldset>
```

```html
1svgMorphingRouter.Get("/shape_transform", func(w http.ResponseWriter, r *http.Request) {
2    sse := datastar.NewSSE(w, r)
3    shape := svgShapes[rand.N(len(svgShapes))]
4    sse.PatchElements(fmt.Sprintf(`<svg id="shape-demo">%s</svg>`, shape))
5})
```

## Multiple Random Elements [#](#multiple-random-elements)

You can morph multiple SVG elements at once. This example updates three circles with random colors and sizes each time you click.

Demo

Randomize All Circles

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <svg height="120" style="border: 1px solid #ccc;" width="120">
   <svg id="multi-demo">
    <circle cx="30" cy="30" fill="red" r="15">
    </circle>
    <circle cx="70" cy="30" fill="blue" r="15">
    </circle>
    <circle cx="50" cy="70" fill="green" r="15">
    </circle>
   </svg>
  </svg>
  <div style="margin-top: 10px;">
   <button data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/svg_morphing/multiple_elements')">
    Randomize All Circles
   </button>
  </div>
 </div>
</fieldset>
```

```html
 1svgMorphingRouter.Get("/multiple_elements", func(w http.ResponseWriter, r *http.Request) {
 2    sse := datastar.NewSSE(w, r)
 3    color1 := svgColors[rand.N(len(svgColors))]
 4    color2 := svgColors[rand.N(len(svgColors))]
 5    color3 := svgColors[rand.N(len(svgColors))]
 6    r1 := 10 + rand.N(20) // radius 10-30
 7    r2 := 10 + rand.N(20)
 8    r3 := 10 + rand.N(20)
 9    sse.PatchElements(fmt.Sprintf(`<svg id="multi-demo">
10        <circle cx="30" cy="30" r="%d" fill="%s" />
11        <circle cx="70" cy="30" r="%d" fill="%s" />
12        <circle cx="50" cy="70" r="%d" fill="%s" />
13    </svg>`, r1, color1, r2, color2, r3, color3))
14})
```

## Animated Sequence [#](#animated-sequence)

This example demonstrates a sequence of SVG morphs that happen automatically when triggered, creating a smooth animation effect.

Demo

Start Animation Sequence

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <svg height="120" style="border: 1px solid #ccc;" width="120">
   <svg id="animated-demo">
    <circle cx="50" cy="50" fill="green" r="20">
    </circle>
   </svg>
  </svg>
  <div style="margin-top: 10px;">
   <button data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/svg_morphing/animated_morph')">
    Start Animation Sequence
   </button>
  </div>
 </div>
</fieldset>
```

```html
 1svgMorphingRouter.Get("/animated_morph", func(w http.ResponseWriter, r *http.Request) {
 2    sse := datastar.NewSSE(w, r)
 3    
 4    // First morph
 5    sse.PatchElements(`<svg id="animated-demo"><circle cx="50" cy="50" r="30" fill="red" /></svg>`)
 6    time.Sleep(500 * time.Millisecond)
 7    
 8    // Second morph
 9    sse.PatchElements(`<svg id="animated-demo"><circle cx="50" cy="50" r="45" fill="orange" /></svg>`)
10    time.Sleep(500 * time.Millisecond)
11    
12    // Third morph
13    sse.PatchElements(`<svg id="animated-demo"><circle cx="50" cy="50" r="60" fill="yellow" /></svg>`)
14    time.Sleep(500 * time.Millisecond)
15    
16    // Reset
17    sse.PatchElements(`<svg id="animated-demo"><circle cx="50" cy="50" r="20" fill="green" /></svg>`)
18})
```

## Key Points [#](#key-points)

- SVG elements must be wrapped in an outer `<svg>` container
- The inner `<svg>` element should have the target ID
- All SVG element types (circle, rect, path, etc.) can be morphed
- Multiple SVG elements can be updated in a single morph operation
- CSS transitions work with SVG morphing for smooth animations