<script lang="ts">
  export let step: number;

  let viewGrid: HTMLElement | null = null;
  let nestNext = false;
  let weights = [3000000, 910000, 900000, 400000, 200000, 60000];

  $: if (viewGrid !== null) {
    nestNext = viewGrid.clientWidth >= 300;
    let template: string[] = [];
    weights.forEach((w) => {
      template.push(`${w}fr`);
    });
    viewGrid.style.gridTemplateColumns = template.join(" ");
  }
</script>

<div bind:this={viewGrid} class="grid-view h-full bg-purple-700">
  {#each weights as w, i}
    <div
      class="bg-green-300 min-w-0 text-ellipsis whitespace-nowrap overflow-hidden"
    >
      {w}
    </div>
    <div class="min-w-0 text-ellipsis whitespace-nowrap overflow-hidden">
      {#if step > 0 && nestNext}
        <svelte:self step={step - 1} />
      {:else}
        0
      {/if}
    </div>
  {/each}
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
