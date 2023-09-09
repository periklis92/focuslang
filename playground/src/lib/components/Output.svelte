<script lang="ts">
	export let output: string[] = [];
</script>

<div class="d-flex flex-column text-bg-dark w-100" style="height: 30%;">
	<span class="bg-body-tertiary bg-dark" data-bs-theme="dark"><div class="mx-1">Output</div></span>
	<div class="output-text-area px-1" contenteditable="false">
		{#each output as line}
			<div class="border-bottom border-dark w-100 mb-1">
				{line}
			</div>
		{/each}
	</div>
</div>

<style>
	.output-text-area {
		background-color: rgb(84, 86, 90);
		color: white;
		height: 100%;
		overflow: auto;
	}
</style>
