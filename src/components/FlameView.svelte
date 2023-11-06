<script lang="ts">
  import type { AggregateEntry } from "../ipc/messages/types/structs";
  import { openPath, jump } from "../ipc";
  import { AppState, appState } from "../app_state";
  import { hrByteSize, hrCount } from "../text";
  import { lerp, lerpc } from "../numbers";
  import { Oklch } from "../color";

  const colorSteps = 4 * 2;
  const fileBg = new Oklch(93.27, 13.25, 157.11);
  const tailBg = new Oklch(72.17, 44.25, 305.5);

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

  function baseColor(i: number): Oklch {
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
    let l = lerpc(75.3, 80.25, level / 4);
    // todo: try start from more chroma?
    let c = lerpc(33.5, 14.5, level / 4);
    // todo: later remove shift?
    let testShift = 33;
    let h = lerp(
      0 + testShift,
      264 + testShift,
      (ix % colorSteps) / colorSteps
    );
    return new Oklch(l, c, h);
  }

  // using function because can't use generated color in hover:
  function colorBlock(
    node: HTMLElement,
    params: { e: AggregateEntry; i: number }
  ) {
    if (params.e.isFile) {
      let bg = fileBg.toString();
      let hover = fileBg.dc(2).dl(6).toString();
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
      let bgHover = bg.dc(2).dl(6);
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
          ? fileBg.dl(-10).dc(5).toString()
          : bc.dl(-9).dc(-1).toString()}
        class="absolute inset-0 flex justify-center items-center text-xl font-bold"
      >
        {hrByteSize(e.size)}
      </div>
      <div
        style:color={e.isFile
          ? fileBg.dl(-35).dc(15).toString()
          : bc.dl(-36).dc(-3).toString()}
        class="absolute inset-0 text-center p-1 text-sm text-ellipsis whitespace-nowrap overflow-hidden"
      >
        {e.name}
      </div>
      {#if !e.isFile}
        <div
          class="absolute inset-x-0 bottom-0 flex justify-center p-1 text-xs"
        >
          <div
            style:color={bc.dl(-17).dc(5).toString()}
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
      class="relative bg-[#c084fc] hover:bg-[#cda0ff] min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    >
      <div
        style:color={tailBg.dl(-10).dc(5).toString()}
        class="absolute inset-0 flex justify-center items-center text-xl font-bold"
      >
        {hrByteSize(entries[index].tailSize)}
      </div>
      <div class="absolute inset-x-0 bottom-0 flex justify-center p-1 text-xs">
        <div
          style:color={tailBg.dl(-17).dc(5).toString()}
          class="text-ellipsis whitespace-nowrap overflow-hidden"
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
