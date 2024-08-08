
<script>
    export let companyLogo;
    export let companyName;
</script>

<div class="company-view row m-1">
    <img src={companyLogo} class="align-self-center" alt="Company Logo" style="height: 5.3rem; width: 5.3rem;" />
    <h1 class="company-name col">{companyName}</h1>
</div>

<style>
    .company-name {
        animation: text-gradient 10s infinite linear;
        font-weight: 900;
        font-family: Roboto, Arial, sans-serif;
        font-size: 10vmax;
        background-size: 150%;
        background-image: linear-gradient(90deg, #E3535D, #ffffff, #2AA5A0);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    @keyframes text-gradient {
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