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
      class="bg-green-300 min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    >
      {e.name}
    </div>
    <div class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden">
      {#if e.nested.length !== 0}
        <svelte:self index={ni} {entries} />
      {/if}
    </div>
  {/each}
  {#if entries[index].tailSize > 0}
    <div
      class="bg-green-800 min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    />
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
