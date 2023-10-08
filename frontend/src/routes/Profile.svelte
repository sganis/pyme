<script>
    import Error from '../lib/Error.svelte';
  import Info from '../lib/Info.svelte';
import { state, working } from '../lib/store';
    import { push } from 'svelte-spa-router';

    let error;
    let info;
    let password;
    let newpassword;
    let newpassword2;
    let url = `${import.meta.env.VITE_PUBLIC_BASE_URL}password`;



    const onSubmit = async (e) => {
        try {
            $working = true;
            console.log('logging in...');
            const r = await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({username: $state.username, password, newpassword}),                
            });
            const js = await r.json();
            console.log(js);
            if (r.status === 200) {
                info = js.result;                
            } else {
                error = js.detail;
            }
        }
        catch (err) {
            console.log(err)
        }
        finally {
            $working = false;
        }
    }

</script>
<div class="container">
<div class="row">
    User: {$state.username}
</div>
<br>
<div class="login">
    <form class="form" on:submit|preventDefault={onSubmit}>
        <div class="mb-3">
            <label  class="form-label" for="password">Current password</label>
            <input 
                bind:value={password} 
                class="form-control" 
                id="password" 
                name="password" 
                type="password" 
                required 
                disabled={$working}/>
        </div>
        <div class="mb-3">
            <label class="form-label" for="newpassword">New password</label>
            <input 
                bind:value={newpassword}    
                class="form-control" 
                id="newpassword" 
                name="newpassword" 
                type="password" 
                required 
                disabled={$working} />
        </div>
        <div class="mb-3">
            <label class="form-label" for="newpassword2">Repeat new passord</label>
            <input 
                bind:value={newpassword2}    
                class="form-control" 
                id="newpassword2" 
                name="newpassword2" 
                type="password" 
                required 
                disabled={$working} />
        </div>
        
        <button class="btn btn-primary" 
            type="submit" 
            disabled={$working}>
            Change Password
        </button>
    </form>
    </div>
    <br>
    <Info {info} />
    <Error {error} />
</div>