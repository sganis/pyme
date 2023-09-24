<script>
  import {onMount} from 'svelte'
  import { state, working } from './store';
  import Working from "./Working.svelte";

  export let urlpath;
  export let title;
  export let params = {}
  
  let url = `${import.meta.env.VITE_PUBLIC_BASE_URL}${urlpath}`
  let error;
  let item = {}

  onMount(async () => {
  console.log('onMount in Item');   
  try {
      $working = true;
      const r = await fetch(`${url}/${params.id}`, {
      headers: {
          Authorization: 'Bearer ' + $state.token
      }
      });
      const js = await r.json();
      if ('detail' in js) {
          error = js.detail;
      } else {
          item = js;  
      }
  } catch (err) {
      console.log(err);
      error = 'Server down';
  } finally {
      $working = false;
  }

  });

  function isObject(value) {
    return value != null && (typeof value === 'object' || typeof value === 'function');
  }

</script>
<h2 class="bg-light p-1 border-bottom">{title}: {params.id}</h2>
<hr>
{#if $working}
  <Working message="Loading..." />
{:else}
  {#if error} 
    <div class="alert alert-danger">{error}</div>
  {:else}
    <table class="table table-hover table-sm">
      <thead class="table-success">
          <tr><th>Property</th><th>Value</th></tr>
      </thead>
      <tbody>
      {#each Object.entries(item) as [key, value]}
        <tr>
            <td>{key}</td>
            <td>{isObject(value) ? JSON.stringify(value, null, 1) : value}</td>        
        </tr>    
      {/each}
      </tbody>
    </table>
  {/if}
{/if}

