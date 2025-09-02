# Active Search

Source: https://data-star.dev/examples/active_search

---

# Active Search

Demo

| First Name | Last Name |
| --- | --- |
| Marguerite | Kerluke |
| Willa | Bergnaum |
| Roderick | O'Conner |
| Karl | Bode |
| Verlie | Kemmer |
| Hanna | Fay |
| Heidi | Hermann |
| Lavina | Gibson |
| Kareem | Collier |
| Maia | Pouros |

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div id="demo">
   <input data-bind-active-search="" data-on-input__debounce.200ms="@get('/examples/active_search/search')" placeholder="Search..." type="text"/>
   <table>
    <thead>
     <tr>
      <th>
       First Name
      </th>
      <th>
       Last Name
      </th>
     </tr>
    </thead>
    <tbody>
     <tr>
      <td>
       Marguerite
      </td>
      <td>
       Kerluke
      </td>
     </tr>
     <tr>
      <td>
       Willa
      </td>
      <td>
       Bergnaum
      </td>
     </tr>
     <tr>
      <td>
       Roderick
      </td>
      <td>
       O'Conner
      </td>
     </tr>
     <tr>
      <td>
       Karl
      </td>
      <td>
       Bode
      </td>
     </tr>
     <tr>
      <td>
       Verlie
      </td>
      <td>
       Kemmer
      </td>
     </tr>
     <tr>
      <td>
       Hanna
      </td>
      <td>
       Fay
      </td>
     </tr>
     <tr>
      <td>
       Heidi
      </td>
      <td>
       Hermann
      </td>
     </tr>
     <tr>
      <td>
       Lavina
      </td>
      <td>
       Gibson
      </td>
     </tr>
     <tr>
      <td>
       Kareem
      </td>
      <td>
       Collier
      </td>
     </tr>
     <tr>
      <td>
       Maia
      </td>
      <td>
       Pouros
      </td>
     </tr>
    </tbody>
   </table>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This example actively searches a contacts database as the user enters text.

The interesting part is the input field:

```html
1<input
2    type="text"
3    placeholder="Search..."
4    data-bind-search
5    data-on-input__debounce.200ms="@get('/examples/active_search/search')"
6/>
```

The input issues a `GET` to `/active_search/search` with the input value bound to `$search`. The `__debounce.200ms` modifier ensures that the search is not issued on every keystroke, but only after the user has stopped typing.