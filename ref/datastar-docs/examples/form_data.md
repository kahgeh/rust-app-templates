# Form Data

Source: https://data-star.dev/examples/form_data

---

# Form Data

Demo

foo:  bar:  baz:

Submit GET request   Submit POST request

Submit GET request from outside the form

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <form id="myform">
   <p>
    foo:
    <input name="checkboxes" type="checkbox" value="foo"/>
    bar:
    <input name="checkboxes" type="checkbox" value="bar"/>
    baz:
    <input name="checkboxes" type="checkbox" value="baz"/>
   </p>
   <p>
    <button data-on-click="@get('/examples/form_data/data', {contentType: 'form'})">
     Submit GET request
    </button>
    <button data-on-click="@post('/examples/form_data/data', {contentType: 'form'})">
     Submit POST request
    </button>
   </p>
  </form>
  <button data-on-click="@get('/examples/form_data/data', {contentType: 'form', selector: '#myform'})">
   Submit GET request from outside the form
  </button>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

Setting the `contentType` option to `form` tells the `@get()` action to look for the closest form, perform validation on it, and send all form elements within it to the backend. A `selector` option can be provided to specify a form element. No signals are sent to the backend in this type of request.

```html
 1<form id="myform">
 2    foo:<input type="checkbox" name="checkboxes" value="foo" />
 3    bar:<input type="checkbox" name="checkboxes" value="bar" />
 4    baz:<input type="checkbox" name="checkboxes" value="baz" />
 5    <button data-on-click="@get('/endpoint', {contentType: 'form'})">
 6        Submit GET request
 7    </button>
 8    <button data-on-click="@post('/endpoint', {contentType: 'form'})">
 9        Submit POST request
10    </button>
11</form>
12
13<button data-on-click="@get('/endpoint', {contentType: 'form', selector: '#myform'})">
14    Submit GET request from outside the form
15</button>
```

Demo

foo:

Submit form

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <form data-on-submit="@get('/examples/form_data/data', {contentType: 'form'})">
   <p>
    foo:
    <input name="foo" required="" type="text"/>
   </p>
   <button>
    Submit form
   </button>
  </form>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

In this example, the `@get()` action is placed inside a submit listener on the form element using `data-on-submit`.

```html
1<form data-on-submit="@get('/endpoint', {contentType: 'form'})">
2    foo: <input type="text" name="foo" required />
3    <button>
4        Submit form
5    </button>
6</form>
```