<script>
    import { onMount } from 'svelte';
    import { push, pop, querystring } from 'svelte-spa-router';
    import { DateInput } from 'date-picker-svelte'
    import Select from 'svelte-select';
    import dayjs from 'dayjs';
    import * as yup from "yup";
    import { API_URL, working, state, apierror } from '../lib/store';
    import Error from '../lib/Error.svelte';
    import CustomerItem from './CustomerItem.svelte';
    import ItemManager from '../lib/items';


    let url = `${API_URL}pyme/`
    let error;
    let errors = {};
    let isModify = false;
    let manager = null;

    let today = dayjs().toDate();
    let customer = null;

    let item = {
        date: today,
        customer: '',
        product: '',
        quantity: 1,
        price: 0,
        
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
        errors = {}
	});

    const customerChanged = () => {
        item.user.id = customer.id;
        console.log('errors before', errors);
        Object.keys(customer).forEach((key, index) => {
            if (key in item.user) {
                if (item.user[key] !== customer[key]) {
                    item.user[key] = customer[key];
                    if ('user.'+key in errors) {
                        delete errors["user."+key]
                        errors = errors; // reactive
                    }
                }
            }
        });
        console.log('errors after', errors);
        
    }

    const handleChange = async () => {
        errors = {}
        try {
			await schema.validate(item, { abortEarly: false });
		}
        catch (err) {
			errors = extractErrors(err);
		}
    }


    const handleSubmit = async () => {
		try {
			await schema.validate(item, { abortEarly: false });
			errors = {};
            await save();
            pop();
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
        if (q.length < 1)
            return []
        try {
            const r = await fetch(`${url}?q=${q}&sortcol=customer`, {
                headers: {
                    Authorization: 'Bearer ' + $state.token
                }
            });
            const js = await r.json();
            if (r.status == 200) {
                console.log(js)
                return js
            } 
        }
        catch (err) {
            console.log(err)
        }
        return []
    } 

    const save = async () => {
        let itemToSave = JSON.parse(JSON.stringify(item))
        // convert date to string
        itemToSave.date = dayjs(item.date).format('YYYY-MM-DD');
        itemToSave.checkout = dayjs(item.checkout).format('YYYY-MM-DD');
        console.log('saving item:', itemToSave);
        if(!isModify) {
            console.log('creating item');
            await manager.create(itemToSave);
        } else {
            console.log('updating item');
            await manager.modify(itemToSave);
        }
        error = manager.error;
    }

    
</script>

<div class="container">

    {#if error}
        <Error message={error} />
    {/if}
    {#if Object.keys(errors).length > 0}
        <Error message={`Check errors: [${Object.keys(errors).toString()}]`} />
    {/if}

    <form on:submit|preventDefault={handleSubmit}  class="needs-validation" novalidate>
    <div class="row">
        <div class="col">
            <label for="date" class="form-label">Date</label>
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
    </div>
   <!-- customer  -->
   <div class="row">
       <div class="col">
           <label for="customer" class="form-label">Customer</label>                    
           <Select 
               loadOptions={getCustomer} 
               itemId="id"
               bind:value={item.customer}
               on:change={customerChanged}
               hideEmptyState={true}
               placeholder="Search..."
               disabled={$working}>
               <div class="customer" slot="item" let:item let:index>
                   <CustomerItem {item} />
               </div>
               <div class="customer" slot="selection" let:selection>
                   <CustomerItem item={selection} />
               </div>
           </Select>
           {#if !customer}<small class="error">New customer</small>{/if}
       </div>
   </div>
   <div class="row">
       <div class="col-sm-6">
           <label for="name" class="form-label">Name</label>
           <input type="text" class="form-control" id="name"
               disabled={$working}
               on:change={handleChange}
               bind:value={item.customer}>
           {#if errors.hasOwnProperty("user.name")}<small class="error">{errors["user.name"]}</small>{/if}
       </div>
    </div>  
    <div class="row">
        <div class="col">

            <label for="id" class="form-label text-nowrap">Product</label>
            <select  disabled={$working}
                class="form-select"
                bind:value={item.product}>                
                <option value="A">A</option>
                <option value="B">B</option>
                <option value="C">C</option>
                <option value="W">W</option>
                <option value="R">R</option>
                <option value="T">T</option>
                <option value="V">V</option>
                <option value="O">Other</option>                        
            </select>
            {#if errors.product}<small class="error">{errors.product}</small>{/if}
         </div>
        <div class="col">
            <label for="quantity" class="form-label">Quantity</label>
            <input type="text" pattern="\d*" 
                disabled={$working}
                bind:value={item.quantity}                        
                class="form-control" id="quantity"
                    min="1" max="100">
            {#if errors.quantity}<small class="error">{errors.quantity}</small>{/if}

        </div>
        <div class="col">
            <label for="price" class="form-label">Price</label>
            <input type="number" 
                disabled={$working}
                bind:value={item.price}
                class="form-control" id="price">
            {#if errors.price}<small class="error">{errors.price}</small>{/if}
        </div>  
    </div>



    <div class="row text-end">
        <div class="col">
            <button class="btn btn-light" 
                on:click={()=>pop()}
                disabled={$working}>
                Cancel
            </button>
            <button class="btn btn-success"
                on:click={handleSubmit}
                disabled={$working}>
                Add Item
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
    input, textarea {
        background-color: whitesmoke;
    }
    :root {
        --date-input-width: 100%;
    }

</style>