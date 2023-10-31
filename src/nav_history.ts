import { derived, writable, type Writable } from "svelte/store";

let navHistory: Writable<[number, string][]> = writable([]);
export let historyIsEmpty = derived(navHistory, ($h) => $h.length < 2);

export function push(index: number, path: string) {
    navHistory.update((h) => {
        if (h.length === 0 || (h[h.length - 1][1] !== path)) {
            h.push([index, path]);
        }
        return h;
    });
}

export function pop(): [number, string] | null {
    let out = null;
    navHistory.update((h) => {
        if (h.length > 1) {
            // skip current one
            h.pop();
            // last one becomes current one
            out = h[h.length - 1];
        }
        return h;
    });
    return out;
}