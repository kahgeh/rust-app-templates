# Infinite Scroll

Source: https://data-star.dev/examples/infinite_scroll

---

# Infinite Scroll

The infinite scroll pattern provides a way to load content dynamically on user scrolling action.

Let's focus on the final row (or the last element of your content):

```html
1<div data-on-intersect="@get('/examples/infinite_scroll/more')">
2    Loading...
3</div>
```

This last element contains a listener which, when scrolled into view, will trigger a request. The result is then appended after it. `data-on-intersect` is an attribute that triggers a request when the element is scrolled into view.

Demo

Agents

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

Loading...![Indicator](/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif)

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-signals='{"offset":0,"limit":10}' id="demo">
   <table>
    <caption>
     Agents
    </caption>
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
       <a class="__cf_email__" data-cfemail="8cfae3e5e8bdcce2f9e0e0a2e3feeb" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="3a4c55535e087a544f56561455485d" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="dea8b1b7baed9eb0abb2b2f0b1acb9" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="4b3d24222f7f0b253e27276524392c" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="7d0b121419483d1308111153120f1a" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="c1b7aea8a5f781afb4adadefaeb3a6" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="c9bfa6a0adfe89a7bca5a5e7a6bbae" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="55233a3c316d153b2039397b3a2732" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="2751484e431e6749524b4b09485540" href="/cdn-cgi/l/email-protection">
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
       <a class="__cf_email__" data-cfemail="3741585e5306077759425b5b19584550" href="/cdn-cgi/l/email-protection">
        [email protected]
       </a>
      </td>
      <td>
       28353a066812b268
      </td>
     </tr>
    </tbody>
   </table>
   <div data-on-intersect="@get('/examples/infinite_scroll/more')">
    Loading...
    <img alt="Indicator" height="32" loading="" src="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif" srcset="/cdn-cgi/image/format=auto,width=32/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 1x, /cdn-cgi/image/format=auto,width=64/static/images/rocket-animated-1d781383a0d7cbb1eb575806abeec107c8a915806fb55ee19e4e33e8632c75e5.gif 2x" width="32"/>
   </div>
  </div>
 </div>
</fieldset>
```