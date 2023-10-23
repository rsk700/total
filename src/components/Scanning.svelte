<script lang="ts">
    import { onMount } from "svelte";
    import { scanStep } from "../ipc";
    import * as ms from "../ipc/messages/types";
    import { AppState, appState } from "../app_state";

    const scanStepTimeBudgetMs = 500;
    let stopScan = false;
    let doneCount = 0;

    onMount(() => {
        runScanStep();
        return () => {
            stopScan = true;
        };
    });

    function runScanStep() {
        if (stopScan) {
            return;
        }
        scanStep(scanStepTimeBudgetMs).then((s) => {
            if (s instanceof ms.variants.ScanState.ScanProgress) {
                doneCount = s.doneCount;
                runScanStep();
            } else if (s instanceof ms.variants.ScanState.Ready) {
                appState.set(AppState.FlameView);
            }
        });
    }
</script>

<div class="absolute inset-0 flex items-center">
    <div class="grow flex justify-center">
        <div class="rounded-[0.625rem] bg-green-200 w-1/3 h-5 mt-12" />
    </div>
</div>

<div class="absolute inset-0 table text-2xl">
    <div
        class="grow text-right overflow-hidden text-ellipsis whitespace-nowrap"
    >
        3111
    </div>
    <div class="shrink px-2">/</div>
    <div class="grow text-left overflow-hidden text-ellipsis whitespace-nowrap">
        {doneCount}
    </div>
</div>

<style>
    .table {
        display: grid;
        grid-template-columns: 1fr auto 1fr;
        align-items: center;
    }
</style>
