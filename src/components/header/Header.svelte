<script lang="ts">
    import { dialog } from "@tauri-apps/api";
    import type { AggregateEntry } from "../../ipc/messages/types/structs";
    import { hrByteSize } from "../../text";
    import HeaderAction from "./HeaderAction.svelte";
    import * as ipc from "../../ipc";
    import { AppState, appState } from "../../app_state";
    import { historyIsEmpty, pop as popHistory } from "../../nav_history";

    export let root: AggregateEntry | null;

    async function choosePath() {
        let path = await dialog.open({
            directory: true,
            title: "Choose directory for scanning",
        });
        if (typeof path === "string") {
            await ipc.startScan(path);
            appState.set(AppState.Scanning);
        }
    }

    async function navigateLevelUp() {
        await ipc.levelUp();
        appState.set(AppState.Scanning);
    }

    function openRoot() {
        if (root !== null) {
            ipc.openPath(root.path);
        }
    }

    async function rescan() {
        await ipc.rescan();
        appState.set(AppState.Scanning);
    }

    async function navBack() {
        let h = popHistory();
        if (h !== null) {
            let [id, path] = h;
            await ipc.navigate(id, path);
            appState.set(AppState.Scanning);
        }
    }
</script>

<div class="h-10 flex flex-row flex-nowrap items-center">
    <HeaderAction
        ><button
            on:click={navBack}
            disabled={$historyIsEmpty}
            class="disabled:text-slate-400"
            ><ion-icon name="play-back" /></button
        ></HeaderAction
    >
    <HeaderAction
        ><button on:click={navigateLevelUp}><ion-icon name="arrow-up" /></button
        ></HeaderAction
    >
    <div class="flex justify-center items-center">
        <div class="mx-2 text-2xl">{hrByteSize(root?.size ?? 0)}</div>
    </div>
    <div class="grow min-w-0 basis flex justify-center items-center">
        <div class="mx-2 text-ellipsis whitespace-nowrap overflow-hidden">
            <button on:click={openRoot}>{root?.path ?? "?"}</button>
        </div>
    </div>
    <HeaderAction
        ><button on:click={rescan}><ion-icon name="refresh" /></button
        ></HeaderAction
    >
    <HeaderAction
        ><button on:click={choosePath}><ion-icon name="folder-open" /></button
        ></HeaderAction
    >
    <HeaderAction><ion-icon name="information" /></HeaderAction>
</div>
