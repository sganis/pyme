<script>
    import { onMount } from 'svelte';
    //import { push } from 'svelte-spa-router';
    import dayjs from 'dayjs';
    import { API_URL, state, apierror } from '../lib/store';
    import ItemsTable from '../lib/ItemTable.svelte';
    import ItemManager from "../lib/itemManager.js"
    import ItemForm from '../lib/ItemForm.svelte';
    
    let url = `${API_URL}pyme/`
    let isModal;
    let isModify;
    let isRemove;
    let sortCol = 'date';
    let sortDesc = true;
    let manager;
    let showForm = false;
    let showToolbar = true;
    let today = dayjs().toDate();

    let title = "Items";
    let table = {
        header : ['Date','Cust','Qty','Prod','Price', 'Paid'],
        columns : ['date','customer','quantity','product','price','paid'],
    }
    let itemInit = {
        date: today,
        customer: '',
        product: 'A',
        quantity: 1,
        price: 0,
        paid: true,
    }
    let item = {...itemInit}

    let items = [];
    let error = '';
    let timer;
    const waitTime = 500;
  
    onMount(async () => {
        console.log('mouning home, state:', JSON.stringify($state));
        console.log(url);
        if (manager===undefined)
            manager = new ItemManager(url, sortCol, sortDesc);
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
        showForm = true;
        showToolbar = false;
        isModify = false;
        error = '';
    }
    const showModify = (e) => {
        let o = e.detail;
        o.date = dayjs(o.date).toDate();
        item = {...o};
        showForm = true;
        showToolbar = false;
        isModify = true;
        error = '';
    }
    const showRemove = (e) => {
        let o = e.detail;
        item = {...o};
        isRemove = true; 
        isModal = true;
        error = '';
    }
    const closeForm = () => {
        isRemove = false;
        showForm = false;
        showToolbar = true;
        isModify = false;
        error = '';
    }
</script>

<svelte:head>
  <title>Pyme</title>
</svelte:head>

<div class="row ms-1">
  Mode: {import.meta.env.MODE},   API: {import.meta.env.VITE_PUBLIC_BASE_URL}<br>
  User: {$state?.username || ""}, Token: {$state?.token?.substr($state?.token.length-10,10)}
</div>
<br>

{#if showForm}

<div class="row bg-light border-bottom">
    <div class="col h2">Item:</div>
</div>
<ItemForm {item} {isModify} on:close={closeForm} on:saved={refresh}/>
{/if}

<br>

<ItemsTable 
    {title}
    {showToolbar}
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