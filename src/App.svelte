<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { AppState, appState } from "./app_state";
  import ChoosePath from "./components/ChoosePath.svelte";
  import FlameViewStart from "./components/FlameViewStart.svelte";
  import Scanning from "./components/Scanning.svelte";
  import { onDestroy } from "svelte";

  let windowWidth = 800;

  let subOff: (() => void)[] = [];
  appWindow.innerSize().then((s) => (windowWidth = s.width));
  appWindow
    .onResized(({ payload }) => {
      windowWidth = payload.width;
    })
    .then((off) => subOff.push(off));

  onDestroy(() => {
    subOff.forEach((off) => off());
  });

  function ignore(e: Event) {
    if (import.meta.env.PROD) {
      e.preventDefault();
      e.stopPropagation();
    }
  }
</script>

<svelte:window on:contextmenu={ignore} />

<div class="absolute inset-0 bg-[#082043]">
  {#if $appState === AppState.ChoosePath}
    <ChoosePath />
  {:else if $appState === AppState.Scanning}
    <Scanning />
  {:else if $appState === AppState.FlameView}
    <FlameViewStart {windowWidth} />
  {:else}
    error: unknown state
  {/if}
</div>
