# How to load more list items

Source: https://data-star.dev/how_tos/load_more_list_items

---

# How to load more list items

Loading more list items into the DOM from the backend is a common alternative to pagination. What makes it different is that we need to append the new items to the existing list, rather than replace them.

## Goal [#](#goal)

Our goal is to incrementally append list items into a specific part of the DOM, each time a button is clicked. Once five items are visible, the button should be removed.

Demo

- Item 1

Click to load another item

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <ul data-signals-offset="1" id="list">
   <li>
    Item 1
   </li>
  </ul>
  <button data-on-click="@get('/how_tos/load_more_list_items/more_items')" id="load-more">
   Click to load another item
  </button>
 </div>
</fieldset>
```

## Steps [#](#steps)

We'll give the list item container and the button unique IDs, so that we can target them individually.

We'll use a `data-signals-*` attribute to set the initial `offset` to `1`, and a `data-on-click` button that will send a `GET` request to the backend.

```html
1<div id="list">
2<div>Item 1</div>
3</div>
4<button id="load-more" 
5        data-signals-offset="1" 
6        data-on-click="@get('/how_tos/load_more/data')">
7Click to load another item
8</button>
```

The backend will receive the `offset` signal and, if not above the max number of allowed items, will return the next item to be appended to the list.

We'll set up our backend to send a [`datastar-patch-elements`](/reference/sse_events#datastar-patch-elements) event with the `selector` option set to `#list` and the `mode` option set to `append`. This tells Datastar to \_append\_ the elements *into* the `#list` container (rather than the default behaviour of replacing it).

```html
1event: datastar-patch-elements
2data: selector #list
3data: mode append
4data: elements <div>Item 2</div>
```

In addition, we'll send a [`datastar-patch-signals`](/reference/sse_events#datastar-patch-signals) event to update the `offset`.

```html
1event: datastar-patch-signals
2data: signals {offset: 2}
```

In the case when all five list items have been shown, we'll remove the button from the DOM entirely.

```html
1event: datastar-patch-elements
2data: selector #load-more
3data: mode remove
```

Here's how it might look using the SDKs.

```html
 1(require
 2  '[starfederation.datastar.clojure.api :as d*]
 3  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]]
 4  '[some.hiccup.library :refer [html]]
 5  '[some.json.library :refer [read-json-str write-json-str]]))
 6
 7
 8(def max-offset 5)
 9
10(defn handler [ring-request]
11  (->sse-response ring-request
12    {on-open
13     (fn [sse]
14       (let [d*-signals (-> ring-request d*/get-signals read-json-str)
15             offset (get d*-signals "offset")
16             limit 1
17             new-offset (+ offset limit)]
18
19         (d*/patch-elements! sse
20                             (html [:div "Item " new-offset])
21                             {d*/selector   "#list"
22                              d*/merge-mode d*/mm-append})
23
24         (if (< new-offset max-offset)
25           (d*/patch-signals! sse (write-json-str {"offset" new-offset}))
26           (d*/remove-fragment! sse "#load-more"))
27
28         (d*/close-sse! sse)))}))
```

```html
 1using System.Text.Json;
 2using StarFederation.Datastar;
 3using StarFederation.Datastar.DependencyInjection;
 4
 5public class Program
 6{
 7    public record OffsetSignals(int offset);
 8
 9    public static void Main(string[] args)
10    {
11        var builder = WebApplication.CreateBuilder(args);
12        builder.Services.AddDatastar();
13        var app = builder.Build();
14
15        app.MapGet("/more", async (IDatastarService datastarService) =>
16        {
17            var max = 5;
18            var limit = 1;
19            var signals = await datastarService.ReadSignalsAsync<OffsetSignals>();
20            var offset = signals.offset;
21            if (offset < max)
22            {
23                var newOffset = offset + limit;
24                await datastarService.PatchElementsAsync($"<div>Item {newOffset}</div>", new()
25                {
26                    Selector = "#list",
27                    PatchMode = PatchElementsMode.Append,
28                });
29                if (newOffset < max)
30                    await datastarService.PatchSignalsAsync(new OffsetSignals(newOffset));
31                else
32                    await datastarService.RemoveElementAsync("#load-more");
33            }
34        });
35
36        app.Run();
37    }
38}
```

```html
 1import (
 2    "fmt"
 3    "net/http"
 4
 5    "github.com/go-chi/chi/v5"
 6    "github.com/starfederation/datastar-go/datastar"
 7)
 8
 9type OffsetSignals struct {
10    Offset int `json:"offset"`
11}
12
13signals := &OffsetSignals{}
14if err := datastar.ReadSignals(r, signals); err != nil {
15    http.Error(w, err.Error(), http.StatusBadRequest)
16}
17
18max := 5
19limit := 1
20offset := signals.Offset
21
22sse := datastar.NewSSE(w, r)
23
24if offset < max {
25    newOffset := offset + limit
26    sse.PatchElements(fmt.Sprintf(`<div>Item %d</div>`, newOffset),
27        datastar.WithSelectorID("list"),
28        datastar.WithModeAppend(),
29    )
30    if newOffset < max {
31        sse.PatchSignals([]byte(fmt.Sprintf(`{offset: %d}`, newOffset)))
32    } else {
33        sse.RemoveElements(`#load-more`)
34    }
35}
```

No example found for Java

```html
 1use starfederation\datastar\enums\ElementPatchMode;
 2use starfederation\datastar\ServerSentEventGenerator;
 3
 4$signals = ServerSentEventGenerator::readSignals();
 5
 6$max = 5;
 7$limit = 1;
 8$offset = $signals['offset'] ?? 1;
 9
10$sse = new ServerSentEventGenerator();
11
12if ($offset < $max) {
13    $newOffset = $offset + $limit;
14    $sse->patchElements("<div>Item $newOffset</div>", [
15        'selector' => '#list',
16        'mode' => ElementPatchMode::Append,
17    ]);
18    if (newOffset < $max) {
19        $sse->patchSignals(['offset' => $newOffset]);
20    } else {
21        $sse->removeElements('#load-more');
22    }
23}
```

```html
 1from datastar_py import ServerSentEventGenerator as SSE
 2from datastar_py.consts import ElementPatchMode
 3from datastar_py.fastapi import datastar_response, ReadSignals
 4
 5MAX_ITEMS = 5
 6
 7@app.get("/how_tos/load_more/data")
 8@datastar_response
 9async def load_data(signals: ReadSignals):
10    if signals["offset"] < MAX_ITEMS:
11        new_offset = signals["offset"] + 1
12        yield SSE.patch_elements(
13            f"<div>Item {new_offset}</div>",
14            mode=ElementPatchMode.APPEND,
15            selector="#list"
16        )
17        if new_offset < MAX_ITEMS:
18            yield SSE.patch_signals({"offset": new_offset})
19        else:
20            yield SSE.remove_elements("#load-more")
```

No example found for Ruby

No example found for Rust

No example found for TypeScript

## Conclusion [#](#conclusion)

While using the default mode of `outer` is generally recommended, appending to a list is a good example of when to use the `append` mode.