# Lazy Tabs

Source: https://data-star.dev/examples/lazy_tabs

---

# Lazy Tabs

Demo

Tab 0Tab 1Tab 2Tab 3Tab 4Tab 5Tab 6Tab 7

Voluptatum magnam eos doloremque accusamus vel. Non officia et perspiciatis ab deserunt. Odit laudantium et aut fuga nam. Id atque sint et commodi amet. Eligendi nobis animi sunt quis quo.

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div id="lazy-tabs">
   <div role="tablist">
    <button aria-selected="true" data-on-click="@get('/examples/lazy_tabs/0')" role="tab">
     Tab 0
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/1')" role="tab">
     Tab 1
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/2')" role="tab">
     Tab 2
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/3')" role="tab">
     Tab 3
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/4')" role="tab">
     Tab 4
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/5')" role="tab">
     Tab 5
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/6')" role="tab">
     Tab 6
    </button>
    <button aria-selected="false" data-on-click="@get('/examples/lazy_tabs/7')" role="tab">
     Tab 7
    </button>
   </div>
   <div role="tabpanel">
    <p>
     Voluptatum magnam eos doloremque accusamus vel. Non officia et perspiciatis ab deserunt. Odit laudantium et aut fuga nam. Id atque sint et commodi amet. Eligendi nobis animi sunt quis quo.
    </p>
   </div>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div id="demo">
 2    <div role="tablist">
 3        <button
 4            role="tab"
 5            aria-selected="true"
 6            data-on-click="@get('/examples/lazy_tabs/0')"
 7        >
 8            Tab 0
 9        </button>
10        <button
11            role="tab"
12            aria-selected="false"
13            data-on-click="@get('/examples/lazy_tabs/1')"
14        >
15            Tab 1
16        </button>
17        <button
18            role="tab"
19            aria-selected="false"
20            data-on-click="@get('/examples/lazy_tabs/2')"
21        >
22            Tab 2
23        </button>
24        <!-- More tabs... -->
25    </div>
26    <div role="tabpanel">
27        <p>Lorem ipsum dolor sit amet...</p>
28        <p>Consectetur adipiscing elit...</p>
29        <!-- Tab content -->
30    </div>
31</div>
```

## Explanation [#](#explanation)

This example shows how easy it is to implement tabs using Datastar. Following the principles of Hypertext As The Engine Of Application State, the selected tab is a part of the application state. Therefore, to display and select tabs in your application, simply include the tab markup in the returned HTML fragment.