
<script>
    import ProgressBar from "./ProgressBar.svelte";

    export let name;
    export let description;
    export let is_selected;
    export let all_memory;
    export let inuse_memory;
    export let view_color_1;
    export let view_color_2;
    export let view_icon;

    let procent_use = inuse_memory / all_memory * 100;
</script>

<button 
    class="repo rounded-1 my-1"
    style="
        --view-color-1: {view_color_1}; 
        --view-color-2: {view_color_2}; 
        --color-border: {is_selected == true ? `#ffffff` : `#111111`};
        --scale: {is_selected == true ? `0.95` : `1`};
    ">
    <i class="bi bi-{view_icon} m-1"></i>
    <b class="name">{name}</b>
    <p class="desription text-start">{description}</p>

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
    .repo {
        animation: gradient 10s infinite linear;
        transition: 0.2s;
        transform: scale(var(--scale));
        overflow: hidden;
        width: 100%;
        border: 2px solid var(--color-border);
        background-size: 400%;
        background-image: linear-gradient(120deg, var(--view-color-2), #ffffff, var(--view-color-1));
    }

    .repo i {
        font-size: 1rem;
    }

    .progress {
        background: #ffffffca;
        height: 5px;
    }

    .co-progress {
        font-size: smaller;
    }

    .repo:hover {
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