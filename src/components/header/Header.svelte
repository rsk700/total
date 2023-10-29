<script lang="ts">
    import { dialog } from "@tauri-apps/api";
    import type { AggregateEntry } from "../../ipc/messages/types/structs";
    import { hrByteSize } from "../../text";
    import HeaderAction from "./HeaderAction.svelte";
    import { startScan } from "../../ipc";
    import { AppState, appState } from "../../app_state";

    export let root: AggregateEntry | null;

    async function choosePath() {
        let path = await dialog.open({
            directory: true,
            title: "Choose directory for scanning",
        });
        if (typeof path === "string") {
            await startScan(path);
            appState.set(AppState.Scanning);
        }
    }
</script>

<div class="h-10 flex flex-row flex-nowrap items-center">
    <HeaderAction><ion-icon name="play-back" /></HeaderAction>
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
    <HeaderAction><button on:click={choosePath}><ion-icon name="folder-open" /></button></HeaderAction
    >
    <HeaderAction><ion-icon name="information" /></HeaderAction>
</div>
