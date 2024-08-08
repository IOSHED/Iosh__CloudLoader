<script>
    export let icon;
    export let func;
    export let hoverColor = '#E3535D';
</script>

<button class="main-button col m-2 d-flex justify-content-center align-items-center" on:click={func} style="--hover-color: {hoverColor}">
    <i class={`bi ${icon}`}></i>
</button>

<style>
    .main-button {
        width: 100px;
        height: 100px;
        transition: 0.2s;
        animation: border-animate 10s infinite linear, gradient 10s infinite linear;
        clip-path: polygon(25% 0, 100% 0, 100% 25%, 100% 75%, 75% 100%, 25% 100%, 0 100%, 0 25%);
        border: 2px solid #E3535D;
        border-image: linear-gradient(250deg, #E3535D, #ffffff00, #2AA5A0);
        border-image-slice: 1;
        background-size: 350%;
        background: linear-gradient(120deg, #ffffff00, #ffffff00);
    }
    
    .main-button i {
        animation: gradient 10s infinite linear;
        font-size: 4rem;
        background-size: 400%;
        background-image: linear-gradient(120deg, #E3535D, #ffffff, #2AA5A0);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .main-button:hover i {
        background-image: linear-gradient(120deg, #ffffff, #ffffff);
    }

    .main-button:hover {
        transform: scale(0.95);
        border: 0px solid #ffffff00;
        background: linear-gradient(120deg, var(--hover-color), var(--hover-color));
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

    @keyframes border-animate {
        0% {
            clip-path: polygon(0 25%, 25% 0, 75% 0, 100% 0, 100% 25%, 100% 75%, 75% 100%, 25% 100%, 0 100%, 0 75%);
        }
        25% {
            clip-path: polygon(0 25%, 0 0, 75% 0, 100% 0, 100% 25%, 100% 100%, 75% 100%, 25% 100%, 0 100%, 0 75%);
        }
        50% {
            clip-path: polygon(0 25%, 0 0, 25% 0, 75% 0, 100% 25%, 100% 75%, 100% 100%, 75% 100%, 25% 100%, 0 75%);
        }
        75% {
            clip-path: polygon(0 25%, 0 0, 75% 0, 100% 0, 100% 25%, 100% 100%, 75% 100%, 25% 100%, 0 100%, 0 75%);
        }
        100% {
            clip-path: polygon(0 25%, 25% 0, 75% 0, 100% 0, 100% 25%, 100% 75%, 75% 100%, 25% 100%, 0 100%, 0 75%);
        }
    }
    /*  0% {
            clip-path: polygon(0 25%, 25% 0, 100% 0, 100% 0, 100% 75%, 75% 100%, 0 100%, 0 100%);
        }
        25% {
            clip-path: polygon(0 0, 0 0, 100% 0, 100% 0, 100% 100%, 100% 100%, 0 100%, 0 100%);
        }
        50% {
            clip-path: polygon(0 0, 0 0, 75% 0, 100% 25%, 100% 100%, 100% 100%, 25% 100%, 0 75%);
        }
        75% {
            clip-path: polygon(0 0, 0 0, 100% 0, 100% 0, 100% 100%, 100% 100%, 0 100%, 0 100%);
        }
        100% {
            clip-path: polygon(0 25%, 25% 0, 100% 0, 100% 0, 100% 75%, 75% 100%, 0 100%, 0 100%);
        } */

    :global(:root) {
        --hover-color: {hoverColor};
    }
</style>
