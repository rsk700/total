<script lang="ts">
    import type { AggregateData } from "../ipc/messages/types/structs";
    import { getAggregateData } from "../ipc";
    import FlameView from "./FlameView.svelte";
    import Header from "./header/Header.svelte";
    import * as navHistory from "../nav_history";

    export let windowWidth: number;

    // todo: quantize upToFraction into steps
    const upToWidthPx = 50;
    let aggregateData: AggregateData | null = null;

    // todo: allow only one request at a time + debounce

    // updateEntries is separate function in order to break feedback loop,
    // otherwise request is made in cycle infinitly
    $: getAggregateData(upToWidthPx / windowWidth).then(updateAggData);

    function updateAggData(d: AggregateData) {
        aggregateData = d;
        if (aggregateData.entries.length > 0) {
            navHistory.push(
                aggregateData.entries[0].globalId,
                aggregateData.entries[0].path
            );
        }
    }
</script>

{#if aggregateData !== null}
    <Header
        pathTop={aggregateData.pathTop}
        pathComponents={aggregateData.pathComponents}
        pathSeparator={aggregateData.pathSeparator}
    />
    {#if aggregateData.entries.length !== 0}
        <FlameView
            index={0}
            entries={aggregateData.entries}
            level={0}
            colorShift={0}
        />
    {/if}
{/if}
