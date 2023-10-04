<script>
// @ts-nocheck

    import { onMount } from 'svelte';
    import { pop, push } from 'svelte-spa-router';
    import { DateInput } from 'date-picker-svelte'
    import AutoComplete from "simple-svelte-autocomplete"
    import dayjs from 'dayjs';
    import * as yup from "yup";
    import { API_URL, working, state } from '../lib/store';
    import Error from '../lib/Error.svelte';
    import ItemManager from '../lib/itemManager';
    import { createEventDispatcher } from 'svelte';
    import OrderItem from '../lib/OrderItem.svelte';
	const dispatch = createEventDispatcher()

    let url = `${API_URL}pyme/`
    let error;
    let info;
    let errors = {};
    let manager = null;
    let today = dayjs().toDate();

    let customers = [];
    let products = [];
    let currentCustomer;
    let isModify; 

    export let params = {};

    export let order = {
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

    const schema = yup.object().shape({
        date: yup.date()
            .transform(function (value, originalValue) {
                return this.isType(value) ? value : dayjs(originalValue, "DD/MM/YYYY").isValid();
            })
            .typeError("Invalid date")
            .required("Required"),
        customer: yup.string().required("Required"), 
        price: yup.number().required("Required"),    
        items: yup.array().of(
            yup.object().shape({
                product: yup.string().required("Required"),
                quantity: yup.number().required("Required"),
                price: yup.number().required("Required"),    
            })
        ),
    });  

    onMount(async () => {
        if (!manager)
            manager = new ItemManager(url);
        if (params.id) {
            isModify = true;
            order = await getOrder(params.id);
            currentCustomer = order.customer;
        }
        products = await getProducts();
        errors = {}
	});

    const getOrder = async (order_id) => {
        let result = {};
        try {
            working.set(true);
            const r = await fetch(`${url}${order_id}`, {
                headers: {
                Authorization: "Bearer " + $state.token,
                },
            });
            const j = await r.json();
            if (r.status !== 200) {
                error = j.detail;
                console.log("error:", error);
            } else {
                console.log(j);
                result = j;
                result.date = dayjs(result.date).toDate();
            }
        } catch (err) {
            console.log(err);
            error = "API: Error in fetching data.";
        } finally {
            working.set(false);
        }
        return result;
    }
    const saveOrder = async () => {
        order.customer = currentCustomer;
        order.price = Number.parseInt(order.price);

        for (let item of order.items) {
            item.quantity = Number.parseInt(item.quantity);
		    item.price = Number.parseInt(item.price);
        }
		
        try {
			await schema.validate(order, { abortEarly: false });
			errors = {};
            await save();
            push("/");
		} catch (err) {
            errors = extractErrors(err);        
		}
    }
    const extractErrors = (err) => {
        console.log(JSON.stringify(err));
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
        let itemToSave = JSON.parse(JSON.stringify(order))
        // convert date to string
        itemToSave.date = dayjs(order.date).format('YYYY-MM-DD');
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
    const handleCreate = (username) => {
        console.log('adding ', username);
        customers.unshift(username);
        customers = customers;
        return username;
    }
    const updatePrice = (e) => {
        const index = e.detail;
        console.log('updating price item', index);
        let item = order.items[index];
        for (const p of products) {
            if (p[0]===item.product) {                
                item.price = item.quantity * Number(p[1]);
                console.log("price", item.price);
                order.item = {...item};
                break;                                
            }
        }
        updateTotal();
    }
    const updateTotal = () => {
        console.log('updating total...');
        let sum = 0;
        order.items.forEach( i => {
            sum += i.price;
        });
        order.price = sum;
    }
    const addItem = async () => {
        order.items.push({
            product: 'A',
            quantity: 1,
            price: 0,
        });
        order.items = [...order.items];
    }
    const removeItem = async (e) => {
        const index = e.detail;
        console.log('removing index', index);
        order.items.splice(index, 1);
        order.items = [...order.items];
        updateTotal()
    }
    const removeOrder = async () => {
        await manager.remove(order.id);
        push("/?info=Order deleted");
    }
</script>

<div class="container">

    <div class="row bg-light border-bottom">
        <div class="col h2">Order:</div>
    </div>

    <Error message={error} />

    <form on:submit|preventDefault={saveOrder}  
        class="needs-validation" novalidate>
    <div class="row">
        <div class="col">
            <label for="date" class="form-label">
                Date
            </label>
            <DateInput
                bind:value={order.date} 
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
                bind:selectedItem={order.customer}
                bind:text={currentCustomer}
                disabled={$working}
                create={true}
                createText={"Item doesn't exist, create one?"} 
                onCreate={handleCreate}         />
                <br>                
            {#if errors.customer}<small class="error">{errors.customer}</small>{/if}
           
       </div>
    </div>

    {#each order.items as item, index}
        <OrderItem {products} {item} hasLabels={index===0} {index}
            on:removeItem={removeItem}
            on:updatePrice={updatePrice}
            on:updateTotal={updateTotal}
        />
    {/each}

    <div class="row">
        <div class="col">            
            <label for="price" class="form-label">Total Price</label>
            <input type="text" pattern="\d*" 
                disabled={$working}
                bind:value={order.price}
                class="form-control" id="price"
                min="1" max="10000">
        </div>  
        <div class="col text-end">
            <button class="btn btn-light mt-4" id="plus" 
                on:click|preventDefault={addItem}>
                <i class="bi-plus-lg"/>        
            </button>
        </div> 
    </div>
    
    <div class="row">
        <div class="col">
            <div class="form-check">
                <input class="form-check-input" type="checkbox" 
                bind:checked={order.paid} disabled={$working} id="paid">
                <label class="form-check-label" for="paid">
                  Paid
                </label>
            </div>
        </div>  
    </div>
    
    <div class="row">
        <div class="col">
            <label for="notes" class="form-label">Notes</label>
            <input type="text"
                disabled={$working}
                bind:value={order.notes}
                class="form-control" id="notes">
        </div>  
    </div>
    <div class="row text-end">
        <div class="col">
            {#if isModify}
            <button class="btn btn-danger  w-100"
                on:click|preventDefault={removeOrder}
                disabled={$working}>
                <i class="bi-trash3"/>
            </button>
            {/if}
        </div>
        <div class="col">
        </div>
        <div class="col">
            <button class="btn btn-secondary w-100" 
                on:click|preventDefault={()=> pop()}
                disabled={$working}>
                Close
            </button>
        </div>
        <div class="col">
            <button class="btn btn-success w-100"
                on:click|preventDefault={saveOrder}
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