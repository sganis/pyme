<script>
    import { state, } from '../lib/store'
</script>

<svelte:head>
  <title>Pyme</title>
</svelte:head>

<h2 class="bg-light p-1 border-bottom">
    Home
</h2>

<p>
    Mode: {import.meta.env.MODE}<br>
    API: {import.meta.env.VITE_PUBLIC_BASE_URL}<br>
    User: {$state?.username || ""}<br>
</p>
Token: 
<div class="wrap">{JSON.stringify($state?.token, null, 1)}</div>




<style>
.wrap {
  overflow-wrap: break-word;
}
</style>