<script lang="ts">
    import type { AggregateEntry } from "../ipc/messages/types/structs";
    import { getAggregateData } from "../ipc";
    import FlameView from "./FlameView.svelte";
    import Header from "./header/Header.svelte";
    import * as navHistory from "../nav_history";

    export let windowWidth: number;

    // todo: quantize upToFraction into steps
    const upToWidthPx = 50;
    let entries: AggregateEntry[] = [];

    // todo: allow only one request at a time + debounce

    // updateEntries is separate function in order to break feedback loop,
    // otherwise request is made in cycle infinitly
    $: getAggregateData(upToWidthPx / windowWidth).then(updateEntries);

    function updateEntries(e: AggregateEntry[]) {
        entries = e;
        if (entries.length > 0) {
            navHistory.push(entries[0].globalId, entries[0].path);
        }
    }
</script>

<Header root={entries[0] ?? null} />
{#if entries.length !== 0}
    <FlameView index={0} {entries} />
{/if}
