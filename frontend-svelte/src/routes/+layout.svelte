<script>
    import Canvas from "$lib/background/Canvas.svelte";

    const colorsBalls = ["#56afba95", "#6fb3e675", "#8bbea15a", "#bd464f77"];
</script>

<div class="m-0 p-0 bg-dark vh-100 d-flex w-auto">
    <Canvas { colorsBalls } quantityBalls=20 speedBalls=80 />

    <div class="overlay">
        <slot></slot>
    </div>

</div>

<style>
    div {
        position: relative;
        overflow: hidden;
    }

    .overlay {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        color: white;
        text-align: center;
    }
</style>

