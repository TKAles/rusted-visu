<script lang="ts">
    export let isOpen: boolean = false;
    export let onClose: () => void = () => {};
    import ScanPropertiesForm from './ScanPropertiesForm.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { browser } from '$app/environment';

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape' && isOpen == true) {
            onClose();
        }
    }

    function cancelEditClick() {
        onClose();
    }

    onMount(() => {
        if(browser && typeof document !== 'undefined')
    {
        document.addEventListener('keydown', handleKeydown);
    }

    });

    onDestroy(() => {
        if(browser && typeof document !== 'undefined')
    {
        document.removeEventListener('keydown', handleKeydown);
    }
    });
</script>

{#if isOpen}
<div class="fixed inset-0 z-50 bg-black/50 
    overflow-auto flex items-center justify-center">
    <div class="relative p-8 bg-white w-full max-w-md m-auto flex-col flex">
        <slot />
        <ScanPropertiesForm scanCoordinates={[]} scanDeltas={[]} 
            scanTitle="" cancelClick={cancelEditClick} scansInCollection={0}/>
    </div>
</div>
{/if}