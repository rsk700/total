<script lang="ts">
  import type { AggregateEntry } from "../ipc/messages/types/structs";

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
</script>

<div bind:this={viewGrid} class="grid-view h-full bg-purple-700">
  {#each entries[index].nested as ni}
    {@const e = entries[ni]}
    <div
      class:bg-green-300={!e.isFile}
      class:bg-green-100={e.isFile}
      class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden text-xs leading-3"
    >
      {e.name}
      <div>ss {e.selfSize}</div>
      <div>s {e.size}</div>
      <div>sfc {e.selfFileCount}</div>
      <div>fc {e.fileCount}</div>
      <div>sdc {e.selfDirCount}</div>
      <div>dc {e.dirCount}</div>
    </div>
    <div class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden">
      {#if e.nested.length !== 0}
        <svelte:self index={ni} {entries} />
      {/if}
    </div>
  {/each}
  {#if entries[index].tailSize > 0}
    <div
      class="bg-purple-500 min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    >
      {entries[index].tailSize}
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
