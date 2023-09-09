<script lang="ts">
	export let output: string | undefined = undefined;
</script>

<div class="d-flex flex-column text-bg-dark w-100" style="height: 30%;">
	<span class="bg-body-tertiary bg-dark" data-bs-theme="dark"><div class="mx-1">Output</div></span>
	<div class="output-text-area px-1" contenteditable="false" bind:innerText={output} />
</div>

<style>
	.output-text-area {
		background-color: rgb(98, 101, 107);
		color: white;
		height: 100%;
		overflow: auto;
	}
</style>
