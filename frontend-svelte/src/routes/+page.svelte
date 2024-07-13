<script>
    import MainButton from "../components/home/MainButton.svelte";
</script>

<div class="container centered-container">
  <div class="btn-container">
      <MainButton icon="bi-plus-lg" label="add repository" />
      <MainButton icon="bi-file-earmark-text-fill" label="documentation" />
  </div>
</div>

<style>
  .centered-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      height: 100vh;
  }
  .btn-container {
      display: flex;
      gap: 20px;
  }
</style>
