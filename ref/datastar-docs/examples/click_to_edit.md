# Click To Edit

Source: https://data-star.dev/examples/click_to_edit

---

# Click To Edit

Demo

First Name: John

Last Name: Doe

Email: [[email protected]](/cdn-cgi/l/email-protection)

Edit Reset

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div id="demo">
   <p>
    First Name: John
   </p>
   <p>
    Last Name: Doe
   </p>
   <p>
    Email:
    <a class="__cf_email__" data-cfemail="bfd5d0daffddd3d0c891dcd0d2" href="/cdn-cgi/l/email-protection">
     [email protected]
    </a>
   </p>
   <div role="group">
    <button class="info" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/click_to_edit/edit')">
     Edit
    </button>
    <button class="warning" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@patch('/examples/click_to_edit/reset')">
     Reset
    </button>
   </div>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

The click to edit pattern is a way to inline edit all or part of a record without a page refresh. This pattern starts with a UI that shows the details of a contact. The div has a button that will get the editing UI for the contact from `/edit`

```html
 1<div id="demo">
 2    <p>First Name: John</p>
 3    <p>Last Name: Doe</p>
 4    <p>Email: [email protected]</p>
 5    <div role="group">
 6        <button
 7            class="info"
 8            data-indicator-_fetching
 9            data-attr-disabled="$_fetching"
10            data-on-click="@get('/examples/click_to_edit/edit')"
11        >
12            Edit
13        </button>
14        <button
15            class="warning"
16            data-indicator-_fetching
17            data-attr-disabled="$_fetching"
18            data-on-click="@patch('/examples/click_to_edit/reset')"
19        >
20            Reset
21        </button>
22    </div>
23</div>
```

This returns a form that can be used to edit the contact

```html
 1<div id="demo">
 2    <label>
 3        First Name
 4        <input
 5            type="text"
 6            data-bind-first-name
 7            data-attr-disabled="$_fetching"
 8        >
 9    </label>
10    <label>
11        Last Name
12        <input
13            type="text"
14            data-bind-last-name
15            data-attr-disabled="$_fetching"
16        >
17    </label>
18    <label>
19        Email
20        <input
21            type="email"
22            data-bind-email
23            data-attr-disabled="$_fetching"
24        >
25    </label>
26    <div role="group">
27        <button
28            class="success"
29            data-indicator-_fetching
30            data-attr-disabled="$_fetching"
31            data-on-click="@put('/examples/click_to_edit')"
32        >
33            Save
34        </button>
35        <button
36            class="error"
37            data-indicator-_fetching
38            data-attr-disabled="$_fetching"
39            data-on-click="@get('/examples/click_to_edit/cancel')"
40        >
41            Cancel
42        </button>
43    </div>
44</div>
```

### There Is No Form [#](#there-is-no-form)

If you compare to htmx you'll notice there is no form, you can use one, but it's unnecessary. This is because you're already using signals and when you `PUT` to `/edit`, the body is the entire contents of the signals, and it's available to handle errors and validation holistically. There is also a profanity filter on the normal rendering of the contact that is not applied to the edit form. Controlling the rendering completely on the server allows you to have a single source of truth for the data and the rendering.

### There Is No Client Side Validation [#](#there-is-no-client-side-validation)

On the backend we've also added a quick sanitizer on the input to avoid bad actors (to some degree). You already have to deal with the data on the server so you might as well do the validation there. In this case, its just modifying how the text is rendered when not editing. This is a simple example, but you can see how to extend it to more complex forms.