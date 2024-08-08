<script>
    export let name;
</script>

<div class="btn-load m-1 p-1 rounded-1">
    <b>{name}</b>
</div>

<style>
    .btn-load {
        border: 2px solid #111;
        background-size: 400%;
        background-image: linear-gradient(120deg, #E3535D, #ffffff, #2AA5A0);
    }

    .btn-load:hover {
        transform: scale(0.95);
    }
</style>