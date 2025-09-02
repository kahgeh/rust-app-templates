# Web Component

Source: https://data-star.dev/examples/web_component

---

# Web Component

Demo

Reversed

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <label>
   Reversed
   <input data-bind-_name="" type="text" value="Your Name"/>
  </label>
  <span data-signals-_reversed="" data-text="$_reversed">
  </span>
  <reverse-component data-attr-name="$_name" data-on-reverse="$_reversed = evt.detail.value">
  </reverse-component>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This is an example of two-way binding with a web component that reverses a string. Normally, the web component would output the reversed value, but in this example, all it does is perform the logic and dispatch an event containing the result, which is then displayed.

```html
1<label>
2    Reversed
3    <input type="text" value="Your Name" data-bind-_name/>
4</label>
5<span data-signals-_reversed data-text="$_reversed"></span>
6<reverse-component
7    data-on-reverse="$_reversed = evt.detail.value"
8    data-attr-name="$_name"
9></reverse-component>
```

The `name` attribute value is bound to the `$_name` signal's value, and an event listener modifies the `$_reversed` signal's value sent in the `reverse` event. The web component observes changes to the `name` attribute and responds by reversing the string and dispatching a `reverse` event containing the resulting value.

```html
 1class ReverseComponent extends HTMLElement {
 2    static get observedAttributes() {
 3        return ["name"];
 4    }
 5
 6    attributeChangedCallback(name, oldValue, newValue) {
 7        const len = newValue.length;
 8        let value = Array(len);
 9        let i = len - 1;
10        for (const char of newValue) {
11            value[i--] = char.codePointAt(0);
12        }
13        value = String.fromCodePoint(...value);
14        this.dispatchEvent(new CustomEvent("reverse", { detail: { value } }));
15    }
16}
17
18customElements.define("reverse-component", ReverseComponent);
```