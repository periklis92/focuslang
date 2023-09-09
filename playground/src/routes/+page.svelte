<script lang="ts">
	import CodeEditor from '$lib/components/CodeEditor.svelte';
	import Nav from '$lib/components/Nav.svelte';
	import Output from '$lib/components/Output.svelte';
	import SideBar from '$lib/components/SideBar.svelte';
	import type { PageData } from './$types';
	export let data: PageData;

	let source: string;
	let output: string | undefined;

	function runCode() {
		if (!source) {
			return;
		}

		try {
			console.debug('Executing code...');
			let result = data.interpreter.interpret_str_web(source);
			console.log(result);
			output += JSON.stringify(result, (_, v) => (typeof v === 'bigint' ? Number(v) : v)) + '\n';
			console.log(result);
			console.debug('Code executed successfully.');
		} catch (err: any) {
			output += err + '\n';
		}
	}
</script>

<main
	class="d-flex flex-nowrap"
	style="height: 100vh; max-height: 100vh; overflow-x: auto; overflow-y: hidden;"
>
	<SideBar on:run={runCode} on:clear={() => (source = '')} />

	<div class="d-flex flex-column w-100">
		<Nav />
		<CodeEditor bind:source />

		<Output bind:output />
	</div>
</main>
