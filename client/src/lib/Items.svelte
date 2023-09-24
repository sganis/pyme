<script>
    import { onMount } from 'svelte';
    import { link } from "svelte-spa-router";
    import { state, working } from '../lib/store';
    import RoomTypeTable from '../routes/RoomTypeTable.svelte';

    export let urlpath;
    export let title;

    let api = `${import.meta.env.VITE_PUBLIC_BASE_URL}/${urlpath}`
    let result = [];
    let error = "";
    let searchText = "";
    let sortCol = "id";
    let sortDesc = true;
    let isModal = false;
    let isRemove = false;
    let isModify = false;

    
    let item  = {
        id: 0,
        name: "",
        num_guests: 2,
        max_guests: 3,
    };
    
    onMount(async () => {
        await search();
	});

    let timer;
    const waitTime = 500;

    const searchLater = async (e) => {
        //console.log(e);
        clearTimeout(timer);
        if (e.key == "Enter") {
            await search();
        } else {
            timer = setTimeout(async () => {
                //console.log('searching after timeout')
                await search();

            }, waitTime);
        }
    }

    const sort = async (col) => {
        if (col === sortCol) {
            sortDesc = !sortDesc;
        } else {
            sortCol = col;
            sortDesc = false;
        }
        await search();
    }

    const search = async () => {
        try {
            $working = true;
            console.log(`searching: ${searchText} sortcol: ${sortCol} desc: ${sortDesc}`);
            let query = `q=${searchText}&sortcol=${sortCol}&desc=${sortDesc}`;
            const r = await fetch(`${api}/?${query}`, {
                headers: {
                    Authorization: 'Bearer ' + $state.token
                }
            });
            const j = await r.json();
            console.log(j);
                result=j ;
        } catch (err) {
            console.log(err);
            error = 'Error in fetching data.';
        } finally {
            $working = false;
	}
};

    const validate = async () => {
        /** @type {HTMLFormElement} */
        let name = document.getElementById("form");
        if (name?.checkValidity() === false) {
            name?.reportValidity();
            return false;
        }
        return true;
    }
    
    const showCreate = () => {
        item.name = "";
        item.num_guests = 2;
        item.max_guests = 3;
        isModify = false;
        isModal = true;
    }

    const showModify = (o) => {
        item = {...o};
        isModify = true;
        isModal = true;
    }

    const showRemove = (o) => {
        item = {...o};
        isRemove = true; 
    }
    
    const save = async () => {
        if(!isModify) {
            await create();
        } else {
            await modify();
        }
    }

    const refresh = async()=> {
        await search();
    }
    const create = async () => {
        if (! await validate()) {
            return;
        }
        try {
            $working = true;
            //console.log($state.token);
            const r = await fetch(api, {
                method: 'POST',                
                headers: {
                    'Content-Type': 'application/json',
                     Authorization: 'Bearer ' + $state.token
                },
                body: JSON.stringify(item)
            });
            const js = await r.json();
            //console.log(js);
            
            if (r.status !== 200) {
                error = js.detail;                
            } else {
                await search();
                isModal = false;                    
            }
        }
        catch (err) {
            console.log(err)
        }
        $working = false;
    }

    const modify = async () => {
        if (! await validate()) {
            return;
        }
        try {
            $working = true;
            const r = await fetch(api, {
                method: 'PUT',                
                headers: {
                    'Content-Type': 'application/json',
                     Authorization: 'Bearer ' + $state.token
                },
                body: JSON.stringify(item)
            });
            const js = await r.json();
            //console.log(js);
            
            if (r.status !== 200) {
                error = js.detail;                
            } else {
                await search();
                isModal = false;                    
            }
        }
        catch (err) {
            console.log(err)
        }
        $working = false;
    }

    const remove = async () => {
        try {
            $working = true;
            const r = await fetch(`${api}/${item.id}`, {
                method: 'DELETE',                
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: 'Bearer ' + $state.token
                },
            });
            const js = await r.json();
            //console.log(js);
            
            if (r.status !== 200) {
                error = js.detail;                
            } else {
                await search();
                isRemove = false;                    
            }
        }
        catch (err) {
            console.log(err)
        }
        $working = false;
    }

    const cancel = async () => {
        isModal=false;
        error = '';
    }
</script>

<h2 class="bg-light p-1 border-bottom">{title}</h2>

<div class="d-flex justify-content-between gap-5">
    <div class="d-flex justify-content-start gap-1">
        <button
            class="btn btn-success  btn-width" on:click={() => showCreate()}>
            Create
        </button>
        <button class="btn btn-light btn-width" 
            on:click={refresh}>
            Refresh            
        </button>
        
    </div>
    <input class="form-control filter-width" type="text" placeholder="Filter..."
        bind:value={searchText}
        on:keyup={searchLater} />    
</div>

<br>
{#if error} 
    <div class="alert alert-danger">{error}</div>
{:else}
    <RoomTypeTable />
{/if}


<style>
    .message {
        height: 60px;
    }
    .btn-width {
        width: 100px;
    }
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