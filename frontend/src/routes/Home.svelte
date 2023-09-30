<script>
    import { onMount } from 'svelte';
    import { push } from 'svelte-spa-router';
    import { API_URL, state } from '../lib/store';
    import ItemsTable from '../lib/ItemTable.svelte';
    import ItemManager from "../lib/itemManager.js"
    
    
    let url = `${API_URL}pyme/`
    let isModal;
    let isModify;
    let isRemove;
    let sortCol = 'id';
    let sortDesc = true;
    let manager;

    let title = "Items";
    let table = {
        header : ['Date','Customer','Qty','Prod','Price'],
        columns : ['date','customer','quantity','product','price'],
    }
    let itemInit  = {
        id: 0,
        name: "",
        num_guests: 2,
        max_guests: 3,
    };

    let item = {...itemInit}

    let items = [];
    let error = '';
    let timer;
    const waitTime = 500;
  
    onMount(async () => {
        console.log('mouning home, state:', JSON.stringify($state));
        if (!$state.token) {
            push('/login');
            return;
        }    
        console.log(url);
        if (manager===undefined)
            manager = new ItemManager(url);
        await manager.search();
        error = manager.error;
        items = manager.result;       
    });

    const refresh = async () => {
        await manager.search();
        error = manager.error;
        items = manager.result;
    }
    
    const searchLater = async (e) => {
        const searchText = e.detail;
        manager.searchText = searchText
        clearTimeout(timer);
        if (e.key == "Enter") {
            await manager.search();
            error = manager.error;
            items = manager.result;
        } else {
            timer = setTimeout(async () => {
                await manager.search();
                error = manager.error;
                items = manager.result;
            }, waitTime);
        }
    }
 

    const save = async () => {
        console.log('saving item:', item);
        if(!isModify) {
            console.log('creating item');
            await manager.create(item);
        } else {
            console.log('updating item');
            await manager.modify(item);
        }
        error = manager.error;
        items = manager.result;
        if (!error) {
            isModal = false;
        }
    }

    const remove = async () => {
        await manager.remove(item.id);
        error = manager.error;
        items = manager.result;
        isModal = false;
        isRemove = false;
    }

    const sort = async (e) => {
        let col = e.detail;
        if (col === manager.sortCol) {
            manager.sortDesc = !manager.sortDesc;
        } else {
            manager.sortCol = col;
            manager.sortDesc = false;
        }
        sortCol = manager.sortCol;
        sortDesc = manager.sortDesc;
        await manager.search();
        error = manager.error;
        items = manager.result;
    }

    const showCreate = () => {
        item = {...itemInit};
        isModal = true;
        isModify = false;

        error = '';
    }


    const showModify = (e) => {
        let o = e.detail;
        item = {...o};
        isModify = true;
        isModal = true;
        error = '';
    }

    const showRemove = (e) => {
        let o = e.detail;
        item = {...o};
        isRemove = true; 
        isModal = true;
        error = '';
    }

    const cancel = () => {
        isRemove = false;
        isModal = false;
        isModify = false;
        error = '';
    }

</script>

<svelte:head>
  <title>Pyme</title>
</svelte:head>

<div class="row ms-1">
  Mode: {import.meta.env.MODE},   API: {import.meta.env.VITE_PUBLIC_BASE_URL}<br>
  User: {$state?.username || ""}, Token: {$state?.token.substr($state?.token.length-10,10)}
</div>
<br>

<ItemsTable 
    {title}
    {items} 
    {table} 
    {sortCol} 
    {sortDesc} 
    on:refresh={refresh}
    on:sort={sort}
    on:searchLater={searchLater} 
    on:showCreate={showCreate}  
    on:showRemove={showRemove}  
    on:showModify={showModify} />


<style>

</style>