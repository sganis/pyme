<script>
    import { link } from "svelte-spa-router";
    export let params = {};
    let wild = params.wild; 
    console.log('path not found:', wild);
</script>
    
<h1>We are sorry!</h1>
<p>The place do not exist.</p>
<img src="https://picsum.photos/id/685/800/400" alt="img">
<p></p>
<a href='/' use:link>Home</a>

<style>
    img {
        width: 100%;
    }
</style>