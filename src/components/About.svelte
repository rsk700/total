<script lang="ts">
    import { onMount } from "svelte";
    import * as ipc from "../ipc";
    import LinkButton from "./LinkButton.svelte";
    import ButtonDark from "./ButtonDark.svelte";

    export let show: boolean;

    let version = "";

    onMount(() => {
        ipc.info().then((about) => (version = about.version));
    });

    function hide() {
        show = false;
    }

    function closeOnEscape(e: KeyboardEvent) {
        if (e.key === "Escape") {
            hide();
        }
    }

    function openUrl(url: string) {
        ipc.openPath(url);
    }
</script>

<svelte:body on:keydown={closeOnEscape} />
<div class:hidden={!show}>
    <div
        on:click={hide}
        on:keydown={hide}
        role="button"
        tabindex="-1"
        class="absolute inset-0 z-10 bg-black opacity-60"
    />
    <div
        class="absolute inset-8 z-20 p-4 rounded bg-slate-50 flex flex-col justify-center overflow-y-auto"
    >
        <div class="text-center">
            <p class="text-lg">
                <span class="font-bold">Total</span> - disk usage visualization
                tool by
                <LinkButton on:click={() => openUrl("http://rsk.me/")}
                    >rsk
                </LinkButton>
            </p>
            <p>v{version}</p>
            <p>
                <LinkButton on:click={() => openUrl("https://github.com/rsk700/total")}
                    >github.com/rsk700/total</LinkButton
                >
            </p>
            <p class="mt-8 text-lg italic">
                Also I'm making note taking, todo list app <LinkButton
                    on:click={() => openUrl("https://heaplist.app/")}
                    >heaplist.app</LinkButton
                >
            </p>
        </div>
        <div class="mt-8 text-center">
            <ButtonDark on:click={hide}>close</ButtonDark>
        </div>
    </div>
</div>
