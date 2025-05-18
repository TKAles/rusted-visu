<script lang="ts">
	import Modal from './Modal.svelte';

	export let title = "VISU | Rusted Edition";
	export let isPropertiesOpen: boolean = false;
	export let WFMFolderSelected: boolean = false;
	export let DCScansProcessed: boolean = false;
	export let FFTsComputed: boolean = false;

	function togglePropertieModal(){
		isPropertiesOpen = !isPropertiesOpen;
	}
	function selectWFMFolder(){
		WFMFolderSelected = true;
	}
	function launchScanCheck() {
		return;
	}
	function processDCScans() {
		DCScansProcessed = true;
	}
	function processFFTs() {
		FFTsComputed = true;
	}	

</script>
	
<main class="flex w-full h-screen bg-gray-200 text-gray-900">
	<aside class="w-1/3 bg-gray-600 h-full p-4 flex flex-col">
	<button on:click={selectWFMFolder} class="btn-sidebar">Open WFM Folder</button>
	<button on:click={launchScanCheck} class="{!WFMFolderSelected ? 'btn-sidebar-disabled' : 'btn-sidebar'}"
	disabled={!WFMFolderSelected}>
		Check Scans</button>
	<button on:click={processDCScans} class="{!WFMFolderSelected ? 'btn-sidebar-disabled' : 'btn-sidebar'}" 
	disabled={!WFMFolderSelected}>
		Process DC Maps</button>
	<button on:click={processFFTs} class="{!DCScansProcessed ? 'btn-sidebar-disabled' : 'btn-sidebar'}" 
	disabled={!DCScansProcessed}>
		Process FFT Maps</button>
	<button on:click={togglePropertieModal} class="{!WFMFolderSelected ? 'btn-sidebar-disabled' : 'btn-sidebar'}"
	disabled={!WFMFolderSelected} 
	>
		Edit Scan Properties</button>			
	</aside>
	<section class="w-2/3 h-full p-4 flex flex-col gap-2">
		<slot />
	</section>
</main>
<Modal isOpen={isPropertiesOpen} onClose={togglePropertieModal} />