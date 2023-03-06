<script lang="ts">
  export let loading;

  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  let sentences = [
    "Loading internet...",
    "Looking through browser history...",
    "God did not approve...",
    "Waiting...",
    "This should only take a moment...",
    "Thou shall not pass...",
    "God's wrath is upon you...",
    "Pray for mercy...",
    "The gods have spoken...",
    "Thy fate is sealed...",
    "May the gods be with you...",
    "The gods have favored you...",
    "Divine intervention is needed...",
    "Thy request has been denied...",
    "The gods have granted thy request...",
    "May the gods have mercy on your soul...",
    "The gods smile upon you...",
    "The gods frown upon you...",
    "Thy offering has been accepted...",
    "The gods demand sacrifice...",
    "Thou art unworthy...",
    "The gods have judged you...",
    "Thy sins have been forgiven...",
    "The gods have blessed you...",
    "Thy faith has been rewarded...",
    "The gods are watching...",
    "Thy destiny awaits...",
    "May the gods guide your path...",
    "The gods have ordained it...",
    "Thou shall obey the gods...",
    "The gods are pleased...",
    "The gods are displeased...",
    "Thou art blessed...",
    "Thou art cursed...",
    "Thy prayers have been answered...",
    "The gods are merciful...",
    "The gods are vengeful...",
    "Please hold on...",
    "Searching for network...",
    "Connecting to server...",
    "One moment please...",
    "Loading data...",
    "Analyzing information...",
    "Calculating results...",
    "Processing request...",
    "Preparing to execute...",
    "Performing security check...",
    "Checking system status...",
    "Verifying credentials...",
    "Syncing data...",
    "Optimizing performance...",
    "Generating report...",
    "Compiling code...",
    "Encrypting data...",
    "Decrypting data...",
    "Backing up files...",
    "Restoring files...",
    "Scanning for viruses...",
    "Updating software...",
    "Installing updates...",
    "Checking for updates...",
    "Logging in...",
    "Logging out...",
    "Saving changes...",
    "Printing document...",
    "Sending message...",
    "Receiving message...",
    "Deleting files...",
    "Copying files...",
    "Moving files...",
    "Renaming files...",
    "Creating new folder...",
    "Deleting folder...",
    "Uploading file...",
    "Downloading file...",
    "Encrypting message...",
    "Decrypting message...",
    "Sending email...",
    "Receiving email...",
    "Checking email...",
    "Forwarding email...",
    "Replying to email...",
    "Archiving email...",
    "Deleting email...",
  ];

  let verse = {
    bookname: "",
    chapter: "",
    verse: "",
    text: "",
  };

  listen("generate", async (_) => {
    await generate();
  });

  async function generate() {
    if (loading) return;

    loading = true;
    verse = {
      bookname: "",
      chapter: "",
      verse: "",
      text: "",
    };
    const verse_request = invoke("generate") as Promise<string>;

    const random_order = sentences.sort(() => 0.5 - Math.random());
    const random_sentences = random_order.slice(0, 7);

    for (let i = 0; i < random_sentences.length; i++) {
      const sentence = random_sentences[i];

      verse.text = sentence;

      await new Promise((resolve) => setTimeout(resolve, sentence.length * 75));
    }

    const verse_string: string = await verse_request;

    verse = JSON.parse(verse_string)[0];
    loading = false;
  }
</script>

<div>
  <div class="row">
    <button disabled={loading} class="generate" on:click={generate}>
      Generate verse
    </button>
  </div>
  <p>
    <strong
      >{verse.bookname}
      {verse.chapter}{verse.bookname ? ":" : ""}{verse.verse}</strong
    >
    {verse.text}
  </p>
</div>

<style>
  .generate:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>
