<script lang="ts">
    export let disabled = false;
</script>

<button
    on:click
    {disabled}
    class:hover:pb-1={!disabled}
    class="block h-full disabled:text-slate-400"><slot /></button
>
