<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
</script>

<nav class="navbar navbar-expand-lg bg-body-tertiary">
	<div class="container-fluid">
		<button
			class="navbar-toggler"
			type="button"
			data-bs-toggle="collapse"
			data-bs-target="#navbarTogglerDemo01"
			aria-controls="navbarTogglerDemo01"
			aria-expanded="false"
			aria-label="Toggle navigation"
		>
			<span class="navbar-toggler-icon" />
		</button>
		<div class="collapse navbar-collapse" id="navbarTogglerDemo01">
			<a class="navbar-brand" href="/">Focus Lang</a>
			<ul class="navbar-nav me-auto mb-2 mb-lg-0" />
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
	</div>
</nav>
