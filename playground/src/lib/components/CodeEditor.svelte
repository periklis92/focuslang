<script lang="ts">
	export let source: string | undefined = undefined;
</script>

<div class="d-flex text-bg-dark w-100 px-1" style="height: 85%;">
	<div
		contenteditable="true"
		class="code-editor"
		bind:innerText={source}
		placeholder="Insert code here..."
	/>
</div>

<style>
	.code-editor {
		font-size: larger;
		border: 0;
		width: 100%;
		height: 100%;
		color: white;
		outline: 0;
		resize: none;
		padding: 0;
		margin: 0;
		font-family: var(--bs-font-monospace);
		font-weight: var(--bs-body-font-weight);
		line-height: var(--bs-body-line-height);
		font-size: 0.975rem;
	}
</style>
