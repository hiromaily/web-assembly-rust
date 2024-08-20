<script lang="ts">
  import * as wasm from "gen-hyimage-wasm";
  import { onMount } from "svelte";
  import { Slider } from "svelte-awesome-slider";

  // Bind input values to variables
  let red = 240;
  let green = 240;
  let blue = 240;
  let alpha = 255;

  const validateRGB = (value: number) =>
    Math.min(255, Math.max(0, isNaN(value) ? 0 : value));
  // const validateRGB = (value: number) => {
  //   if (isNaN(value)) return 0;
  //   if (value < 0) return 0;
  //   if (value > 255) return 255;
  //   return value;
  // };

  // Reactive statement to automatically update validated values
  $: red = validateRGB(red);
  $: green = validateRGB(green);
  $: blue = validateRGB(blue);
  $: alpha = validateRGB(alpha);

  const generate = () => {
    console.log("generated click");

    // const r = validateRGB(parseInt(document.getElementById("red").value));
    // const g = validateRGB(parseInt(document.getElementById("green").value));
    // const b = validateRGB(parseInt(document.getElementById("blue").value));
    // const a = validateRGB(parseInt(document.getElementById("alpha").value));
    const imageBytes = wasm.generate_image(red, green, blue, alpha);

    // Convert the image bytes to a Blob and display it as an image
    const blob = new Blob([imageBytes], { type: "image/png" });
    const url = URL.createObjectURL(blob);

    // Set image
    document.getElementById("generated-image").src = url;
  };

  onMount(async () => {
    generate();
  });
</script>

<h2>Wasm Image Generator</h2>

<div class="color-control">
  <label for="red">R:</label>
  <input type="number" bind:value={red} min="0" max="255" on:input={generate} />
  <Slider
    min={0}
    max={255}
    bind:value={red}
    step={5}
    name="red"
    --track-background="rgba(255,255,255,0.6)"
    --thumb-background="#FFF"
    --track-width="40%"
    --track-height="20px"
    on:input={generate}
  />
</div>

<div class="color-control">
  <label for="green">G:</label>
  <input
    type="number"
    bind:value={green}
    min="0"
    max="255"
    on:input={generate}
  />
  <Slider
    min={0}
    max={255}
    bind:value={green}
    step={5}
    name="red"
    --track-background="rgba(255,255,255,0.6)"
    --thumb-background="#FFF"
    --track-width="40%"
    --track-height="20px"
    on:input={generate}
  />
</div>

<div class="color-control">
  <label for="blue">B:</label>
  <input
    type="number"
    bind:value={blue}
    min="0"
    max="255"
    on:input={generate}
  />
  <Slider
    min={0}
    max={255}
    bind:value={blue}
    step={5}
    name="red"
    --track-background="rgba(255,255,255,0.6)"
    --thumb-background="#FFF"
    --track-width="40%"
    --track-height="20px"
    on:input={generate}
  />
</div>

<div class="color-control">
  <label for="alpha">A:</label>
  <input
    type="number"
    bind:value={alpha}
    min="0"
    max="255"
    on:input={generate}
  />
  <Slider
    min={0}
    max={255}
    bind:value={alpha}
    step={5}
    name="red"
    --track-background="rgba(255,255,255,0.6)"
    --thumb-background="#FFF"
    --track-width="40%"
    --track-height="20px"
    on:input={generate}
  />
</div>

<div>
  <button on:click={generate}> Generate Image </button>

  <br />
  <br />
  <img id="generated-image" alt="Generated Image" />
</div>

<style>
  .color-control {
    display: flex;
    align-items: center;
    gap: 10px; /* Space between elements */
  }
</style>
