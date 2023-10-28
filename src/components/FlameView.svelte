<script lang="ts">
  import type { AggregateEntry } from "../ipc/messages/types/structs";
  import { openPath, jump } from "../ipc";
  import { AppState, appState } from "../app_state";
  import { hrByteSize, hrCount } from "../text";

  // todo: export level (for height)
  // todo: export shift (for colors)

  export let index: number;
  export let entries: AggregateEntry[];

  let viewGrid: HTMLElement | null = null;

  $: if (viewGrid !== null) {
    let template: string[] = [];
    entries[index].nested.forEach((ni) => {
      template.push(`${entries[ni].size}fr`);
    });
    // todo: add threshold to tailSize (don't show if very narrow or set min width)
    if (entries[index].tailSize > 0) {
      template.push(`${entries[index].tailSize}fr`);
    }
    viewGrid.style.gridTemplateColumns = template.join(" ");
  }

  function clickEntry(e: MouseEvent, entry: AggregateEntry) {
    if (e.button === 0) {
      jumpInside(entry);
    } else if (e.button === 1) {
      openDir(entry);
    }
  }

  function keyUpEntry(e: KeyboardEvent, entry: AggregateEntry) {
    if (e.key === " ") {
      jumpInside(entry);
    }
  }

  async function jumpInside(entry: AggregateEntry) {
    if (entry.isFile && entry.localParent !== null) {
      await jump(entries[entry.localParent].globalId);
    } else {
      await jump(entry.globalId);
    }
    appState.set(AppState.Scanning);
  }

  function contextMenu(e: Event, entry: AggregateEntry) {
    e.preventDefault();
    e.stopPropagation();
    openDir(entry);
  }

  function openDir(entry: AggregateEntry) {
    if (entry.isFile && entry.localParent !== null) {
      openPath(entries[entry.localParent].path);
    } else {
      openPath(entry.path);
    }
  }
</script>

<div bind:this={viewGrid} class="grid-view h-full bg-purple-700">
  {#each entries[index].nested as ni}
    {@const e = entries[ni]}
    <div
      on:click={(event) => clickEntry(event, e)}
      on:contextmenu={(event) => contextMenu(event, e)}
      on:keyup={(event) => keyUpEntry(event, e)}
      role="button"
      tabindex="0"
      class:bg-green-300={!e.isFile}
      class:bg-green-100={e.isFile}
      class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden text-xs leading-3"
    >
      {e.name}
      <div>ss {hrByteSize(e.selfSize)}</div>
      <div>s {hrByteSize(e.size)}</div>
      <div>sfc {hrCount(e.selfFileCount)}</div>
      <div>fc {hrCount(e.fileCount)}</div>
      <div>sdc {hrCount(e.selfDirCount)}</div>
      <div>dc {hrCount(e.dirCount)}</div>
    </div>
    <div class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden">
      {#if e.nested.length !== 0}
        <svelte:self index={ni} {entries} />
      {/if}
    </div>
  {/each}
  {#if entries[index].tailSize > 0}
    <div
      on:click={(event) => clickEntry(event, entries[index])}
      on:contextmenu={(event) => contextMenu(event, entries[index])}
      on:keyup={(event) => keyUpEntry(event, entries[index])}
      role="button"
      tabindex="0"
      class="bg-purple-500 min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    >
      {hrByteSize(entries[index].tailSize)}
    </div>
    <div class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden" />
  {/if}
</div>

<style>
  .grid-view {
    justify-items: stretch;
    align-items: stretch;
    display: grid;
    /* todo: height based on nested level */
    grid-template-rows: 100px auto;
    gap: 1px;
    grid-auto-flow: column;
  }
</style>
