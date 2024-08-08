<script>
  import { onMount } from 'svelte';
  import { tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  const companyLogo = 'icons/icon.png';
  const companyName = 'Cloud Loader';
  const companyEmail = 'ioshcloudloader@gmail.com';

  let text = "...";

  async function getDocumetationText() {
    await invoke('get_reading_docs').then((message) => text = message);
  }

  function redirectToHome() {
    window.location.href = '/';
  }

  function sendEmail() {
    window.location.href = `mailto:${companyEmail}`;
  }

  function openSupport() {
    window.location.href = '/support';
  }

  onMount(async () => {
    await getDocumetationText();

    window.addEventListener('scroll', async (event) => {
      if (window.innerHeight + window.scrollY >= document.body.offsetHeight) {
        await tick();
      }
    });
  });
</script>

<div class="container py-4">
  <div class="header d-flex align-items-center justify-content-center mb-4">
    <button class="btn p-0 border-0 bg-transparent" on:click={redirectToHome} aria-label="Go to home">
      <img src={companyLogo} alt="Company Logo" class="me-2" style="height: 3rem; width: 3rem;" />
    </button>
    <h1 class="m-0">Documentation</h1>
  </div>

  <div class="scrollable">
    <p>{text}</p>
  </div>

  <div class="footer d-flex justify-content-between align-items-center border-top pt-3 mt-4 text-center">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="bg-transparent text-white" on:click={sendEmail} aria-label="Send email">
      <div class="text-center p-0 border-0">
        Our mail:
      </div>
      <div class="text-primary">
        {companyEmail}
      </div>
    </div>
    
    <button class="btn p-0 text-primary border-0 bg-transparent d-flex align-items-center text-center" on:click={redirectToHome} aria-label="Go to home">
      <img src={companyLogo} alt="Company Logo" class="me-2" style="height: 3rem; width: 3rem;" />
      <h2>{companyName}</h2>
    </button>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="bg-transparent text-white" on:click={openSupport} aria-label="Send email">
      <div class="text-center p-0 border-0">
        Our chat:
      </div>
      <div class="text-primary">
        User Support
      </div>
    </div>
  </div>
</div>

<style>
  .scrollable {
    max-height: calc(100vh - 7rem);
    overflow-y: auto;
    padding: 1rem;
    scrollbar-width: thin;
    scrollbar-color: #ccc transparent;
  }

  .scrollable::-webkit-scrollbar {
    width: 8px;
  }

  .scrollable::-webkit-scrollbar-track {
    background: transparent;
  }

  .scrollable::-webkit-scrollbar-thumb {
    background-color: #ccc;
    border-radius: 10px;
  }
</style>
