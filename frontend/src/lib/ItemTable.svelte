<script>
    import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher()

    export let title;
    export let table;
    export let sortCol;
    export let sortDesc;
    export let items;
    export let offset;
    export let limit;
    export let total;
    export let showToolbar;

    $:page = offset/limit + 1;
    $:total_pages = Math.ceil(total/limit);

    let searchText;
    
    const searchLater = () => {
        dispatch('searchLater', searchText);
    }
    const sort = (col) => {
        dispatch('sort', col);
    }
    const refresh = () => {
        dispatch('refresh');
    }
    const showCreate = () => {
        dispatch('showCreate');
    }
    const showModify = (o) => {
        dispatch('showModify', o);
    }
    const goToPage = (page) => {
        dispatch('goToPage', page);
    }
    
</script>


<div class="row bg-light border-bottom">
    <div class="col-sm-6 h2">{title}</div>
    <div class="col-sm-6">
        
    </div>
</div>
<br>

{#if showToolbar}
<div class="d-flex justify-content-center gap-2">
    <input class="form-control" 
            type="text" placeholder="Search..."
            bind:value={searchText}
            on:keyup={searchLater} />  

    <button class="btn btn-light " 
        on:click={refresh}>
        <i class="bi-repeat"></i>          
    </button>

    <div class=""></div>   
    
    <button
        class="btn btn-success  btn-width" on:click={showCreate}>
        New
    </button>
</div>
{/if}



<div class="row mt-3">
    <div class="table-responsive">
        <table class="table table-hover table-sm">
            <thead class="table-warning">
                <tr>
                    {#each table.columns as col, i}
                    <th on:click={()=>sort(col)} role="button" class="text-nowrap">
                        {table.header[i]}
                        <i class:bi-sort-up-alt="{sortCol == col && !sortDesc}"/>
                        <i class:bi-sort-down="{sortCol == col && sortDesc}"/>
                        <i class:bi-app="{sortCol !== col}"/>  
                    </th>
                    {/each}
                    <!-- <th>Actions</th> -->
                </tr>
            </thead>
            <tbody>
            {#each items as o, i}
            <tr class="clickable" on:click={()=>showModify(o)}>
                {#each table.columns as col, index}
                    {#if col === 'paid' && o[col] }
                    <td><i class="bi-check2"/></td>
                    {:else}
                    <td class="text-nowrap">{ o[col] || ""}</td>
                    {/if}
                {/each}
            </tr>    
            {/each} 
            </tbody>
        </table>        
    </div>
</div>


<div class="d-flex flex-row justify-content-end gap-3">
    {#if total_pages}
    {#if page > 1}
    <a href="#/" on:click={()=>goToPage(1)} class="page"><i class="bi-chevron-double-left"/></a>
    <a href="#/" on:click={()=>goToPage(page - 1)} class="page"><i class="bi-chevron-left"/></a>
    {:else}
    <i class="bi-chevron-double-left link-disabled" />
    <i class="bi-chevron-left link-disabled" />
    {/if}
    Page {page}/{total_pages}
    {#if page + 1 <= total_pages}
    <a href="#/" on:click={()=>goToPage(page + 1)} class="page"><i class="bi-chevron-right"/></a>
    <a href="#/" on:click={()=>goToPage(total_pages)} class="page"><i class="bi-chevron-double-right"/></a>
    {:else}
    <i class="bi-chevron-right link-disabled" />
    <i class="bi-chevron-double-right link-disabled" />
    {/if}    
    {/if}
</div>


<style>
    .bi-app {
        content: "\2122";
        color: transparent !important;
    }
    .clickable {
        cursor: pointer;
    }
    .link-disabled {
        color: lightgray;
    }
</style>