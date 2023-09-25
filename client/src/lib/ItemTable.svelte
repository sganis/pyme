<script>
    import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher()

    export let title;
    export let showCreate;
    export let refresh;
    export let table;
    export let sortCol;
    export let sortDesc;
    export let items;

    let searchText;

    const searchLater = () => {
        dispatch('searchLater', searchText);
    }
    const sort = (col) => {
        dispatch('sort', col);
    }
    const showModify = (o) => {
        dispatch('showModify', o);
    }
    const showRemove = (o) => {
        dispatch('showRemove', o);
    }
    
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


<div class="row">
    <div class="table-responsive">
        <table class="table table-hover table-sm">
            <thead class="table-success">
                <tr>
                    {#each table.columns as col, i}
                    <th on:click={()=>sort(col)} role="button" class="text-nowrap">
                        {table.header[i]}
                        <i class:bi-sort-up-alt="{sortCol == col && !sortDesc}"/>
                        <i class:bi-sort-down="{sortCol == col && sortDesc}"/>
                        <i class:bi-app="{sortCol !== col}"/>  
                    </th>
                    {/each}
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
            {#each items as o, i}
            <tr>
                {#each table.columns as col}
                <td>{ o[col] || ""}</td>
                {/each}
                <td class="text-nowrap text-end">
                    <button class="btn btn-light btn-sm btn-width-sm" type="button" 
                        on:click={() => showModify(o)}>
                        <i class="bi-pencil"/>
                    </button>
                    <button class="btn btn-light btn-sm btn-width-sm" type="button" 
                        on:click={() => showRemove(o)}>
                        <i class="bi-trash3"/>
                    </button>
                </td>            
            </tr>    
            {/each} 
            </tbody>
        </table>        
    </div>
</div>

<div>Total: {items.length}</div>


<style>
    .btn-width-sm {
        width: 32px;
    }
    a {
        text-decoration: none;
    }
    .bi-app {
        content: "\2122";
        color: transparent !important;
    }
</style>