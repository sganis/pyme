<script>
    import{_} from 'svelte-i18n';
    import { link } from "svelte-spa-router";
    import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

    export let table;
    export let sortCol;
    export let sortDesc;
    export let urlpath;
    export let items;

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
                <td><a href="/{urlpath}/{o[table.id_column]}" 
                    use:link>{String(o.id).padStart(5, '0')}</a>
                </td>
                {#each table.columns.slice(1) as col}
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

<div>{$_("total.items", {values: {count: items.length}})}</div>


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