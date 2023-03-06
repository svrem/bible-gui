<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let loading = false;
  let verse = {
    bookname: "",
    chapter: "",
    verse: "",
    text: "",
  };

  async function generate() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    loading = true;
    const verse_string: string = await invoke("generate");
    loading = false;

    verse = JSON.parse(verse_string)[0];
  }
</script>

<div>
  <div class="row">
    <button disabled={loading} on:click={generate}> Generate verse </button>
  </div>
  <p>
    <strong>{verse.bookname} {verse.chapter}:{verse.verse}</strong>
    {verse.text}
  </p>
</div>
