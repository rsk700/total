<script lang="ts">
    export let disabled = false;
    export let title = "";
</script>

<button
    on:click
    {disabled}
    class:hover:pb-1={!disabled}
    class="block h-full disabled:text-[#9aa5d3] text-[#2b2d62] hover:text-[#297324] transition-all ease-out duration-100"
    {title}><slot /></button
>
