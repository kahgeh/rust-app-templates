# DBmon

Source: https://data-star.dev/examples/dbmon

---

# DBmon

Demo

Average render time for entire page: 0s

Mutation Rate %  FPS

|  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- |
| cluster1 | 6 | 1ms | 3ms | 4ms | 4ms | 5ms |
| cluster1slave | 15 | 0s | 0s | 1ms | 1ms | 3ms |
| cluster2 | 13 | 0s | 0s | 3ms | 6ms | 7ms |
| cluster2slave | 15 | 0s | 2ms | 2ms | 2ms | 6ms |
| cluster3 | 7 | 2ms | 3ms | 4ms | 5ms | 6ms |
| cluster3slave | 15 | 0s | 1ms | 2ms | 4ms | 5ms |
| cluster4 | 9 | 0s | 0s | 2ms | 4ms | 7ms |
| cluster4slave | 8 | 0s | 3ms | 3ms | 4ms | 5ms |
| cluster5 | 13 | 2ms | 4ms | 5ms | 6ms | 6ms |
| cluster5slave | 8 | 1ms | 2ms | 4ms | 7ms | 8ms |
| cluster6 | 11 | 0s | 3ms | 3ms | 4ms | 4ms |
| cluster6slave | 15 | 1ms | 2ms | 3ms | 3ms | 3ms |

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <div data-on-load="@get('/examples/dbmon/updates')" data-signals-_editing__ifmissing="false" id="demo">
   <p>
    Average render time for entire page: 0s
   </p>
   <div role="group">
    <label>
     Mutation Rate %
     <input data-attr-data-bind-_mutation-rate="!$_editing" data-attr-data-bind-mutation-rate="$_editing" data-on-blur="@put('/examples/dbmon/inputs'); $_editing = false" data-on-focus="$_editing = true" max="100" min="0" type="number" value="20"/>
    </label>
    <label>
     FPS
     <input data-attr-data-bind-_fps="!$_editing" data-attr-data-bind-fps="$_editing" data-on-blur="@put('/examples/dbmon/inputs'); $_editing = false" data-on-focus="$_editing = true" max="144" min="1" type="number" value="60"/>
    </label>
   </div>
   <table style="table-layout: fixed; width: 100%; word-break: break-all">
    <tbody>
     <tr>
      <td>
       cluster1
      </td>
      <td class="success" style="background-color: var(--_active-color)">
       6
      </td>
      <td aria-description="SELECT blah from something">
       1ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       3ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       4ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       5ms
      </td>
     </tr>
     <tr>
      <td>
       cluster1slave
      </td>
      <td class="error" style="background-color: var(--_active-color)">
       15
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="vacuum">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       1ms
      </td>
      <td aria-description="SELECT blah from something">
       1ms
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
     </tr>
     <tr>
      <td>
       cluster2
      </td>
      <td class="warning" style="background-color: var(--_active-color)">
       13
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="vacuum">
       0s
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       6ms
      </td>
      <td aria-description="vacuum">
       7ms
      </td>
     </tr>
     <tr>
      <td>
       cluster2slave
      </td>
      <td class="error" style="background-color: var(--_active-color)">
       15
      </td>
      <td aria-description="vacuum">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       6ms
      </td>
     </tr>
     <tr>
      <td>
       cluster3
      </td>
      <td class="success" style="background-color: var(--_active-color)">
       7
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       5ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       6ms
      </td>
     </tr>
     <tr>
      <td>
       cluster3slave
      </td>
      <td class="error" style="background-color: var(--_active-color)">
       15
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       1ms
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="vacuum">
       4ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       5ms
      </td>
     </tr>
     <tr>
      <td>
       cluster4
      </td>
      <td class="success" style="background-color: var(--_active-color)">
       9
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="vacuum">
       4ms
      </td>
      <td aria-description="SELECT blah from something">
       7ms
      </td>
     </tr>
     <tr>
      <td>
       cluster4slave
      </td>
      <td class="success" style="background-color: var(--_active-color)">
       8
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
      <td aria-description="SELECT blah from something">
       5ms
      </td>
     </tr>
     <tr>
      <td>
       cluster5
      </td>
      <td class="warning" style="background-color: var(--_active-color)">
       13
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
      <td aria-description="SELECT blah from something">
       5ms
      </td>
      <td aria-description="vacuum">
       6ms
      </td>
      <td aria-description="SELECT blah from something">
       6ms
      </td>
     </tr>
     <tr>
      <td>
       cluster5slave
      </td>
      <td class="success" style="background-color: var(--_active-color)">
       8
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       1ms
      </td>
      <td aria-description="vacuum">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       7ms
      </td>
      <td aria-description="SELECT blah from something">
       8ms
      </td>
     </tr>
     <tr>
      <td>
       cluster6
      </td>
      <td class="warning" style="background-color: var(--_active-color)">
       11
      </td>
      <td aria-description="SELECT blah from something">
       0s
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
      <td aria-description="&lt;IDLE&gt; in transaction">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
      <td aria-description="SELECT blah from something">
       4ms
      </td>
     </tr>
     <tr>
      <td>
       cluster6slave
      </td>
      <td class="error" style="background-color: var(--_active-color)">
       15
      </td>
      <td aria-description="vacuum">
       1ms
      </td>
      <td aria-description="SELECT blah from something">
       2ms
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
      <td aria-description="SELECT blah from something">
       3ms
      </td>
     </tr>
    </tbody>
   </table>
  </div>
 </div>
</fieldset>
```

## HTML [#](#html)

```html
 1<div
 2    id="demo"
 3    data-on-load="@get('/examples/dbmon/updates')"
 4    data-signals-_editing__ifmissing="false"
 5>
 6    <p>
 7        Average render time for entire page: { renderTime }
 8    </p>
 9    <div role="group">
10        <label>
11            Mutation Rate %
12            <input
13                type="number"
14                min="0"
15                max="100"
16                value="20"
17                data-on-focus="$_editing = true"
18                data-on-blur="@put('/examples/dbmon/inputs'); $_editing = false"
19                data-attr-data-bind-mutation-rate="$_editing"
20                data-attr-data-bind-_mutation-rate="!$_editing"
21            />
22        </label>
23        <label>
24            FPS
25            <input
26                type="number"
27                min="1"
28                max="144"
29                value="60"
30                data-on-focus="$_editing = true"
31                data-on-blur="@put('/examples/dbmon/inputs'); $_editing = false"
32                data-attr-data-bind-fps="$_editing"
33                data-attr-data-bind-_fps="!$_editing"
34            />
35        </label>
36    </div>
37    <table style="table-layout: fixed; width: 100%; word-break: break-all">
38        <tbody>
39            <!-- Dynamic rows generated by server -->
40            <tr>
41                <td>cluster1</td>
42                <td style="background-color: var(--_active-color)" class="success">
43                    8
44                </td>
45                <td aria-description="SELECT blah from something">
46                    12ms
47                </td>
48                <!-- More query cells... -->
49            </tr>
50            <!-- More database rows... -->
51        </tbody>
52    </table>
53</div>
```

## Explanation [#](#explanation)

Per a conversation on the discord server there was a desire to port an old React Conf talk, [DBMon](https://conf2015.reactjs.org/schedule.html#hype), to Datastar.

The logic is 1:1 but all done on the backend, and since it's Go, it's an interesting comparison to the SPA based approach. We've limited purely since the site is run on a free tier server and don't want to be a bad user. If you run the site from source you can easily 10x the rows without major issues.

### Note [#](#note)

If you open your Network tab in DevTools we are leveraging ZSTD compression so the data rate is relatively low for the contents.