<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import { onDestroy, onMount } from "svelte";
    import type { PathComponent } from "../../ipc/messages/types/structs";
    import { navigate, openPath, startScan } from "../../ipc";
    import { AppState, appState } from "../../app_state";

    export let pathTop: string;
    export let components: PathComponent[];
    export let separator: string;
    export let isLeftHidden = false;

    let selectedSection: number = -1;
    let firstNode: HTMLElement | null = null;
    let subOff: (() => void)[] = [];

    $: {
        let _ = components;
        updateIsLeftHidden();
    }

    appWindow
        .onResized(() => {
            updateIsLeftHidden();
        })
        .then((off) => subOff.push(off));

    onMount(updateIsLeftHidden);
    onDestroy(() => {
        subOff.forEach((off) => off());
    });

    function updateIsLeftHidden() {
        if (firstNode !== null) {
            isLeftHidden =
                firstNode.getBoundingClientRect().left <
                (firstNode.parentNode as HTMLElement).getBoundingClientRect()
                    .left;
        }
    }

    function overPath(n: number) {
        selectedSection = n;
    }

    function contextMenu(e: Event, n: number) {
        e.preventDefault();
        e.stopPropagation();
        openPath(components[n].path);
    }

    async function onPathClick(e: MouseEvent, n: number) {
        if (e.button === 0) {
            await navigate(null, components[n].path);
            appState.set(AppState.Scanning);
        } else if (e.button === 1) {
            openPath(components[n].path);
        }
    }
</script>

<div bind:this={firstNode} />
{#each components as c, i}
    {@const first = i === 0}
    {@const last = i === components.length - 1}
    <button
        on:mouseover={() => overPath(i)}
        on:mouseleave={() => overPath(-1)}
        on:focus={() => overPath(i)}
        on:blur={() => overPath(-1)}
        on:click={(e) => onPathClick(e, i)}
        on:contextmenu={(e) => contextMenu(e, i)}
        class:selected-color={selectedSection >= i}
        class:pb-1={selectedSection >= i}
        class="block h-full text-[#10112d] transition-all ease-out duration-100"
        >{#if first}{pathTop}{/if}{c.name}{#if !last}{separator}{/if}</button
    >
{/each}

<style>
    .selected-color {
        color: #035600;
    }
</style>
