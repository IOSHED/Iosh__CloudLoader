<script>
    export let all_memory;
    export let inuse_memory;
    let procent_use = inuse_memory / all_memory * 100;
</script>

{#if procent_use < 50}
    <div class="progress-bar bg-success" role="progressbar" style="width: {procent_use}%;" aria-valuenow="{procent_use}" aria-valuemin="0" aria-valuemax="100"></div>
{:else if procent_use < 75}
    <div class="progress-bar bg-warning" role="progressbar" style="width: {procent_use}%;" aria-valuenow="{procent_use}" aria-valuemin="0" aria-valuemax="100"></div>
{:else if procent_use < 100}
    <div class="progress-bar bg-warning" role="progressbar" style="width: {procent_use}%;" aria-valuenow="{procent_use}" aria-valuemin="0" aria-valuemax="100"></div>
{/if}