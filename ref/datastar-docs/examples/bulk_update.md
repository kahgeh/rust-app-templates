# Bulk Update

Source: https://data-star.dev/examples/bulk_update

---

# Bulk Update

Demo

|  | Name | Email | Status |
| --- | --- | --- | --- |
|  | Joe Smith | [[email protected]](/cdn-cgi/l/email-protection) | Active |
|  | Angie MacDowell | [[email protected]](/cdn-cgi/l/email-protection) | Active |
|  | Fuqua Tarkenton | [[email protected]](/cdn-cgi/l/email-protection) | Active |
|  | Kim Yee | [[email protected]](/cdn-cgi/l/email-protection) | Active |

Activate Deactivate

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-signals__ifmissing="{_fetching: false, selections: []}" id="demo">
   <table>
    <thead>
     <tr>
      <th>
       <input data-attr-disabled="$_fetching" data-effect="el.checked = $selections.every(Boolean)" data-on-change="@setAll(el.checked, {include: /^selections/})" type="checkbox"/>
      </th>
      <th>
       Name
      </th>
      <th>
       Email
      </th>
      <th>
       Status
      </th>
     </tr>
    </thead>
    <tbody>
     <tr>
      <td>
       <input data-attr-disabled="$_fetching" data-bind-selections="" type="checkbox"/>
      </td>
      <td>
       Joe Smith
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="bed4d1dbfecdd3d7cad690d1ccd9" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       Active
      </td>
     </tr>
     <tr>
      <td>
       <input data-attr-disabled="$_fetching" data-bind-selections="" type="checkbox"/>
      </td>
      <td>
       Angie MacDowell
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="c2a3aca5aba782afa3a1a6adb5a7aeaeecadb0a5" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       Active
      </td>
     </tr>
     <tr>
      <td>
       <input data-attr-disabled="$_fetching" data-bind-selections="" type="checkbox"/>
      </td>
      <td>
       Fuqua Tarkenton
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="badccfcbcfdbfacedbc8d1dfd4ced5d494d5c8dd" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       Active
      </td>
     </tr>
     <tr>
      <td>
       <input data-attr-disabled="$_fetching" data-bind-selections="" type="checkbox"/>
      </td>
      <td>
       Kim Yee
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="3f5456527f465a5a11504d58" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       Active
      </td>
     </tr>
    </tbody>
   </table>
   <div role="group">
    <button class="success" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@put('/examples/bulk_update/activate')">
     <iconify-icon icon="pixelarticons:user-plus" noobserver="">
     </iconify-icon>
     Activate
    </button>
    <button class="error" data-attr-disabled="$_fetching" data-indicator-_fetching="" data-on-click="@put('/examples/bulk_update/deactivate')">
     <iconify-icon icon="pixelarticons:user-x" noobserver="">
     </iconify-icon>
     Deactivate
    </button>
   </div>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div
 2    id="demo"
 3    data-signals__ifmissing="{_fetching: false, selections: Array(4).fill(false)}"
 4>
 5    <table>
 6        <thead>
 7            <tr>
 8                <th>
 9                    <input
10                        type="checkbox"
11                        data-bind-_all
12                        data-on-change="$selections = Array(4).fill($_all)"
13                        data-effect="$selections; $_all = $selections.every(Boolean)"
14                        data-attr-disabled="$_fetching"
15                    />
16                </th>
17                <th>Name</th>
18                <th>Email</th>
19                <th>Status</th>
20            </tr>
21        </thead>
22        <tbody>
23            <tr>
24                <td>
25                    <input
26                        type="checkbox"
27                        data-bind-selections
28                        data-attr-disabled="$_fetching"
29                    />
30                </td>
31                <td>Joe Smith</td>
32                <td>[email protected]</td>
33                <td>Active</td>
34            </tr>
35            <!-- More rows... -->
36        </tbody>
37    </table>
38    <div role="group">
39        <button
40            class="success"
41            data-on-click="@put('/examples/bulk_update/activate')"
42            data-indicator-_fetching
43            data-attr-disabled="$_fetching"
44        >
45            <i class="pixelarticons:user-plus"></i>
46            Activate
47        </button>
48        <button
49            class="error"
50            data-on-click="@put('/examples/bulk_update/deactivate')"
51            data-indicator-_fetching
52            data-attr-disabled="$_fetching"
53        >
54            <i class="pixelarticons:user-x"></i>
55            Deactivate
56        </button>
57    </div>
58</div>
```

## Explanation [#](#explanation)

This example shows how to implement a common pattern where rows are selected and then bulk updated. This is accomplished by putting a form around a table, with checkboxes in the table, and then including the checked values in `PUT`s to two different endpoints: activate and deactivate.

The server will either activate or deactivate the checked users and then re-render the table with updated rows.