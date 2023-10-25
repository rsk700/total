<script lang="ts">
    import { onMount } from "svelte";
    import type { AggregateEntry } from "../ipc/messages/types/structs";
    import { getAggregateData } from "../ipc";

    const upToWidthPx = 50;
    let entries: AggregateEntry[] = [];

    onMount(() => {
        // todo: calculate upToFraction based on window width from upToWidthPx
        // getAggregateData(0.1).then((e) => (entries = e));
        getAggregateData(0.1).then((e) => {console.log(e);entries = e;});
    });
</script>

start
