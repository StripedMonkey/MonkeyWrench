<script lang="ts">
    import { onMount } from "svelte";

    export let text: String;
    export let type: "Input" | "Output";

    let label: HTMLDivElement;
    onMount(() => {
        if (type === "Input") {
            label.style.textAlign = "Left";
        } else if (type === "Output") {
            label.style.textAlign = "Right";
        }
    });
</script>

<div class="label" bind:this={label}>
    {text}
</div>

<style>
    .label {
        color: white;
        margin-left: 10px;
        margin-right: 10px;


        user-select: none;
        /**This can be replaced with `user-select: none` when Webkit actually supports it*/
        cursor: default; 
        -webkit-user-select: none;
    }
</style>
