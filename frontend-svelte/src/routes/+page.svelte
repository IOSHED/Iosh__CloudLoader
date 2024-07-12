<script>
  import { invoke } from "@tauri-apps/api/core";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="container">

  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>

</div>