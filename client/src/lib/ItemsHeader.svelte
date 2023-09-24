<script>
    import { createEventDispatcher } from 'svelte'
    const dispatch = createEventDispatcher()

    function searchLater() {
        dispatch('searchLater', searchText);
    }

    export let title;
    export let showCreate;
    export let refresh;
    let searchText;
    
</script>


<div class="row bg-light border-bottom">
    <div class="col-sm-6 h2">{title}</div>
    <div class="col-sm-6">
        
    </div>
</div>
<br>
<div class="d-flex justify-content-center gap-2">
    <input class="form-control" 
            type="text" placeholder="Filter..."
            bind:value={searchText}
            on:keyup={searchLater} />  

    <button class="btn btn-light " 
        on:click={refresh}>
        <i class="bi-repeat"></i>          
    </button>

    <div class=""></div>   
    
    <button
        class="btn btn-success  btn-width" on:click={showCreate}>
        Create
    </button>
</div>


<br>
