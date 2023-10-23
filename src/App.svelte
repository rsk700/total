<script lang="ts">
  import { AppState, appState } from "./app_state";
  import ChoosePath from "./components/ChoosePath.svelte";
  import FlameView from "./components/FlameView.svelte";
  import Scanning from "./components/Scanning.svelte";
</script>

<div class="absolute inset-0 bg-purple-300">
  {#if $appState === AppState.ChoosePath}
    <ChoosePath />
  {:else if $appState === AppState.Scanning}
    <Scanning />
  {:else if $appState === AppState.FlameView}
    <FlameView step={4} />
  {:else}
    error: unknown state
  {/if}
</div>
