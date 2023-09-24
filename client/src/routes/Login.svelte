<script>
    import { state, working } from '../lib/store';
    import Working from '../lib/Working.svelte';
    import {fade} from 'svelte/transition'
  import { push } from 'svelte-spa-router';

    let error = null;
    let url = `${import.meta.env.VITE_PUBLIC_BASE_URL}token`;

    const onSubmit = async (e) => {
        try {
            $working = true;
            const data = new FormData(e.target)
            console.log('logging in...');
            const r = await fetch(url, {
                method: 'POST',
                body: data
            });
            const js = await r.json();
            console.log(js);
            if (r.status === 200) {
                state.set({
                    'username': js.username, 
                    'token': js.token,
                });
                localStorage.setItem('state', JSON.stringify($state));
                push('/');                
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

<div in:fade="{{duration: 500}}">

<h2 class="bg-light p-1 border-bottom">
    Login
</h2>
    

<div class="login">
<form class="form" on:submit|preventDefault={onSubmit}>
	<div class="mb-3">
		<label class="form-label" for="username">Username</label>
		<input class="form-control" id="username" name="username" type="text" required disabled={$working} />
	</div>
	<div class="mb-3">
		<label  class="form-label" for="password">Password</label>
		<input  class="form-control" id="password" name="password" type="password" required disabled={$working}/>
	</div>
	<button class="btn btn-primary" type="submit" disabled={$working}>Log in</button>
</form>
</div>
<br>
{#if $working}
    <Working message="Logging in..." />
{:else}
    {#if error}
        <p class="error">{error}</p>
    {/if}
{/if}
</div>

<style>
.login {
	max-width: 250px;
}
.error {
	color: red;
}
	
</style>

