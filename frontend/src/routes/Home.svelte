<script>
    import { onMount } from 'svelte';
    import { push, querystring } from 'svelte-spa-router';
    import dayjs from 'dayjs';
    import { API_URL, state, apierror } from '../lib/store';
    import ItemsTable from '../lib/ItemTable.svelte';
    import ItemManager from "../lib/itemManager.js"
    import Info from '../lib/Info.svelte';
    
    let info = "";
    if ($querystring) {
        let querystr = new URLSearchParams($querystring);
        info = querystr.get("info");
        console.log(info);
    }
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

    let title = "Orders";
    let table = {
        header : ['ID','Date','Cust','Total', 'Paid'],
        columns : ['id','date','customer','price','paid'],
    }
    let itemInit = {
        date: today,
        customer: '',
        price: 0,
        paid: true,
        notes: '',
        items : [{
            product: 'A',
            quantity: 1,
            price: 0,
        }]        
    }
    let order = {...itemInit}

    let result = {
        items : [],
        offset : 0,
    };
    let limit = 10;
    let error = '';
    let timer;
    const waitTime = 500;
  
    onMount(async () => {
        console.log('mouning home, state:', JSON.stringify($state));
        console.log(url);
        if (manager===undefined)
            manager = new ItemManager(url, sortCol, sortDesc, limit, 0);
        await manager.search();
        error = manager.error;
        result = manager.result;       
    });

    const refresh = async () => {
        await manager.search();
        error = manager.error;
        result = manager.result;       
    } 
    
    const searchLater = async (e) => {
        const searchText = e.detail;
        manager.searchText = searchText
        clearTimeout(timer);
        if (e.key == "Enter") {
            await manager.search();
            error = manager.error;
            result = manager.result;       
        } else {
            timer = setTimeout(async () => {
                await manager.search();
                error = manager.error;
                result = manager.result;       
            }, waitTime);
        }
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
        result = manager.result;       
    }
    const showCreate = () => {
        order = {...itemInit};
        showForm = true;
        showToolbar = false;
        isModify = false;
        error = '';
        push("/order");
    }
    const showModify = (e) => {
        let o = e.detail;
        o.date = dayjs(o.date).toDate();
        order = {...o};
        showForm = true;
        showToolbar = false;
        isModify = true;
        error = '';
        push("/order/"+o.id);
    }
    const showRemove = (e) => {
        let o = e.detail;
        order = {...o};
        isRemove = true; 
        isModal = true;
        error = '';
    }
    const goToPage = async (e) => {
        let page = e.detail;
        if (page < 1) page = 1;
        let offset = (page-1)*limit;
        //console.log('goto page:', page, offset);
        
        manager.offset = offset;
        await manager.search();
        error = manager.error;
        result = manager.result;
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

<Info {info} />

<ItemsTable 
    {title}
    {showToolbar}
    items={result.items} 
    {limit}
    offset={result.offset}
    total={result.total}
    {table} 
    {sortCol} 
    {sortDesc} 
    on:refresh={refresh}
    on:sort={sort}
    on:searchLater={searchLater} 
    on:showCreate={showCreate}  
    on:showRemove={showRemove}  
    on:showModify={showModify} 
    on:goToPage={goToPage} />


<style>

</style>