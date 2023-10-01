<script>
    import { onMount } from 'svelte';
    import { working } from './store';
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher()

    export let item = {};
    export let products = [];
    export let errors = [];
    export let hasLabels = true;
    export let index;


    onMount(async () => {
        dispatch('updatePrice',index);
	});


    const updatePrice = () => {
        dispatch('updatePrice', index);
    }
    const updateTotal = () => {
        dispatch('updateTotal');
    }
    const removeItem = () => {
        dispatch('removeItem', index);
    }
</script>

<div class="row" class:mt-2={!hasLabels}>
    <div class="col" >
        {#if hasLabels}
        <label for="id" class="form-label text-nowrap">Product</label>
        {/if}
        <select  disabled={$working}
            class="form-select"  
            bind:value={item.product}              
            on:change={updatePrice}>    
            {#each products as p}            
                <option value={p[0]}>{p[0]} ({p[1]})</option>
            {/each}
        </select>
        <!-- {#if errors.product}<small class="error">{errors.product}</small>{/if} -->
     </div>
    <div class="col">
        {#if hasLabels}
        <label for="quantity" class="form-label">Units</label>
        {/if}
        <input type="text" pattern="\d*" 
            disabled={$working}
            bind:value={item.quantity}
            on:change={updatePrice}                        
            class="form-control" id="quantity"
                min="1" max="100">
        <!-- {#if errors.quantity}<small class="error">{errors.quantity}</small>{/if} -->
    </div>
    <div class="col">
        {#if hasLabels}
        <label for="price" class="form-label">Price</label>
        {/if}
        <input type="text" pattern="\d*" 
            disabled={$working}
            bind:value={item.price}
            on:change={updateTotal}  
            class="form-control" id="price">
        <!-- {#if errors.price}<small class="error">{errors.price}</small>{/if} -->
    </div>  
    <div class="col text-end">
        {#if hasLabels}
        <label for="trash" class="form-label">&nbsp;</label><br>
        {:else}
        <button class="btn btn-light" id="trash"
            on:click|preventDefault={removeItem}>
            <i class="bi-trash3"/>        
        </button>
        {/if}        
    </div>  
</div>


<style>
    label {
        font-size: small;
        margin-bottom: 0px;
    }
</style>