<script lang="ts">
    import type { AggregateEntry } from "../../ipc/messages/types/structs";
    import { hrByteSize } from "../../text";
    import HeaderAction from "./HeaderAction.svelte";

    export let root: AggregateEntry | null;
</script>

<div class="h-10 flex flex-row flex-nowrap items-center">
    <HeaderAction><ion-icon name="arrow-back" /></HeaderAction>
    <HeaderAction><ion-icon name="arrow-up" /></HeaderAction>
    {#if root !== null}
        <div class="flex justify-center items-center">
            <div class="mx-2 text-2xl">{hrByteSize(root.size)}</div>
        </div>
        <div class="grow min-w-0 basis flex justify-center items-center">
            <div class="mx-2 text-ellipsis whitespace-nowrap overflow-hidden">
                {root.path}
            </div>
        </div>
    {/if}
    <HeaderAction><ion-icon name="refresh" /></HeaderAction>
    <HeaderAction><ion-icon name="folder-open" /></HeaderAction>
    <HeaderAction><ion-icon name="information" /></HeaderAction>
</div>
