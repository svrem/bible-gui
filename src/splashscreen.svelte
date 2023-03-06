<script>
    import { invoke } from "@tauri-apps/api/tauri";

    const sentences = [
        "Connecting to the vatican...",
        "Waiting for gods permission...",
        "Playing a potje among us...",
        "Got killed, fuck...",
        "I think we may be almost done...",
        "I hope anyway...",
        "Oh shit those were all the messages...",
        "Loading...",
    ];
    let index = 0;
    let sentence = sentences[index];

    setInterval(() => {
        index++;
        if (index >= sentences.length) {
            invoke("close_splashscreen");
        }

        sentence = sentences[index];
    }, 2000);
</script>

<div class="wrapper">
    <h1 class="curr-sentence">{sentence}</h1>
</div>

<style>
    @keyframes background {
        from {
            background-position: 0% 0%;
        }
        to {
            background-position: 100% 0%;
        }
    }
    .wrapper {
        animation: background 10s infinite linear;
        /* filter: blur(8px); */
        background-image: url("/cross.webp");
        background-repeat: repeat-x;
        background-size: contain;
        color: #f6f6f6;
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        background-color: #2f2f2f;
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
        margin: 0;
    }

    .curr-sentence {
        background-color: rgba(0, 0, 0, 0.5);
        padding-top: 30px;
        padding-bottom: 30px;
        backdrop-filter: blur(10px);
        width: 100%;
    }
</style>
