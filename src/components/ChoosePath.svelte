<script lang="ts">
    import { dialog } from "@tauri-apps/api";
    import { AppState, appState } from "../app_state";
    import { startScan } from "../ipc";
    import BigButtonLight from "./BigButtonLight.svelte";

    let showButton = true;

    async function choosePath() {
        let path = await dialog.open({
            directory: true,
            title: "Choose directory for scanning",
        });
        if (typeof path === "string") {
            showButton = false;
            await startScan(path);
            appState.set(AppState.Scanning);
        }
    }
</script>

{#if showButton}
    <div class="h-full flex justify-center items-center">
        <BigButtonLight on:click={choosePath}>choose path</BigButtonLight>
    </div>
{/if}
