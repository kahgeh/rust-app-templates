# Edit Row

Source: https://data-star.dev/examples/edit_row

---

# Edit Row

Demo

| Name | Email | Actions |
| --- | --- | --- |
| Joe Smith | [[email protected]](/cdn-cgi/l/email-protection) | Edit |
| Angie MacDowell | [[email protected]](/cdn-cgi/l/email-protection) | Edit |
| Fuqua Tarkenton | [[email protected]](/cdn-cgi/l/email-protection) | Edit |
| Kim Yee | [[email protected]](/cdn-cgi/l/email-protection) | Edit |

Reset

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-signals-_fetching__ifmissing="false" id="demo">
   <table>
    <thead>
     <tr>
      <th>
       Name
      </th>
      <th>
       Email
      </th>
      <th>
       Actions
      </th>
     </tr>
    </thead>
    <tbody>
     <tr>
      <td>
       Joe Smith
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="a6ccc9c3e6d5cbcfd2ce88c9d4c1" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="small info" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/edit_row/0')">
        <iconify-icon icon="pixelarticons:edit" noobserver="">
        </iconify-icon>
        Edit
       </button>
      </td>
     </tr>
     <tr>
      <td>
       Angie MacDowell
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="e4858a838d81a4898587808b93818888ca8b9683" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="small info" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/edit_row/1')">
        <iconify-icon icon="pixelarticons:edit" noobserver="">
        </iconify-icon>
        Edit
       </button>
      </td>
     </tr>
     <tr>
      <td>
       Fuqua Tarkenton
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="a0c6d5d1d5c1e0d4c1d2cbc5ced4cfce8ecfd2c7" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="small info" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/edit_row/2')">
        <iconify-icon icon="pixelarticons:edit" noobserver="">
        </iconify-icon>
        Edit
       </button>
      </td>
     </tr>
     <tr>
      <td>
       Kim Yee
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="701b191d300915155e1f0217" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="small info" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@get('/examples/edit_row/3')">
        <iconify-icon icon="pixelarticons:edit" noobserver="">
        </iconify-icon>
        Edit
       </button>
      </td>
     </tr>
    </tbody>
   </table>
   <div>
    <button class="warning" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@put('/examples/edit_row/reset')">
     <iconify-icon icon="pixelarticons:reload" noobserver="">
     </iconify-icon>
     Reset
    </button>
   </div>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This example shows how to implement editable rows. First let's look at the row prior to editing:

```html
1<tr>
2    <td>Joe Smith</td>
3    <td>[email protected]</td>
4    <td>
5        <button data-on-click="@get('/examples/edit_row/0')">
6            Edit
7        </button>
8    </td>
9</tr>
```

This will trigger a whole table replacement as we are going to remove the edit buttons from other rows as well as change out the inputs to allow editing.

Finally, here is what the row looks like when the data is being edited:

```html
 1<tr>
 2    <td>
 3        <input type="text" data-bind-name>
 4    </td>
 5    <td>
 6        <input type="text" data-bind-email>
 7    </td>
 8    <td>
 9        <button data-on-click="@get('/examples/edit_row/cancel')">
10            Cancel
11        </button>
12        <button data-on-click="@patch('/examples/edit_row/0')">
13            Save
14        </button>
15    </td>
16</tr>
```

Here we have a few things going on, clicking the cancel button will bring back the read-only version of the row. Finally, there is a save button that issues a `PATCH` to update the contact.