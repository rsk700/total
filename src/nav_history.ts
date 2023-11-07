import { derived, writable, type Writable } from "svelte/store";

export class HistoryEntry {
    constructor(
        public globalId: number,
        public name: string,
        public path: string,
    ) { }
}
export let navHistory: Writable<HistoryEntry[]> = writable([]);
export let historyIsEmpty = derived(navHistory, ($h) => $h.length < 2);

export function push(index: number, name: string, path: string) {
    navHistory.update((h) => {
        if (h.length === 0 || (h[h.length - 1].path !== path)) {
            h.push(new HistoryEntry(index, name, path));
        }
        return h;
    });
}

export function pop(): [number, string] | null {
    let out = null;
    navHistory.update((h: HistoryEntry[]) => {
        if (h.length > 1) {
            // skip current one
            h.pop();
            // last one becomes current one
            let last = h[h.length - 1];
            out = [last.globalId, last.path];
        }
        return h;
    });
    return out;
}