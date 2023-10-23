<script lang="ts">
    import { dialog } from "@tauri-apps/api";
    import { AppState, appPath, appState } from "../app_state";
    import { startScan } from "../ipc";

    let showButton = true;

    async function choosePath() {
        let path = await dialog.open({
            directory: true,
            title: "Choose directory for scanning",
        });
        if (typeof path === "string") {
            showButton = false;
            await startScan(path);
            appPath.set(path);
            appState.set(AppState.Scanning);
        }
    }
</script>

{#if showButton}
    <div class="h-full flex justify-center items-center">
        <button
            on:click={choosePath}
            class="
    border-2
    rounded
    p-2
    text-2xl
    border-green-100
    hover:bg-green-100
    active:bg-green-300
    active:text-green-900
    ">choose path</button
        >
    </div>
{/if}
