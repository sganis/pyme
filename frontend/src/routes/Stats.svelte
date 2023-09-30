<script>
    import { onMount } from 'svelte';
    import { push } from 'svelte-spa-router';
    import { API_URL, state, working, apierror } from '../lib/store';
  import StatItem from '../lib/StatItem.svelte';
    
    let url = `${API_URL}pyme/stat`
    let error = '';
    let quarters = [];
    let years = []
    let products = [];
    let customers = [];


    onMount(async () => {
        quarters = await getTop('quarters');          
        years = await getTop('years');          
        products = await getTop('products');          
        customers = await getTop('customers');          
    });

    const getTop = async (table) => {
        let items = [];
        try {
            $working = true;
            const r = await fetch(`${url}/${table}/`, {
                headers: {
                    "Content-Type": "application/json",
                    Authorization: 'Bearer ' + $state.token,
                },
            });
            const js = await r.json();
            if (r.status === 200) {
                items = js;
            } else {
                error = js.detail;
            }
        }
        catch (err) {
            console.log(err)
        }
        finally {
            $working = false;
            return items;
        }
    }


</script>

<div class="row bg-light border-bottom">
    <div class="col h2">Stats</div>
</div>
<br>
<StatItem title="Quarters" items={quarters}/>
<StatItem title="Years" items={years}/>
<StatItem title="Products" items={products}/>
<StatItem title="Customers" items={customers}/>
