<script lang="ts">
  import * as wasm from "gen-hyimage-wasm";
  import { onMount } from "svelte";

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
</script>

<h2>Wasm Image Generator</h2>
<div>
  <label for="red">R:</label>
  <input type="number" bind:value={red} min="0" max="255" />
  <label for="green">G:</label>
  <input type="number" bind:value={green} min="0" max="255" />
  <label for="blue">B:</label>
  <input type="number" bind:value={blue} min="0" max="255" />
  <label for="alpha">A:</label>
  <input type="number" bind:value={alpha} min="0" max="255" />

  <button on:click={generate}> Generate Image </button>

  <br />
  <br />
  <img id="generated-image" alt="Generated Image" />
</div>
