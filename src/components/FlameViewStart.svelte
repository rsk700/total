<script lang="ts">
    import type { AggregateEntry } from "../ipc/messages/types/structs";
    import { getAggregateData } from "../ipc";
    import FlameView from "./FlameView.svelte";

    export let windowWidth: number;

    // todo: quantize upToFraction into steps
    const upToWidthPx = 50;
    let entries: AggregateEntry[] = [];

    // todo: allow only one request at a time + debounce
    $: getAggregateData(upToWidthPx / windowWidth).then((e) => (entries = e));
</script>

{#if entries.length !== 0}
    <FlameView index={0} {entries} />
{/if}
