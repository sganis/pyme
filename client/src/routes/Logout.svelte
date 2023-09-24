<script>
    import { state, } from '../lib/store'
    import { push } from "svelte-spa-router";
    $state = {username: '', token: ''}
    localStorage.removeItem('state');
    push('/login');


</script>