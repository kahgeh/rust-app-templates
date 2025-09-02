# Sortable

Source: https://data-star.dev/examples/sortable

---

# Sortable

Demo

Item 1 Item 2 Item 3 Item 4 Item 5

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <p data-signals-order-info="'Initial order'" data-text="$orderInfo">
  </p>
  <div data-on-reordered="$orderInfo = event.detail.orderInfo" id="sortContainer">
   <a class="button">
    Item 1
   </a>
   <a class="button">
    Item 2
   </a>
   <a class="button">
    Item 3
   </a>
   <a class="button">
    Item 4
   </a>
   <a class="button">
    Item 5
   </a>
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

Datastar allows you to listen for custom events using `data-on-*` and react to them by modifying signals.

```html
 1<div data-signals-order-info="'Initial order'" data-text="$orderInfo"></div>
 2<div id="sortContainer" data-on-reordered="$orderInfo = event.detail.orderInfo">
 3    <button>Item 1</button>
 4    <button>Item 2</button>
 5    <button>Item 3</button>
 6    <button>Item 4</button>
 7    <button>Item 5</button>
 8</div>
 9
10<script type="module">
11    import Sortable from 'https://cdn.jsdelivr.net/npm/sortablejs/+esm'
12    new Sortable(sortContainer, {
13        animation: 150,
14        ghostClass: 'opacity-25',
15        onEnd: (evt) => {
16            sortContainer.dispatchEvent(
17                new CustomEvent('reordered', {detail: {
18                    orderInfo: `Moved from position ${evt.oldIndex + 1} to ${evt.newIndex + 1}`
19                }})
20            )
21        }
22    })
23</script>
```

We create an `orderInfo` signal and modify it whenever a `reordered` event is triggered.

We instruct the [SortableJS](https://sortablejs.github.io/Sortable/) library to dispatch a custom event `reordered` whenever the sortable list is changed. This event contains the order information that we can use to update the `orderInfo` signal.