# Bad Apple

Source: https://data-star.dev/examples/bad_apple

---

# Bad Apple

Demo

Demo code:

```html
<fieldset class="demo">
 <legend>
  Demo
 </legend>
 <div>
  <label data-on-load="@get('/examples/bad_apple/updates')" data-signals="{_percentage: 0, _contents: 'bad apple frames go here'}">
   <span data-text="`Percentage: ${$_percentage.toFixed(2)}%`">
   </span>
   <input data-attr-value="$_percentage" disabled="" max="100" min="0" step="0.01" style="cursor: default" type="range"/>
  </label>
  <pre data-text="$_contents" style="line-height: 100%"></pre>
 </div>
</fieldset>
```

## Explanation [#](#explanation)

Per a conversation on the [htmx meme discord channel](https://discordapp.com/channels/725789699527933952/996832027083026563/1276380165613813894) there was an offhand remark about adding the [Bad Apple Music video](https://www.youtube.com/watch?v=FtutLA63Cp8) as a benchmark. Thought it'd be fun to do so. We take the [already converted](https://github.com/trung-kieen/bad-apple-ascii) frames of video and turn them into a ZSTD compressed Gob file that's embedded in the server binary. This makes the whole animation about 1.9MB. We then stream the frames to the client and update the contents of a pre tag with the frames. The percentage is updated with the current frame number.

```html
 1<label
 2    data-signals="{_percentage: 0, _contents: 'bad apple frames go here'}"
 3    data-on-load="@get('/examples/bad_apple/updates')"
 4>
 5    <span data-text="`Percentage: ${$_percentage.toFixed(2)}%`"></span>
 6    <input
 7        type="range"
 8        min="0"
 9        max="100"
10        step="0.01"
11        disabled
12        style="cursor: default"
13        data-attr-value="$_percentage"
14    />
15</label>
16<pre style="line-height: 100%" data-text="$_contents"></pre>
```

This is using Datastar's ability to patch signals directly. ***No need to generate HTML elements, as the contents are already bound to existing elements.*** We could also stream down the raster frames using base64 encoded images and update the src of an image tag. Either way works, you would just have to use `data-attr-src` on an image tag. Open your browser dev tool's inspector tab for the contents of the `pre` tag. You'll see the frames being updated in real-time (in this case 30fps).