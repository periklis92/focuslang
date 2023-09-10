<script lang="ts">
	import CodeEditor from '$lib/components/CodeEditor.svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Output from '$lib/components/Output.svelte';
	import SideBar from '$lib/components/Sidebar.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	import('$lib/assets/test.html?raw').then((res) => console.log(res));

	let output: Output;
	let sidebar: SideBar;
	let source: string;

	function runCode() {
		if (!source) {
			return;
		}

		try {
			console.debug('Executing code...');
			let result = data.interpreter.interpret_str_web(source);
			console.log(result);
			const log = JSON.stringify(result, (_, v) => (typeof v === 'bigint' ? Number(v) : v));
			output.log(log);
			console.log(result);
			console.debug('Code executed successfully.');
		} catch (err: any) {
			output.log(err);
		}
	}
</script>

<main
	class="d-flex flex-nowrap"
	style="height: 100vh; max-height: 100vh; overflow-x: auto; overflow-y: hidden;"
>
	<SideBar
		bind:this={sidebar}
		on:selected={(event) => {
			sidebar.setContent(`<h1>${event.detail}</h1>`);
		}}
		menu={[
			{
				title: 'Tutorials',
				icon: 'bi bi-book',
				items: [{ title: '01. Basics', id: 'tutorial-01' }]
			},

			{
				title: 'Examples',
				icon: 'bi bi-journals',
				items: [{ title: '01. Fibonnachi', id: 'example-01' }]
			}
		]}
	/>

	<div class="d-flex flex-column w-100">
		<Nav on:run={runCode} />
		<CodeEditor bind:source />

		<Output bind:this={output} />
	</div>
</main>
