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

<div class="h-full flex justify-center items-center">
    <div
        class="done-counter-text overflow-hidden text-ellipsis whitespace-nowrap"
    >
        {doneCount}
    </div>
</div>

<style>
    .done-counter-text {
        font-size: max(7vmin, 2rem);
    }
</style>
