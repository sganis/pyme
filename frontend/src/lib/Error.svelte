<script>
    export let message;

</script>

{#if message}
<div class="row alert alert-danger">
    {message}
</div>
{/if}