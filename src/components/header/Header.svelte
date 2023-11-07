<script lang="ts">
    import { dialog } from "@tauri-apps/api";
    import type { PathComponent } from "../../ipc/messages/types/structs";
    // import { hrByteSize } from "../../text";
    import HeaderAction from "./HeaderAction.svelte";
    import * as ipc from "../../ipc";
    import { AppState, appState } from "../../app_state";
    import {
        historyIsEmpty,
        navHistory,
        pop as popHistory,
    } from "../../nav_history";
    import About from "../About.svelte";
    import HeaderButton from "./HeaderButton.svelte";
    import PathButton from "./PathButton.svelte";
    import { derived } from "svelte/store";

    export let pathTop: string;
    export let pathComponents: PathComponent[];
    export let pathSeparator: string;

    let showAbout = false;
    let isPathLeftHidden = false;
    const backTitle = derived(navHistory, ($h) => {
        if ($h.length > 1) {
            let prev = $h[$h.length - 2];
            return `back to "${prev.name}"`;
        } else {
            return "";
        }
    });

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

<About bind:show={showAbout} />
<div
    class="h-10 border-b border-[#929fcc] flex flex-row flex-nowrap items-center bg-[#b3bfee]"
>
    <HeaderAction>
        <HeaderButton
            on:click={navBack}
            disabled={$historyIsEmpty}
            title={$backTitle}
        >
            <ion-icon name="play-back" class="block h-full" />
        </HeaderButton>
    </HeaderAction>
    <HeaderAction
        ><HeaderButton on:click={navigateLevelUp} title="open parent folder"
            ><ion-icon name="arrow-up" class="block h-full" /></HeaderButton
        >
    </HeaderAction>
    <!-- <div class="flex justify-center items-center">
        <div class="mx-2 text-2xl">{hrByteSize(root?.size ?? 0)}</div>
    </div> -->
    <div
        style:justify-content={isPathLeftHidden ? "end" : "center"}
        class="grow min-w-0 basis flex items-center h-full overflow-hidden"
    >
        <PathButton
            {pathTop}
            components={pathComponents}
            separator={pathSeparator}
            bind:isLeftHidden={isPathLeftHidden}
        />
    </div>
    <HeaderAction>
        <HeaderButton on:click={rescan} title="rescan folder">
            <ion-icon name="refresh" class="block h-full" />
        </HeaderButton>
    </HeaderAction>
    <HeaderAction>
        <HeaderButton on:click={choosePath} title="open another folder">
            <ion-icon name="folder-open" class="block h-full" />
        </HeaderButton>
    </HeaderAction>
    <HeaderAction>
        <HeaderButton
            on:click={() => (showAbout = true)}
            title="about"
        >
            <ion-icon name="information" class="block h-full" />
        </HeaderButton>
    </HeaderAction>
</div>
