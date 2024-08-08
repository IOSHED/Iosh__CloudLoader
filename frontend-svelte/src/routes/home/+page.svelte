<script>
    import RepositoryItem from "../../components/home/RepositoryItem.svelte";
    import BlackLine from "../../components/home/BlackLine.svelte";
    import ProgressBar from "../../components/home/ProgressBar.svelte";
    import CloudItem from "../../components/home/CloudItem.svelte";
    import BtnAddRepo from "../../components/home/BtnAddRepo.svelte";
    import BtnLoad from "../../components/home/BtnLoad.svelte";
</script>

<div class="row h-100 w-100 m-0 p-5">
    <aside class="repositories col-3 rounded-2 p-3">
        <div class="line m-0 p-0 text-start"><b>Repositories</b></div>
        <BlackLine />
        <div class="scroll p-1 h-100">
            <BtnAddRepo />
            <!-- {#each repoitorys as repo}
                <RepositoryItem
                    name={repo.name} 
                    description={repo.description}
                    is_selected={repo.is_selected} 
                    all_memory={repo.all_memory}
                    inuse_memory={repo.inuse_memory}
                    view_color_1={repo.view_color_1}
                    view_color_2={repo.view_color_2}
                    view_icon={repo.view_icon}
                />
            {/each} -->
            <RepositoryItem
                name="I Love Russia" 
                description="My photos when I have been in Moscow, St.Petersburg and other city of Russia."
                is_selected=1
                all_memory=128
                inuse_memory=45
                view_color_1="#E3535D"
                view_color_2="#2AA5A0"
                view_icon="heart-fill"
            />
            <RepositoryItem
                name="New Name (1)" 
                description=""
                is_selected=0
                all_memory=8
                inuse_memory=0
                view_color_1="#666666"
                view_color_2="#aaff23"
                view_icon="arrow-down"
            />
            <RepositoryItem
                name="New Name (2)" 
                description=""
                is_selected=0
                all_memory=4
                inuse_memory=0
                view_color_1="#ffdd12"
                view_color_2="#aaff23"
                view_icon="arrow-down"
            />
        </div>
    </aside>
    <section class="content col rounded-2 p-2 w-100 h-100">
        <button class="setting-repo m-0 p-0"><i class="bi bi-gear"></i></button>
        <div class="name display-1 text-center col">
            <i class="bi bi-heart-fill m-1 col"></i>
            <b>I Love Russia</b>
        </div>
        <div class="description text-start">My photos when I have been in Moscow, St.Petersburg and other city of Russia.</div>
        <div class="memory-bar d-flex">
            <div><b>Memory: </b></div>
            <div class="progress m-2 w-100 rounded-1">
                <ProgressBar all_memory=128 inuse_memory=45 />
            </div>
        </div>
        <BlackLine />
        <div class="horizontal-scroll">
            <CloudItem 
                name="Google Drive 1"
                cloud="Google Drive"
                token=Nan
                all_memory=64
                inuse_memory=22
            />
        </div>
        <BlackLine />
        
        <div class="btns d-flex align-items-center justify-content-end w-100">
            <BtnLoad name="Download" />
            <BtnLoad name="Upload" />    
        </div>

        <table class="table table-striped table-hover">
            <thead>
              <tr>
                <th scope="col">#</th>
                <th scope="col">File/Folder</th>
                <th scope="col">Uploaded</th>
                <th scope="col">Body</th>
              </tr>
            </thead>
            <tbody class="table-group-divider">
              <tr>
                <th scope="row">1</th>
                <td>Moscow</td>
                <td>21.04.2021</td>
                <td>Photo 1.png, Photo 2.png ...</td>
              </tr>
              <tr>
                <th scope="row">2</th>
                <td>St.Peterburg</td>
                <td>17.12.2022</td>
                <td>bdkfose239do.png, sf23sjs.png ...</td>
              </tr>
              <tr>
                <th scope="row">3</th>
                <td>preview.md</td>
                <td>06.08.2024</td>
                <td># My trips in Russi...</td>
              </tr>
            </tbody>
          </table>
    </section>
</div>

<style>
    .setting-repo {
        background: #ffffff00;
        border-width: 0;
        transition: 0.5s;
    }

    .setting-repo:hover {
        transform: scale(1.1);
    }

    .repositories {
        background: #ffffff99;
    }

    .content {
        background: #ffffff99;
        margin-left: 50px;
    }

    .scroll {
        max-height: calc(100vh - 10rem);
        overflow-y: scroll;
        padding: 1rem;
        scrollbar-width: thin;
        scrollbar-color: #000000 transparent;
    }

    .horizontal-scroll {
        max-width: 100%;
        min-width: 100%;
        /* display: flex; */
        /* white-space: nowrap; */
        max-height: 200px;
        /* overflow-x: scroll; */
        padding: 1rem;
        overflow-x: auto;	
        scrollbar-width: thin;
        scrollbar-color: #000000 transparent;
        /* background-color: #fff; */
        /* width: 100%; */
        /* min-height: 200px; */
    }

    .progress {
        background: #ffffffca;
        height: 12px;
    }
</style>