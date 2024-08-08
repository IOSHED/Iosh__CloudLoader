<script>
    import MainButton from '../components/main/MainButton.svelte';
    import { page } from '$app/stores';

    function redirectToHome() {
      window.location.href = '/';
    }
</script>


<div class="d-flex flex-column justify-content-center align-items-center h-100">
    <div class="info border border-danger p-5 text-danger text-center row">
        <h1>{$page.status}</h1>
        <h2>{$page.error.message}</h2>
        <MainButton icon="bi-house" func={redirectToHome} label="home" />
    </div>
</div>
