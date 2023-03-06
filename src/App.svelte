<script lang="ts">
  import Generate from "./lib/Generate.svelte";

  let loading = false;
</script>

<main class="container">
  <div class="row">
    <img
      src="/cross.webp"
      class={"god " + (loading ? "god-animate" : "")}
      alt="God"
      id="god"
    />
  </div>

  <h1>Welcome to the most divine app in existence!</h1>
  <p>Click on the button to generate a random bible verse.</p>

  <div class="row">
    <Generate bind:loading />
  </div>
</main>

<style>
  .god {
    transition: all 0.75s ease;
  }
  .god:hover {
    filter: drop-shadow(0 0 2em white);
  }

  @keyframes pulse {
    from,
    to {
      filter: drop-shadow(0 0 -0.1em white);
    }

    50% {
      filter: drop-shadow(0 0 2em white);
    }
  }

  .god-animate {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>
