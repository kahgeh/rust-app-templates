# Title Update

Source: https://data-star.dev/examples/title_update

---

# Title Update

Demo

Look at the title change in the browser tab!

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <p data-on-load="@get('/examples/title_update/updates')">
   Look at the title change in the browser tab!
  </p>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

A user in the Discord channel was asking about needing a plugin similar to htmx's head support to update title or head elements. With Datastar this is unnecessary as you can just update the title directly with a patch elements event.

```html
1event: datastar-patch-elements
2data: selector title
3data: elements <title>08:30:36</title>
```