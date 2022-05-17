<div class="flow-canvas">
    <slot />
</div>

<style>
    .flow-canvas {
        overflow: hidden;
        flex: 1;

        /* Creates a dotted background */
        background-image: radial-gradient(#282828 3px, transparent 0);
        background-size: 40px 40px;
        /* background-position: 2048px 2048px; */
        margin: 1.5%;
        background-color: #1d1d1d;
        border: white dotted 0.5px;
    }
</style>
