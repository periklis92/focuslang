import type { PageLoad } from './$types';
import init, { Interpreter } from '$lib/interpreter.js';

export const load = (async () => {
    await init();

    return {
        interpreter: Interpreter.new()
    };
}) satisfies PageLoad;