# Click To Load

Source: https://data-star.dev/examples/click_to_load

---

# Click To Load

Demo

| Name | Email | ID |
| --- | --- | --- |
| Agent Smith 0 | [[email protected]](/cdn-cgi/l/email-protection) | 1982e3a7bb241055 |
| Agent Smith 1 | [[email protected]](/cdn-cgi/l/email-protection) | 65cd25028f98f158 |
| Agent Smith 2 | [[email protected]](/cdn-cgi/l/email-protection) | 7b95a7322f5da314 |
| Agent Smith 3 | [[email protected]](/cdn-cgi/l/email-protection) | 7324dc1e7e9474f0 |
| Agent Smith 4 | [[email protected]](/cdn-cgi/l/email-protection) | 628911027fcf803f |
| Agent Smith 5 | [[email protected]](/cdn-cgi/l/email-protection) | 5edb980100c87e72 |
| Agent Smith 6 | [[email protected]](/cdn-cgi/l/email-protection) | 3564a48862bc4a0d |
| Agent Smith 7 | [[email protected]](/cdn-cgi/l/email-protection) | 6eed105b82285fa |
| Agent Smith 8 | [[email protected]](/cdn-cgi/l/email-protection) | 664f427c6b2c4bea |
| Agent Smith 9 | [[email protected]](/cdn-cgi/l/email-protection) | 28353a066812b268 |

Load More

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-signals='{"offset":0,"limit":10}' id="demo">
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
       ID
      </th>
     </tr>
    </thead>
    <tbody id="agents">
     <tr>
      <td>
       Agent Smith 0
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="d2a4bdbbb6e392bca7bebefcbda0b5" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       1982e3a7bb241055
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 1
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="cdbba2a4a9ff8da3b8a1a1e3a2bfaa" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       65cd25028f98f158
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 2
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="d9afb6b0bdea99b7acb5b5f7b6abbe" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       7b95a7322f5da314
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 3
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="b8ced7d1dc8cf8d6cdd4d496d7cadf" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       7324dc1e7e9474f0
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 4
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="4731282e23720729322b2b69283520" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       628911027fcf803f
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 5
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="ec9a838588daac82998080c2839e8b" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       5edb980100c87e72
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 6
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="f4829b9d90c3b49a819898da9b8693" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       3564a48862bc4a0d
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 7
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="8afce5e3eeb2cae4ffe6e6a4e5f8ed" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       6eed105b82285fa
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 8
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="43352c2a277a032d362f2f6d2c3124" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       664f427c6b2c4bea
      </td>
     </tr>
     <tr>
      <td>
       Agent Smith 9
      </td>
      <td>
       <a class="__cf_email__" data-cfemail="cfb9a0a6abfeff8fa1baa3a3e1a0bda8" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       28353a066812b268
      </td>
     </tr>
    </tbody>
   </table>
   <button class="info wide" data-attr-aria-disabled="`${$_fetching}`" data-indicator-_fetching="" data-on-click="!$_fetching &amp;&amp; @get('/examples/click_to_load/more')">
    Load More
   </button>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This example shows how to implement click-to-load the next page in a table of data. The crux of the example is the final row:

```html
1<button
2    class="info wide"
3    data-indicator-_fetching
4    data-attr-aria-disabled="`${$_fetching}`"
5    data-on-click="!$_fetching && @get('/examples/click_to_load/more')"
6>
7    Load More
8</button>
```

After clicking this button, the server responds with a set of elements in a `text/event-stream` with the next page of results. And so on.