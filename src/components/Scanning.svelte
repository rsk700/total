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

    function cancelScan() {
        appState.set(AppState.ChoosePath);
    }
</script>

<div class="relative h-full w-full">
    <div class="absolute inset-0 flex justify-center items-center">
        <div
            class="done-counter-text overflow-hidden text-ellipsis whitespace-nowrap text-slate-300"
        >
            {doneCount}
        </div>
    </div>

    <div class="absolute inset-0 flex justify-center items-center">
        <button
            on:click={cancelScan}
            class="
                block
                border
                rounded
                px-2
                text-sm
                mt-24
                border-green-100
                hover:bg-green-100
                active:bg-green-300
                active:text-green-900
                ">cancel</button
        >
    </div>
</div>

<style>
    .done-counter-text {
        font-size: max(7vmin, 2rem);
    }
</style>
