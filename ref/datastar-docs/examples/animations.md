# Animations

Source: https://data-star.dev/examples/animations

---

# Animations

Datastar is designed to allow you to use CSS transitions and the new View Transitions API to add smooth animations and transitions to your web page using only CSS and HTML. Below are a few examples of various animation techniques.

## Color Throb [#](#color-throb)

The simplest animation technique in Datastar is to keep the id of an element stable across a content swap. If the id of an element is kept stable, Datastar will swap it in such a way that CSS transitions can be written between the old version of the element and the new one.

Consider this div

Demo

brown on orange

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-on-load="@get('/examples/animations/throb')" id="throb" style="color: var(--brown-8); background-color: var(--orange-5);">
   brown on orange
  </div>
 </div>
</fieldset>
```

With SSE, we just update the style every second

```html
1<div
2    id="color-throb"
3    style="color: var(--blue-8); background-color: var(--orange-5);"
4>
5    blue on orange
6</div>
```

## View Transitions [#](#view-transitions)

The swapping of the button below is happening on the backend. Each click is causing a transition of state. The animated opacity animation is provided automatically by the View Transition API (not yet supported by Firefox). Doesn't matter if the targeted elements are different types, it will still "do the right thing".

Demo

Swap It!

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button data-attr-disabled="$_vtFetching" data-indicator-_vt-fetching="" data-on-click="@get('/examples/animations/view_transition')" data-signals='{"shouldRestore":false}' id="view-transition">
   Swap It!
  </button>
 </div>
</fieldset>
```

## Fade Out On Swap [#](#fade-out-on-swap)

If you want to fade out an element that is going to be removed when the request ends, just send an SSE event with the opacity set to 0 and set a transition duration. This will fade out the element before it is removed.

Demo

Fade out then delete on click

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button data-attr-disabled="$_fadeOutFetching" data-indicator-_fade-out-fetching="" data-on-click="@delete('/examples/animations')" id="fade-out-swap">
   Fade out then delete on click
  </button>
 </div>
</fieldset>
```

## Fade In On Addition [#](#fade-in-on-addition)

Building on the previous example, we can fade in the new content the same way, starting from an opacity of 0 and transitioning to an opacity of 1.

Demo

Fade me in on click

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <button data-attr-disabled="$_fadeInFetching" data-indicator-_fade-in-fetching="" data-on-click="@get('/examples/animations/fade_me_in')" id="fade-me-in" style="transition: opacity 1s ease-out">
   Fade me in on click
  </button>
 </div>
</fieldset>
```