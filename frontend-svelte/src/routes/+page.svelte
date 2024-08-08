
<script>
    import MainButton from "../components/main/MainButton.svelte";
    import CompanyView from "../components/main/CompanyView.svelte";

    const companyLogo = "icons/icon.png";
    const companyName = "CLOUD LOADER";

    async function redirectToAddRepository() {
        window.location.href = '/home';
    }

    async function redirectToShowDocs() {
        window.location.href = '/docs';
    }
</script>


<div class="welcom-page d-block m-auto h-100">
    <div class="d-flex flex-column justify-content-center align-items-center h-100">
        <CompanyView {companyLogo} {companyName} />
        <div class="block-buttons row">
            <MainButton icon="bi-plus-lg" label="add repository" func={redirectToAddRepository} hoverColor="#E3535D" />
            <MainButton icon="bi-file-earmark-text-fill" label="documentation" func={redirectToShowDocs} hoverColor="#2AA5A0" />
        </div>
    </div>
</div>