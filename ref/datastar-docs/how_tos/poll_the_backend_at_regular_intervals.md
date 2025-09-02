# How to poll the backend at regular intervals

Source: https://data-star.dev/how_tos/poll_the_backend_at_regular_intervals

---

# How to poll the backend at regular intervals

Polling is a pull-based mechanism for fetching data from the server at regular intervals. It is useful when you want to refresh the UI on the frontend, based on real-time data from the backend.

This in contrast to a push-based mechanism, in which a long-lived SSE connection is kept open between the client and the server, and the server pushes updates to the client whenever necessary. Push-based mechanisms are more efficient than polling, and can be achieved using Datastar, but may be less desirable for some backends.

In PHP, for example, keeping long-lived SSE connections is fine for a dashboard in which users are authenticated, as the number of connections are limited. For a public-facing website, however, it is not recommended to open many long-lived connections, due to the architecture of most PHP servers.

## Goal [#](#goal)

Our goal is to poll the backend at regular intervals (starting at 5 second intervals) and update the UI accordingly. The backend will determine changes to the DOM and be able to control the rate at which the frontend polls based on some criteria. For this example, we will simply output the server time, increasing the polling frequency to 1 second during the last 10 seconds of every minute. The criteria could of course be anything such as the number of times previously polled, the user's role, load on the server, etc.

Demo

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <p data-on-interval__duration.5s.leading="@get('/how_tos/poll_the_backend_at_regular_intervals/current_time')" id="time">
  </p>
 </div>
</fieldset>
```

## Steps [#](#steps)

The `data-on-interval` attribute allows us to run an expression at a regular interval. We'll use it to send a `GET` request to the backend, and use the `__duration` modifier to set the interval duration.

```html
1<div id="time"
2     data-on-interval__duration.5s="@get('/endpoint')"
3></div>
```

In addition to the interval, we could also run the expression immediately by adding `.leading` to the modifier.

```html
1<div id="time"
2     data-on-interval__duration.5s.leading="@get('/endpoint')"
3></div>
```

Most of the time, however, we'd just render the current time on page load using a backend templating language.

```html
1<div id="time"
2     data-on-interval__duration.5s="@get('/endpoint')"
3>
4     {{ now }}
5</div>
```

Now our backend can respond to each request with a [`datastar-patch-elements`](/reference/sse_events#datastar-patch-elements) event with an updated version of the element.

```html
1event: datastar-patch-elements
2data: elements <div id="time" data-on-interval__duration.5s="@get('/endpoint')">
3data: elements     {{ now }}
4data: elements </div>
```

Be careful not to add `.leading` to the modifier in the response, as it will cause the frontend to immediately send another request.

Here's how it might look using the SDKs.

```html
 1(require
 2  '[starfederation.datastar.clojure.api :as d*]
 3  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]])
 4  '[some.hiccup.library :refer [html]])
 5
 6(import
 7  'java.time.format.DateTimeFormatter
 8  'java.time.LocalDateTime)
 9
10(def formatter (DateTimeFormatter/ofPattern "YYYY-MM-DD HH:mm:ss"))
11
12(defn handle [ring-request]
13   (->sse-response ring-request
14     {on-open
15      (fn [sse]
16        (d*/patch-elements! sse
17          (html [:div#time {:data-on-interval__duration.5s (d*/sse-get "/endpoint")}
18                  (LocalDateTime/.format (LocalDateTime/now) formatter)])))}))
19
20        (d*/close-sse! sse))}))
```

```html
 1using StarFederation.Datastar.DependencyInjection;
 2
 3app.MapGet("/endpoint", async (IDatastarService datastarService) =>
 4{
 5    var currentTime = DateTime.Now.ToString("yyyy-MM-dd hh:mm:ss");
 6    await datastarService.PatchElementsAsync($"""
 7        <div id="time" data-on-interval__duration.5s="@get('/endpoint')">
 8            {currentTime}
 9        </div>
10    """);
11});
```

```html
 1import (
 2    "time"
 3    "github.com/starfederation/datastar-go/datastar"
 4)
 5
 6currentTime := time.Now().Format("2006-01-02 15:04:05")
 7
 8sse := datastar.NewSSE(w, r)
 9sse.PatchElements(fmt.Sprintf(`
10    <div id="time" data-on-interval__duration.5s="@get('/endpoint')">
11        %s
12    </div>
13`, currentTime))
```

No example found for Java

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3$currentTime = date('Y-m-d H:i:s');
 4
 5$sse = new ServerSentEventGenerator();
 6$sse->patchElements(`
 7    <div id="time"
 8         data-on-interval__duration.5s="@get('/endpoint')"
 9    >
10        $currentTime
11    </div>
12`);
```

```html
 1from datastar_py import ServerSentEventGenerator as SSE
 2from datastar_py.sanic import DatastarResponse
 3
 4@app.get("/endpoint")
 5async def endpoint():
 6    current_time = datetime.now()
 7
 8    return DatastarResponse(SSE.patch_elements(f"""
 9        <div id="time" data-on-interval__duration.5s="@get('/endpoint')">
10            {current_time:%Y-%m-%d %H:%M:%S}
11        </div>
12    """))
```

```html
 1datastar = Datastar.new(request:, response:)
 2
 3current_time = Time.now.strftime('%Y-%m-%d %H:%M:%S')
 4
 5datastar.patch_elements <<~FRAGMENT
 6    <div id="time"
 7         data-on-interval__duration.5s="@get('/endpoint')"
 8    >
 9        #{current_time}
10    </div>
11FRAGMENT
```

```html
 1use datastar::prelude::*;
 2use chrono::Local;
 3use async_stream::stream;
 4
 5let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
 6
 7Sse(stream! {
 8    yield PatchElements::new(
 9        format!(
10            "<div id='time' data-on-interval__duration.5s='@get(\"/endpoint\")'>{}</div>",
11            current_time
12        )
13    ).into();
14})
```

```html
 1import { createServer } from "node:http";
 2import { ServerSentEventGenerator } from "../npm/esm/node/serverSentEventGenerator.js";
 3
 4const server = createServer(async (req, res) => {
 5  const currentTime = new Date().toISOString();
 6  
 7  ServerSentEventGenerator.stream(req, res, (sse) => {
 8    sse.patchElements(`
 9       <div id="time"
10          data-on-interval__duration.5s="@get('/endpoint')"
11       >
12         ${currentTime}
13       </div>
14    `);
15  });
16});
```

Our second requirement was that the polling frequency should increase to 1 second during the last 10 seconds of every minute. To make this possible, we'll calculate and output the interval duration based on the current seconds of the minute.

```html
 1(require
 2  '[starfederation.datastar.clojure.api :as d*]
 3  '[starfederation.datastar.clojure.adapter.http-kit :refer [->sse-response on-open]])
 4  '[some.hiccup.library :refer [html]])
 5
 6(import
 7  'java.time.format.DateTimeFormatter
 8  'java.time.LocalDateTime)
 9
10(def date-time-formatter (DateTimeFormatter/ofPattern "YYYY-MM-DD HH:mm:ss"))
11(def seconds-formatter (DateTimeFormatter/ofPattern "ss"))
12
13(defn handle [ring-request]
14  (->sse-response ring-request
15    {on-open
16     (fn [sse]
17       (let [now (LocalDateTime/now)
18             current-time (LocalDateTime/.format now date-time-formatter)
19             seconds (LocalDateTime/.format now seconds-formatter)
20             duration (if (neg? (compare seconds "50"))
21                         "5"
22                         "1")]
23         (d*/patch-elements! sse
24           (html [:div#time {(str "data-on-interval__duration." duration "s")
25                             (d*/sse-get "/endpoint")}
26                   current-time]))))}))
27
28         (d*/close-sse! sse))}))
```

```html
 1using StarFederation.Datastar.DependencyInjection;
 2
 3app.MapGet("/endpoint", async (IDatastarService datastarService) =>
 4{
 5    var currentTime = DateTime.Now.ToString("yyyy-MM-dd hh:mm:ss");
 6    var currentSeconds = DateTime.Now.Second;
 7    var duration = currentSeconds < 50 ? 5 : 1;
 8    await datastarService.PatchElementsAsync($"""
 9        <div id="time" data-on-interval__duration.{duration}s="@get('/endpoint')">
10            {currentTime}
11        </div>
12    """);
13});
```

```html
 1import (
 2    "time"
 3    "github.com/starfederation/datastar-go/datastar"
 4)
 5
 6currentTime := time.Now().Format("2006-01-02 15:04:05")
 7currentSeconds := time.Now().Format("05")
 8duration := 1
 9if currentSeconds < "50" {
10    duration = 5
11}
12
13sse := datastar.NewSSE(w, r)
14sse.PatchElements(fmt.Sprintf(`
15    <div id="time" data-on-interval__duration.%ds="@get('/endpoint')">
16        %s
17    </div>
18`, duration, currentTime))
```

No example found for Java

```html
 1use starfederation\datastar\ServerSentEventGenerator;
 2
 3$currentTime = date('Y-m-d H:i:s');
 4$currentSeconds = date('s');
 5$duration = $currentSeconds < 50 ? 5 : 1;
 6
 7$sse = new ServerSentEventGenerator();
 8$sse->patchElements(`
 9    <div id="time"
10         data-on-interval__duration.${duration}s="@get('/endpoint')"
11    >
12        $currentTime
13    </div>
14`);
```

```html
 1from datastar_py import ServerSentEventGenerator as SSE
 2from datastar_py.sanic import DatastarResponse
 3
 4@app.get("/endpoint")
 5async def endpoint():
 6    current_time = datetime.now()
 7    duration = 5 if current_time.seconds < 50 else 1
 8
 9    return DatastarResponse(SSE.patch_elements(f"""
10        <div id="time" data-on-interval__duration.{duration}s="@get('/endpoint')">
11            {current_time:%Y-%m-%d %H:%M:%S}
12        </div>
13    """))
```

```html
 1datastar = Datastar.new(request:, response:)
 2
 3now = Time.now
 4current_time = now.strftime('%Y-%m-%d %H:%M:%S')
 5current_seconds = now.strftime('%S').to_i
 6duration = current_seconds < 50 ? 5 : 1
 7
 8datastar.patch_elements <<~FRAGMENT
 9    <div id="time"
10         data-on-interval__duration.#{duration}s="@get('/endpoint')"
11    >
12        #{current_time}
13    </div>
14FRAGMENT
```

```html
 1use datastar::prelude::*;
 2use chrono::Local;
 3use async_stream::stream;
 4
 5let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
 6let current_seconds = Local::now().second();
 7let duration = if current_seconds < 50 {
 8    5
 9} else {
10    1
11};
12
13Sse(stream! {
14    yield PatchElements::new(
15        format!(
16            "<div id='time' data-on-interval__duration.{}s='@get(\"/endpoint\")'>{}</div>",
17            duration,
18            current_time,
19        )
20    ).into();
21})
```

```html
 1import { createServer } from "node:http";
 2import { ServerSentEventGenerator } from "../npm/esm/node/serverSentEventGenerator.js";
 3
 4const server = createServer(async (req, res) => {
 5  const currentTime = new Date();
 6  const duration = currentTime.getSeconds > 50 ? 5 : 1;
 7
 8  ServerSentEventGenerator.stream(req, res, (sse) => {
 9    sse.patchElements(`
10       <div id="time"
11          data-on-interval__duration.${duration}s="@get('/endpoint')"
12       >
13         ${currentTime.toISOString()}
14       </div>
15    `);
16  });
17});
```

## Conclusion [#](#conclusion)

Using this approach, we not only end up with a way to poll the backend at regular intervals, but we can also control the rate at which the frontend polls based on whatever criteria our backend requires.