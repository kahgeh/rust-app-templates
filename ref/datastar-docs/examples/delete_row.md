# Delete Row

Source: https://data-star.dev/examples/delete_row

---

# Delete Row

Demo

| Name | Email | Actions |
| --- | --- | --- |
| Joe Smith | [[email protected]](/cdn-cgi/l/email-protection) | Delete |
| Angie MacDowell | [[email protected]](/cdn-cgi/l/email-protection) | Delete |
| Fuqua Tarkenton | [[email protected]](/cdn-cgi/l/email-protection) | Delete |
| Kim Yee | [[email protected]](/cdn-cgi/l/email-protection) | Delete |

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
       <a class="__cf_email__" data-cfemail="2d4742486d5e4044594503425f4a" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="error" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="confirm('Are you sure?') &amp;&amp; @delete('/examples/delete_row/0')">
        <iconify-icon icon="pixelarticons:trash" noobserver="">
        </iconify-icon>
        Delete
       </button>
      </td>
     </tr>
     <tr>
      <td>
       Angie MacDowell
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="ccada2aba5a98ca1adafa8a3bba9a0a0e2a3beab" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="error" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="confirm('Are you sure?') &amp;&amp; @delete('/examples/delete_row/1')">
        <iconify-icon icon="pixelarticons:trash" noobserver="">
        </iconify-icon>
        Delete
       </button>
      </td>
     </tr>
     <tr>
      <td>
       Fuqua Tarkenton
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="bcdac9cdc9ddfcc8ddced7d9d2c8d3d292d3cedb" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="error" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="confirm('Are you sure?') &amp;&amp; @delete('/examples/delete_row/2')">
        <iconify-icon icon="pixelarticons:trash" noobserver="">
        </iconify-icon>
        Delete
       </button>
      </td>
     </tr>
     <tr>
      <td>
       Kim Yee
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="98f3f1f5d8e1fdfdb6f7eaff" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       <button class="error" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="confirm('Are you sure?') &amp;&amp; @delete('/examples/delete_row/3')">
        <iconify-icon icon="pixelarticons:trash" noobserver="">
        </iconify-icon>
        Delete
       </button>
      </td>
     </tr>
    </tbody>
   </table>
   <button class="warning" data-indicator-_fetching="" data-on-click="@put('/examples/delete_row/reset')">
    Reset
   </button>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This example shows how to implement a delete button that removes a table row upon completion. First let's look at the table body:

```html
 1<table>
 2    <thead>
 3        <tr>
 4            <th>Name</th>
 5            <th>Email</th>
 6            <th>Actions</th>
 7        </tr>
 8    </thead>
 9    <tbody>
10        <tr>
11            <td>Joe Smith</td>
12            <td>[email protected]</td>
13            <td>
14                <button
15                    class="error"
16                    data-on-click="confirm('Are you sure?') && @delete('/examples/delete_row/0')"
17                    data-indicator-_fetching
18                    data-attr-disabled="$_fetching"
19                >
20                    Delete
21                </button>
22            </td>
23        </tr>
24    </tbody>
25</table>
```

The row has a normal confirm to `confirm()` the delete action.