
<button class="btn-add-repo rounded-1">
    <i class="bi bi-plus-circle-fill m-1"></i>
    <b>Add new repository</b>
</button>

<style>
    .btn-add-repo {
        animation: gradient 10s infinite linear;
        transition: 0.2s;
        width: 100%;
        height: 100px;
        border: 2px solid #111111;
        display: flex;
        text-align: center;
        justify-content: center;
        align-items: center;
        background-size: 400%;
        background-image: linear-gradient(120deg, #E3535D, #ffffff, #2AA5A0);
    }

    .btn-add-repo:hover {
        transform: scale(0.95);
        background-size: 200%;
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