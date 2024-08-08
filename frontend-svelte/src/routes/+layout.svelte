
<div class="overlay vh-100 vw-100">
    <slot></slot>
</div>

<style>
    .overlay {
        background: linear-gradient(80deg, #2AA5A0, #ffffff, #E3535D);
        animation: gradient 10s infinite linear;
        background-size: 300%;
    }

    @keyframes gradient {
        0% {
            background-position: 80% 0%;
        }
        50% {
            background-position: 20% 100%;
        }
        100% {
            background-position: 80% 0%;
        }
    }
</style>