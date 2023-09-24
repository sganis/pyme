<script>
	export let item = undefined;	
</script>

<div class="customItem">
    <div class="customItem_title">
        <div class="customer">{item.customer}</div>
    </div>
</div>

<style>
    .customItem {
        display: flex;
        align-items: center;
        cursor: default;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
    }
    .customItem_title {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
    .customer {
        display: inline-block;
        /* margin-right: 10px; */
    }
    .lastname {
        display: inline-block;
    }
    .email {
        display: inline-block;
    }
</style>