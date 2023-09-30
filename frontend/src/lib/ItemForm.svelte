<script>
// @ts-nocheck

    import { onMount } from 'svelte';
    import { push } from 'svelte-spa-router';
    import { DateInput } from 'date-picker-svelte'
    import AutoComplete from "simple-svelte-autocomplete"
    import dayjs from 'dayjs';
    import * as yup from "yup";
    import { API_URL, working, state } from './store';
    import Error from './Error.svelte';
    import ItemManager from './itemManager';
    import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher()

    const close = (e) => {
        e.preventDefault();
        dispatch('close');
    }

    let url = `${API_URL}pyme/`
    let error;
    let errors = {};
    let manager = null;
    let today = dayjs().toDate();

    let customers = [];
    let products = [];
    let currentCustomer;
    let currentProduct;

    export let isModify;
    export let item = {
        date: today,
        customer: '',
        product: 'A',
        quantity: 1,
        price: 0,
        paid: true,
        
    }

    const schema = yup.object().shape({
        date: yup.date()
            .transform(function (value, originalValue) {
                return this.isType(value) ? value : dayjs(originalValue, "DD/MM/YYYY").isValid();
            })
            .typeError("Invalid date")
            .required("Required"),
        customer: yup.string().required("Required"), 
        product: yup.string().required("Required"),
        quantity: yup.number().required("Required"),
        price: yup.number().required("Required"),    
    });    
    
    onMount(async () => {
        if (!manager)
            manager = new ItemManager(url);
        products = await getProducts();
        updatePrice();
        errors = {}
	});

    const handleSubmit = async () => {
        item.customer = currentCustomer;
        item.quantity = Number.parseInt(item.quantity);
		item.price = Number.parseInt(item.price);
		try {
			await schema.validate(item, { abortEarly: false });
			errors = {};
            await save();
		} catch (err) {
            errors = extractErrors(err);
            //console.log(errors);           
		}

    }
    const extractErrors = (err) => {
		return err.inner.reduce((acc, err) => {
			return { ...acc, [err.path]: err.message };
		}, {});
	}

    const getCustomer = async (q) => {
        if (q.length < 1){
            customers = [];
            return customers;
        }
        try {
            const r = await fetch(`${API_URL}pyme/customers/?q=${q}`, {
                headers: {
                    Authorization: 'Bearer ' + $state.token
                }
            });
            const js = await r.json();
            if (r.status == 200) {
                console.log(js)
                customers = js
                return customers;
            } 
        }
        catch (err) {
            console.log(err)
        }
        customers = []
        return customers;
    } 
    const getProducts = async () => {
        try {
            const r = await fetch(`${API_URL}pyme/products/`, {
                headers: {
                    Authorization: 'Bearer ' + $state.token
                }
            });
            const js = await r.json();
            if (r.status == 200) {
                //console.log(js);
                customers = js;
                return customers;
            } 
        }
        catch (err) {
            console.log(err)
        }
        return [];
    } 
    const save = async () => {
        let itemToSave = JSON.parse(JSON.stringify(item))
        // convert date to string
        itemToSave.date = dayjs(item.date).format('YYYY-MM-DD');
        console.log('saving item:', itemToSave);
        if(!isModify) {
            console.log('creating item');
            await manager.create(itemToSave);
        } else {
            console.log('updating item');
            await manager.modify(itemToSave);
        }
        error = manager.error;
        dispatch('saved');
    }

    const remove = async () => {

    }
    const handleCreate = (username) => {
        console.log('adding ', username);
        customers.unshift(username);
        customers = customers;
        return username;
    }
    const updatePrice = () => {
        console.log('updating price...');
        for (const p of products) {
            if (p[0]===item.product) {
                item.price = item.quantity * Number(p[1]);
                break;
            }
        }
    }
</script>

<div class="container">
    {#if error}
        <Error message={error} />
    {/if}
    {#if Object.keys(errors).length > 0}
        <Error message={`Check errors: [${Object.keys(errors).toString()}]`} />
    {/if}

    <form on:submit|preventDefault={handleSubmit}  
        class="needs-validation" novalidate>
    <div class="row">
        <div class="col">
            <label for="date" class="form-label">
                Date
            </label>
            <DateInput
                bind:value={item.date} 
                closeOnSelection={true}
                format="dd/MM/yyyy"
                visible={false}
                browseWithoutSelecting={true}
                placeholder="Checkin date"
                disabled={$working} />
            {#if errors.date}<small class="error">{errors.date}</small>{/if}

        </div>
        <div class="col">
            <label for="customer" class="form-label">
                Customer
            </label>                    
            <br>
            <AutoComplete
                inputClassName="form-control"
                searchFunction={getCustomer}
                delay="200"
                bind:selectedItem={item.customer}
                bind:text={currentCustomer}
                create={true}
                createText={"Item doesn't exist, create one?"} 
                onCreate={handleCreate}         />
                <br>                
            {#if errors.customer}<small class="error">{errors.customer}</small>{/if}
           
       </div>
    </div>
    <div class="row align-items-center">
        <div class="col">
            <label for="id" class="form-label text-nowrap">Product</label>
            <select  disabled={$working}
                class="form-select"                
                bind:value={item.product}
                on:change={updatePrice}>    
                {#each products as p}            
                    <option value={p[0]}>{p[0]} ({p[1]})</option>
                {/each}
            </select>
            {#if errors.product}<small class="error">{errors.product}</small>{/if}
         </div>
        <div class="col">
            <label for="quantity" class="form-label">Units</label>
            <input type="text" pattern="\d*" 
                disabled={$working}
                bind:value={item.quantity}
                on:change={updatePrice}                        
                class="form-control" id="quantity"
                    min="1" max="100">
            {#if errors.quantity}<small class="error">{errors.quantity}</small>{/if}

        </div>
        <div class="col">
            <label for="price" class="form-label">Price</label>
            <input type="text" pattern="\d*" 
                disabled={$working}
                bind:value={item.price}
                class="form-control" id="price">
            {#if errors.price}<small class="error">{errors.price}</small>{/if}
        </div>  
        <div class="col">
            <div class="form-check mt-4">
                <input class="form-check-input" type="checkbox" 
                bind:checked={item.paid} disabled={$working} id="paid">
                <label class="form-check-label" for="paid">
                  Paid
                </label>
              </div>
            {#if errors.price}<small class="error">{errors.price}</small>{/if}
        </div>  
    </div>
    <div class="row">
        <div class="col">
            <label for="notes" class="form-label">Notes</label>
            <input type="text"
                disabled={$working}
                bind:value={item.notes}
                class="form-control" id="notes">
        </div>  
    </div>
    <br>
    <div class="row text-end">
        <div class="col">
            {#if isModify}
            <button class="btn btn-danger  w-100"
                on:click={remove}
                disabled={$working}>
                <i class="bi-trash3"/>
            </button>
            {/if}
        </div>
        <div class="col">
        </div>
        <div class="col">
            <button class="btn btn-secondary w-100" 
                on:click={close}
                disabled={$working}>
                Close
            </button>
        </div>
        <div class="col">
            <button class="btn btn-success w-100"
                on:click={handleSubmit}
                disabled={$working}>
                Save
            </button>
        </div>
    </div>
    
    
</form>
</div>


<style>
    .row {
        margin-top: 0.5em;
    }
    label {
        margin-bottom: 2px;
        font-size: small;
    }
    input{
        background-color: whitesmoke;
    }
    :root {
        --date-input-width: 100%;
    }

</style>