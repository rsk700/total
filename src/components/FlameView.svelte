<script lang="ts">
  import type { AggregateEntry } from "../ipc/messages/types/structs";
  import { openPath, jump } from "../ipc";
  import { AppState, appState } from "../app_state";
  import { hrByteSize, hrCount } from "../text";
  import { lerp, lerpc } from "../numbers";

  // todo: use level (for height)

  const colorSteps = 4 * 2;

  export let index: number;
  export let entries: AggregateEntry[];
  export let level: number;
  export let colorShift: number;

  let startIndex = 0;
  let viewGrid: HTMLElement | null = null;

  $: if (entries.length > 0) {
    startIndex = entries[0].nameHash % colorSteps;
  }

  $: if (viewGrid !== null) {
    let template: string[] = [];
    entries[index].nested.forEach((ni) => {
      template.push(`${entries[ni].size}fr`);
    });
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

  function colorBlock(
    node: HTMLElement,
    params: { e: AggregateEntry; i: number }
  ) {
    if (params.e.isFile) {
      let bg = "#cef4db";
      let hover = "#f0fdf4";
      node.style.backgroundColor = bg;
      node.addEventListener(
        "mouseover",
        () => (node.style.backgroundColor = hover)
      );
      node.addEventListener(
        "mouseleave",
        () => (node.style.backgroundColor = bg)
      );
    } else {
      let ix = colorShift;
      if (level === 0) {
        ix += startIndex + params.i;
      }
      // shuffling like this, in order to avoid similar colors touching each other:
      // 0 1 2 3, 4 5 6 7
      // 1 3 0 2, 5 7 4 6
      let ixShuffle = ix % 4;
      if (ixShuffle === 0) {
        ix += 2;
      } else if (ixShuffle === 1) {
        ix -= 1;
      } else if (ixShuffle === 2) {
        ix += 1;
      } else if (ixShuffle === 3) {
        ix -= 2;
      }
      let h = lerp(0, 264, (ix % colorSteps) / colorSteps);
      let s = lerpc(86, 56, level / 4);
      let l = lerpc(60, 75, level / 4);
      let bg = `hsl(${h} ${s}% ${l}%)`;
      let bgHover = `hsl(${h} ${s + 4}% ${l + 10}%)`;
      node.style.backgroundColor = bg;
      node.addEventListener(
        "mouseover",
        () => (node.style.backgroundColor = bgHover)
      );
      node.addEventListener(
        "mouseleave",
        () => (node.style.backgroundColor = bg)
      );
    }
  }
</script>

<!-- todo: try gradient for bg dark -> light (or reverse) -->
<div bind:this={viewGrid} class="grid-view h-full bg-[#082043]">
  {#each entries[index].nested as ni, i}
    {@const e = entries[ni]}
    <div
      on:click={(event) => clickEntry(event, e)}
      on:contextmenu={(event) => contextMenu(event, e)}
      on:keyup={(event) => keyUpEntry(event, e)}
      role="button"
      tabindex="0"
      use:colorBlock={{ e, i }}
      class="relative min-w-0 whitespace-nowrap overflow-hidden"
    >
      <div
        class="invisible absolute inset-0 flex justify-center items-center text-green-400 text-xl font-bold"
      >
        {hrByteSize(e.size)}
      </div>
      <div
        class="invisible absolute inset-0 text-center p-1 text-sm text-green-900 text-ellipsis whitespace-nowrap overflow-hidden"
      >
        {e.name}
      </div>
      {#if !e.isFile}
        <div
          class="absolute inset-x-0 bottom-0 flex justify-center p-1 text-xs text-green-700"
        >
          <div
            class="invisible text-ellipsis whitespace-nowrap overflow-hidden"
          >
            f{hrCount(e.fileCount)}, d{hrCount(e.dirCount)}
          </div>
        </div>
      {/if}
    </div>
    <div class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden">
      <svelte:self
        index={ni}
        {entries}
        level={level + 1}
        colorShift={level === 0 ? startIndex + colorShift + i : colorShift}
      />
    </div>
  {/each}
  {#if entries[index].tailSize > 0}
    <div
      on:click={(event) => clickEntry(event, entries[index])}
      on:contextmenu={(event) => contextMenu(event, entries[index])}
      on:keyup={(event) => keyUpEntry(event, entries[index])}
      role="button"
      tabindex="0"
      class="relative bg-purple-400 hover:bg-purple-300 min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    >
      <div
        class="invisible absolute inset-0 flex justify-center items-center text-xl font-bold text-purple-600"
      >
        {hrByteSize(entries[index].tailSize)}
      </div>
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
