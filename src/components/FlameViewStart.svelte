<script lang="ts">
    import type { AggregateData } from "../ipc/messages/types/structs";
    import { getAggregateData } from "../ipc";
    import FlameView from "./FlameView.svelte";
    import Header from "./header/Header.svelte";
    import * as navHistory from "../nav_history";
    import { Subject, auditTime, filter } from "rxjs";
    import { onDestroy } from "svelte";

    export let windowWidth: number;

    const upToWidthPx = 50;
    // big negative value allows first request always to be performed
    let prevSize = -1000;
    let aggregateData: AggregateData | null = null;
    let resizeEvent = new Subject<null>();
    let subOff = resizeEvent
        .pipe(
            // quantize by time
            auditTime(100),
            // quantize by distance (only update aggregated data if resized more
            // than on `upToWidthPx` width)
            filter(() => Math.abs(prevSize - windowWidth) >= upToWidthPx)
        )
        .subscribe(() => {
            prevSize = windowWidth;
            getAggregateData(upToWidthPx / windowWidth).then(updateAggData);
        });

    onDestroy(() => {
        subOff.unsubscribe();
    });

    $: {
        let _ = [windowWidth];
        // updateAggData is separate function in order to break feedback loop,
        // otherwise request is made in cycle infinitly
        resizeEvent.next(null);
    }

    function updateAggData(d: AggregateData) {
        aggregateData = d;
        if (aggregateData.entries.length > 0) {
            let e = aggregateData.entries[0];
            navHistory.push(e.globalId, e.name, e.path);
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
