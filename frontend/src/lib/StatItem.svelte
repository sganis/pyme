<script>
    export let items = [];
    export let title;
</script>

<!-- <div class="row bg-light border-bottom">
    <div class="col h4">{title}</div>
</div> -->

<table class="table table-hover table-sm">
    <thead  class="table-success">
        <tr><th>{title}</th>
            <th class="text-end">Units</th>
            <th class="text-end">SAR</th>
            <th class="text-end">USD</th></tr>
    </thead>
    {#each items as i}
    <tbody>
        <tr><td class="col-70 text-nowrap">{i[0]}</td>
            <td class="col-10 text-end">{i[1]}</td>
            <td class="col-10 text-end">{i[2]}</td>
            <td class="col-10 text-end">{Math.round(Number(i[2]) /3.75)}</td></tr>
    </tbody>
    {/each}
</table>
