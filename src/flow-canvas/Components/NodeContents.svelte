<script lang="ts">
    import SimpleLabel from "./NodeContentTypes/SimpleLabel.svelte";

    export let data;
    console.log(data);
</script>

<div class="container">
    {#each Object.entries(data) as [key, item]}
        <SimpleLabel text={item} type="Input"/>
    {/each}
</div>

<style>
    .container {
        flex: content;
        background-color: #303030;
        border: grey 0.5px;
        border-bottom-left-radius: 9px;
        border-bottom-right-radius: 9px;

    }

    .item {
        border: white 0.5px;
    }
</style>
