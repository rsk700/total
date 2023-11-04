<script lang="ts">
  import type { AggregateEntry } from "../ipc/messages/types/structs";
  import { openPath, jump } from "../ipc";
  import { AppState, appState } from "../app_state";
  import { hrByteSize, hrCount } from "../text";
  import { lerp, lerpc } from "../numbers";
  import { Hsl } from "../color";

  const colorSteps = 4 * 2;
  const fileBg = new Hsl(141, 63, 88);

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

  function baseColor(i: number): Hsl {
    let ix = colorShift;
    if (level === 0) {
      ix += startIndex + i;
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
    return new Hsl(h, s, l);
  }

  function colorBlock(
    node: HTMLElement,
    params: { e: AggregateEntry; i: number }
  ) {
    if (params.e.isFile) {
      let bg = fileBg.toString();
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
      let bg = baseColor(params.i);
      let bgHover = bg.ds(4).dl(10);
      node.style.backgroundColor = bg.toString();
      node.addEventListener(
        "mouseover",
        () => (node.style.backgroundColor = bgHover.toString())
      );
      node.addEventListener(
        "mouseleave",
        () => (node.style.backgroundColor = bg.toString())
      );
    }
  }
</script>

<!-- todo: try gradient for bg dark -> light (or reverse) -->
<div
  bind:this={viewGrid}
  style:grid-template-rows={`${lerpc(100, 70, level / 5)}px auto`}
  class="grid-view h-full"
>
  {#each entries[index].nested as ni, i}
    {@const e = entries[ni]}
    {@const bc = baseColor(i)}
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
        style:color={e.isFile
          ? fileBg.dl(-15).ds(-20).toString()
          : bc.dl(-10).ds(-20).toString()}
        class="absolute inset-0 flex justify-center items-center text-xl font-bold"
      >
        {hrByteSize(e.size)}
      </div>
      <div
        style:color={e.isFile
          ? fileBg.dl(-50).ds(-20).toString()
          : bc.dl(-40).ds(-20).toString()}
        class="absolute inset-0 text-center p-1 text-sm text-ellipsis whitespace-nowrap overflow-hidden"
      >
        {e.name}
      </div>
      {#if !e.isFile}
        <div
          class="absolute inset-x-0 bottom-0 flex justify-center p-1 text-xs"
        >
          <div
            style:color={bc.dl(-25).ds(-10).toString()}
            class="text-ellipsis whitespace-nowrap overflow-hidden"
          >
            <ion-icon name="document-outline" />
            {hrCount(e.fileCount)}
            <ion-icon name="folder-outline" />
            {hrCount(e.dirCount)}
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
        class="absolute inset-0 flex justify-center items-center text-xl font-bold text-purple-500"
      >
        {hrByteSize(entries[index].tailSize)}
      </div>
      <div class="absolute inset-x-0 bottom-0 flex justify-center p-1 text-xs">
        <div
          class="text-ellipsis whitespace-nowrap overflow-hidden text-purple-600"
        >
          <ion-icon name="document-outline" />
          {hrCount(entries[index].tailFileCount)}
          <ion-icon name="folder-outline" />
          {hrCount(entries[index].tailDirCount)}
        </div>
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
    /* height is based on nested level, applied using `style` directive */
    /* grid-template-rows: 100px auto; */
    gap: 1px;
    grid-auto-flow: column;
  }
</style>
