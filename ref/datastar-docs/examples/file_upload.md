# File Upload

Source: https://data-star.dev/examples/file_upload

---

# File Upload

Demo

Pick anything less than 1 MiB

 Submit

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <label>
   <p>
    Pick anything less than 1 MiB
   </p>
   <input data-bind-files="" multiple="" type="file"/>
  </label>
  <button class="warning" data-attr-aria-disabled="`${!$files.length}`" data-on-click="$files.length &amp;&amp; @post('/examples/file_upload')">
   Submit
  </button>
  <div hidden="" id="file-upload">
  </div>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

In this example we show how to create a file upload form that will be submitted via fetch.

```html
 1<label>
 2    <p>Pick anything less than 1MB</p>
 3    <input type="file" data-bind-files multiple/>
 4</label>
 5<button
 6    class="warning"
 7    data-on-click="$files.length && @post('/examples/file_upload')"
 8    data-attr-aria-disabled="`${!$files.length}`"
 9>
10    Submit
11</button>
```

We don't need a form because everything is encoded as signals and automatically sent to the server. We `POST` the form to `/examples/file_upload`, since the `input` is using `data-bind` the file will be automatically encoded as base64. For file inputs that have bound signals, the `{signalName}Mimes` and `{signalName}Names` are set automatically as well. All three signals are arrays and files / metainfo will be appended in the order of selection.

### Note [#](#note)

If you try to upload a file that is too large you will get an error message in the console.