<script>
    import ProgressBar from "./ProgressBar.svelte";

    export let name;
    export let cloud;
    export let token;
    export let all_memory;
    export let inuse_memory;
</script>

<button class="cloud-item rounded-1 m-1">
    <img src="img/cloud/google-drive.png" class="align-self-center" alt="Company Logo" style="height: 1rem; width: 1rem;" />
    <b class="name">{name}</b>

    <div class="progress m-2">
        <ProgressBar {all_memory} {inuse_memory} />
    </div>

    <div class="co-progress row">
        <div class="left-val col text-start"><b>0</b> Gb</div>
        <div class="mid-val col"><b>{inuse_memory}</b> Gb</div>
        <div class="right-val col text-end"><b>{all_memory}</b> Gb</div>
    </div>
</button>

<style>
    .cloud-item  {
        animation: gradient 10s infinite linear;
        transition: 0.2s;
        flex: 0 0 auto;
        overflow: hidden;
        width: 200px;
        border: 2px solid #111;
        background-size: 400%;
        background-image: linear-gradient(120deg, var(--view-color-2), #ffffff, var(--view-color-1));
    }

    .progress {
        background: #ffffffca;
        height: 5px;
    }

    .co-progress {
        font-size: smaller;
    }

    .cloud-item:hover {
        transform: scale(0.95);
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