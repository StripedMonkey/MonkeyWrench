<script lang="ts">
	import CollapsableNode from "./flow-canvas/Components/CollapsableNode.svelte";
	import FlowCanvas from "./flow-canvas/FlowCanvas.svelte";
</script>

<main>
	<h1>Header!</h1>
	<FlowCanvas>
		<CollapsableNode
			name="Testing"
			data={{
				"First Item": "First content",
				"Second Item": "Second Content",
				"Third Item": "Third Content",
			}}
		/>
	</FlowCanvas>
</main>

<style>
	:global(body, html) {
		background-color: #1d1d1d;
	}

	main {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	h1 {
		text-align: center;
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 2em;
		font-weight: 100;
	}
</style>
