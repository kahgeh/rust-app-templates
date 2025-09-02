# Custom Event

Source: https://data-star.dev/examples/custom_event

---

# Custom Event

Demo

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <p data-on-myevent="$_eventDetails = evt.detail" data-signals-_event-details="" data-text="`Last Event Details: ${$_eventDetails}`" id="foo">
  </p>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<p
 2    id="foo"
 3    data-signals-_event-details
 4    data-on-myevent="$_eventDetails = evt.detail"
 5    data-text="`Last Event Details: ${$_eventDetails}`"
 6></p>
 7<script>
 8    const foo = document.getElementById("foo");
 9    setInterval(() => {
10        foo.dispatchEvent(
11            new CustomEvent("myevent", {
12                detail: JSON.stringify({
13                    eventTime: new Date().toLocaleTimeString(),
14                }),
15            })
16        );
17    }, 1000);
18</script>
```

## Explanation [#](#explanation)

The `data-on-*` plugin can listen to any event, including custom events. In this example, we are listening to a custom event myevent on the foo element. When the event is triggered, the `$_eventDetails` signal is set to the event's details.

This is primarily used when interacting with Web Components or other custom elements that emit custom events.

### Note [#](#note)

There is an extra variable `evt` available in the event handler that contains the event object. This is used to access the event details like `evt.detail` in this example.