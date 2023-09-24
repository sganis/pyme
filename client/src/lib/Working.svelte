<script>
    import {working} from './store'
    export let message;
</script>

{#if $working}
<div>
    <div class="spinner-border spinner-border-sm text-secondary" aria-hidden="true"></div>
    &nbsp;
    {message}
</div>
{/if}
