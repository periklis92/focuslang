<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
</script>

<nav class="navbar navbar-expand-lg bg-body-tertiary">
	<div class="container-fluid">
		<a class="navbar-brand" href="/" style="margin-left: 100px;">Focus Lang</a>
		<ul class="nav nav-pills mb-auto text-center mr-0 ml-auto">
			<li class="nav-item">
				<button
					class="nav-link rounded-0"
					aria-current="page"
					on:click|preventDefault={() => dispatch('run')}
				>
					<i class="bi bi-play fs-2" />
				</button>
			</li>
		</ul>
	</div>
</nav>
