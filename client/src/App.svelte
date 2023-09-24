<script>
// @ts-nocheck

  import 'bootstrap/dist/css/bootstrap.min.css';
  import 'bootstrap/dist/js/bootstrap.min.js';
  import 'bootstrap-icons/font/bootstrap-icons.css'

  import { state, apierror } from './lib/store'

  import { link, push, pop, replace, querystring } from "svelte-spa-router";
  import active from 'svelte-spa-router/active'
  import Router from 'svelte-spa-router'
  import Home from './routes/Home.svelte'
  import NotFound from './routes/NotFound.svelte'
  import Login from './routes/Login.svelte';
  import Logout from './routes/Logout.svelte';
  import Error from './lib/Error.svelte';
  import Working from './lib/Working.svelte';
  import ItemForm from './routes/ItemForm.svelte';
  
  
  const routes = {
      '/': Home,
      '/login': Login,
      '/logout': Logout,
      '/add': ItemForm,

      // Catch-all
      '*': NotFound,
  }


</script>

<nav class="navbar navbar-expand-sm navbar-light bg-light">
  <div class="container-fluid">
    <a class="navbar-brand" href="/">
      <img src="/favicon.svg" alt="logo" width="32" />
      &nbsp;
      Pyme</a>
      <Working message="" />
    <button class="navbar-toggler" type="button" 
      data-bs-toggle="collapse" data-bs-target="#navbarNav" 
      aria-controls="navbarNav" aria-expanded="false" 
      aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarNav">
      <ul class="navbar-nav justify-content-end ms-auto">
        {#if $state?.username }
        <li class="nav-item">
          <a class="nav-link" href="/add" use:link use:active>
            Add
          </a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="/stats" use:link use:active>
           Stats
          </a>
        </li>
        <li class="nav-item">
          <a class="nav-link" href="/logout" use:link >
            Logout
          </a>
        </li>
        {:else}
          <li class="nav-item">
            <a class="nav-link" href="/login" use:link use:active>
              Login
            </a>
          </li>
        {/if}       
    </ul>
    </div>
  </div>
</nav>

<br>

<div class="container">

  <Error message={$apierror} />

  <Router {routes} restoreScrollState={true} />

  <!-- svelte-ignore missing-declaration -->
  <footer>
    <br>
    v{PKG.version}
  </footer>
</div>

<style>
  /* Style for "active" links; 
  need to mark this :global because the router adds the class directly */
  :global(a.active) {
      color: black;
      text-decoration: none;
  }
  </style>