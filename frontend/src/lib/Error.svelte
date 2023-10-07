<script>
    export let error;

</script>

{#if error}
<div class="row alert alert-danger">
    {error}
</div>
{/if}