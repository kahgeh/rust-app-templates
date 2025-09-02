# TodoMVC

Source: https://data-star.dev/examples/todomvc

---

# TodoMVC

Demo

- Learn any backend language
- Learn Datastar
- ???
- Profit

**3** items pending AllPendingCompleted Delete Reset

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <section data-on-load="@get('/examples/todomvc/updates')" id="todomvc">
   <ul id="todo-list">
    <li data-on-dblclick="evt.target === el &amp;&amp; @get('/examples/todomvc/0')" role="button" tabindex="0">
     <input data-on-click__prevent="@post('/examples/todomvc/0/toggle')" data-on-load="el.checked = true" id="todo-checkbox-0" type="checkbox"/>
     <label for="todo-checkbox-0">
      Learn any backend language
     </label>
     <button class="error small" data-on-click="@delete('/examples/todomvc/0')">
      <iconify-icon icon="pixelarticons:close" noobserver="">
      </iconify-icon>
     </button>
    </li>
    <li data-on-dblclick="evt.target === el &amp;&amp; @get('/examples/todomvc/1')" role="button" tabindex="0">
     <input data-on-click__prevent="@post('/examples/todomvc/1/toggle')" data-on-load="el.checked = false" id="todo-checkbox-1" type="checkbox"/>
     <label for="todo-checkbox-1">
      Learn Datastar
     </label>
     <button class="error small" data-on-click="@delete('/examples/todomvc/1')">
      <iconify-icon icon="pixelarticons:close" noobserver="">
      </iconify-icon>
     </button>
    </li>
    <li data-on-dblclick="evt.target === el &amp;&amp; @get('/examples/todomvc/2')" role="button" tabindex="0">
     <input data-on-click__prevent="@post('/examples/todomvc/2/toggle')" data-on-load="el.checked = false" id="todo-checkbox-2" type="checkbox"/>
     <label for="todo-checkbox-2">
      ???
     </label>
     <button class="error small" data-on-click="@delete('/examples/todomvc/2')">
      <iconify-icon icon="pixelarticons:close" noobserver="">
      </iconify-icon>
     </button>
    </li>
    <li data-on-dblclick="evt.target === el &amp;&amp; @get('/examples/todomvc/3')" role="button" tabindex="0">
     <input data-on-click__prevent="@post('/examples/todomvc/3/toggle')" data-on-load="el.checked = false" id="todo-checkbox-3" type="checkbox"/>
     <label for="todo-checkbox-3">
      Profit
     </label>
     <button class="error small" data-on-click="@delete('/examples/todomvc/3')">
      <iconify-icon icon="pixelarticons:close" noobserver="">
      </iconify-icon>
     </button>
    </li>
   </ul>
   <div id="todo-actions">
    <span>
     <strong>
      3
     </strong>
     items pending
    </span>
    <button class="small info" data-on-click="@put('/examples/todomvc/mode/0')">
     All
    </button>
    <button class="small" data-on-click="@put('/examples/todomvc/mode/1')">
     Pending
    </button>
    <button class="small" data-on-click="@put('/examples/todomvc/mode/2')">
     Completed
    </button>
    <button class="error small" data-on-click="@delete('/examples/todomvc/-1')">
     <iconify-icon icon="pixelarticons:trash" noobserver="">
     </iconify-icon>
     Delete
    </button>
    <button class="warning small" data-on-click="@put('/examples/todomvc/reset')">
     <iconify-icon icon="pixelarticons:reload" noobserver="">
     </iconify-icon>
     Reset
    </button>
   </div>
  </section>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

This is a full implementation of TodoMVC using Datastar. It demonstrates complex state management, including adding, editing, deleting, and filtering todos, all handled through server-sent events.

## HTML [#](#html)

```html
 1<section
 2    id="todomvc"
 3    data-on-load="@get('/examples/todomvc/updates')"
 4>
 5    <header id="todo-header">
 6        <input
 7            type="checkbox"
 8            data-on-click__prevent="@post('/examples/todomvc/-1/toggle')"
 9            data-on-load="el.checked = false"
10        />
11        <input
12            id="new-todo"
13            type="text"
14            placeholder="What needs to be done?"
15            data-signals-input
16            data-bind-input
17            data-on-keydown="
18                evt.key === 'Enter' && $input.trim() && @patch('/examples/todomvc/-1') && ($input = '');
19            "
20        />
21    </header>
22    <ul id="todo-list">
23        <!-- Todo items are dynamically rendered here -->
24    </ul>
25    <div id="todo-actions">
26        <span>
27            <strong>0</strong> items pending
28        </span>
29        <button class="small info" data-on-click="@put('/examples/todomvc/mode/0')">
30            All
31        </button>
32        <button class="small" data-on-click="@put('/examples/todomvc/mode/1')">
33            Pending
34        </button>
35        <button class="small" data-on-click="@put('/examples/todomvc/mode/2')">
36            Completed
37        </button>
38        <button class="error small" aria-disabled="true">
39            Delete
40        </button>
41        <button class="warning small" data-on-click="@put('/examples/todomvc/reset')">
42            Reset
43        </button>
44    </div>
45</section>
```