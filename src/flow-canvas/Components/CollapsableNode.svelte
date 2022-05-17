<script lang="ts">
    import { CollapsibleCard } from "svelte-collapsible";
    import NodeContents from "./NodeContents.svelte";

    export let name: string;
    export let data;
</script>

<div class="container">
    <CollapsibleCard>
        <div slot="header" class="header">{name}</div>
        <NodeContents slot="body" {data} />
    </CollapsibleCard>
</div>

<style>
    .header {
        background-color: green;
        color: #e6e6e6;
        border-top-left-radius: 9px;
        border-top-right-radius: 9px;
        padding-left: 10px;


        cursor: default;
        user-select: none;
        -webkit-user-select: none;
    }

    .container {
        position:absolute;
    }
</style>
