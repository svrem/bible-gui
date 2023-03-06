<script lang="ts">
  import Generate from "./lib/Generate.svelte";
</script>

<main class="container">
  <div class="row">
    <img src="/cross.webp" class="god" alt="God" />
  </div>

  <h1>Welcome to the most divine app in existence!</h1>
  <p>Click on the button to generate a random bible verse.</p>

  <div class="row">
    <Generate />
  </div>
</main>

<style>
  .god {
    transition: all 0.5s ease;
  }
  .god:hover {
    filter: drop-shadow(0 0 2em white);
  }
</style>
