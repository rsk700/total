<script lang="ts">
    import { onMount } from "svelte";
    import * as ipc from "../ipc";

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
                <button
                    on:click={() => openUrl("http://rsk.me/")}
                    class="text-blue-700 hover:text-blue-500 active:text-blue-300"
                    >rsk</button
                >
            </p>
            <p>v{version}</p>
            <p>
                <button
                    on:click={() => openUrl("https://github.com")}
                    class="text-blue-700 hover:text-blue-500 active:text-blue-300"
                    >https://todo-url</button
                >
            </p>
            <p class="mt-8 text-lg italic">
                Also I'm making note taking, todo list app <button
                    on:click={() => openUrl("https://heaplist.app/")}
                    class="text-blue-700 hover:text-blue-500 active:text-blue-300"
                    >heaplist.app</button
                >
            </p>
        </div>
        <div class="mt-8 text-center">
            <button
                on:click={hide}
                class="border-2
            rounded
            px-2
            text-lg
            border-green-400
            hover:bg-green-400
            active:bg-green-300
            active:border-green-300
            active:text-green-900">close</button
            >
        </div>
    </div>
</div>
