# Inline Validation

Source: https://data-star.dev/examples/inline_validation

---

# Inline Validation

Demo

Email Address 

The only valid email address is "[[email protected]](/cdn-cgi/l/email-protection)".

First Name  Last Name  Sign Up

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div id="demo">
   <label>
    Email Address
    <input aria-describedby="email-info" aria-live="polite" data-bind-email="" data-on-keydown__debounce.500ms="@post('/examples/inline_validation/validate')" required="" type="email"/>
   </label>
   <p class="info" id="email-info">
    The only valid email address is "
    <a class="__cf_email__" data-cfemail="225647515662564751560c414d4f" href="/cdn-cgi/l/email-protection">
     [email protected]
    </a>
    ".
   </p>
   <label>
    First Name
    <input aria-live="polite" data-bind-first-name="" data-on-keydown__debounce.500ms="@post('/examples/inline_validation/validate')" required="" type="text"/>
   </label>
   <label>
    Last Name
    <input aria-live="polite" data-bind-last-name="" data-on-keydown__debounce.500ms="@post('/examples/inline_validation/validate')" required="" type="text"/>
   </label>
   <button aria-disabled="true" class="success">
    <iconify-icon icon="material-symbols:person-add" noobserver="">
    </iconify-icon>
    Sign Up
   </button>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div id="demo">
 2    <label>
 3        Email Address
 4        <input
 5            type="email"
 6            required
 7            aria-live="polite"
 8            aria-describedby="email-info"
 9            data-bind-email
10            data-on-keydown__debounce.500ms="@post('/examples/inline_validation/validate')"
11        />
12    </label>
13    <p id="email-info" class="info">The only valid email address is "[email protected]".</p>
14    <label>
15        First Name
16        <input
17            type="text"
18            required
19            aria-live="polite"
20            data-bind-first-name
21            data-on-keydown__debounce.500ms="@post('/examples/inline_validation/validate')"
22        />
23    </label>
24    <label>
25        Last Name
26        <input
27            type="text"
28            required
29            aria-live="polite"
30            data-bind-last-name
31            data-on-keydown__debounce.500ms="@post('/examples/inline_validation/validate')"
32        />
33    </label>
34    <button
35        class="success"
36        data-on-click="@post('/examples/inline_validation')"
37    >
38        <i class="material-symbols:person-add"></i>
39        Sign Up
40    </button>
41</div>
```

## Explanation [#](#explanation)

This example shows how to do inline field validation, in this case of an email address. To do this we need to create a form with an input that `POST`s back to the server with the value to be validated and updates the DOM with the validation results. Since it's easy to replace the whole form, the logic for displaying the validation results is kept simple.