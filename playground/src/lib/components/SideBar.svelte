<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
</script>

<div
	class="d-flex flex-column flex-shrink-0 bg-body-tertiary bg-dark"
	data-bs-theme="dark"
	style="width: 4.5rem;"
>
	<ul class="nav nav-pills nav-flush flex-column mb-auto text-center">
		<li class="nav-item">
			<a
				href="/"
				class="nav-link border-bottom rounded-0 py-3"
				aria-current="page"
				on:click|preventDefault={() => dispatch('run')}
			>
				<i class="bi bi-play fs-4" />
			</a>
		</li>
		<li class="nav-item">
			<a
				href="/"
				class="nav-link border-bottom rounded-0 py-3"
				aria-current="page"
				on:click|preventDefault={() => dispatch('clear')}
			>
				<i class="bi bi-x fs-4" />
			</a>
		</li>
	</ul>
</div>

<div class="divider" />

<style>
	.divider {
		flex-shrink: 0;
		width: 0.5rem;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.65);
		border: solid rgba(0, 0, 0, 0.15);
		border-width: 1px 0;
		box-shadow: inset 0 0.5em 1.5em rgba(0, 0, 0, 0.1), inset 0 0.125em 0.5em rgba(0, 0, 0, 0.15);
	}
</style>
