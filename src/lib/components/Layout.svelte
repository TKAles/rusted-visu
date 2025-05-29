<script lang="ts">
	import Modal from './Modal.svelte';
	import { currentView, WFMFolderSelected, foundWFMFiles, isDCProcessed,
		isFFTProcessed, isVMapProcessed, VIEW_STATES, viewHelpers } from '$lib/stores.js';
	import StartupScreen from './StartupScreen.svelte';
	import ScanningScreen from './ScanningScreen.svelte';
	
	import { invoke } from '@tauri-apps/api/core';
	import { resolveRoute } from '$app/paths';

	let isPropertiesOpen: Boolean = false;
	let DCScansProcessed: Boolean = false;
	let FFTsComputed: Boolean = false;

	// Initialize the view to startup screen
	currentView.set(VIEW_STATES.STARTUP);

	function togglePropertieModal(){
		isPropertiesOpen = !isPropertiesOpen;
	}

	async function selectWFMFolder(){
		let selected_folder: string[] = await pickWFMFolder();
		if(selected_folder && selected_folder.length > 0) {
			WFMFolderSelected.set(true);
			foundWFMFiles.set(selected_folder);
			// Switch to a different view after folder selection
			currentView.set(VIEW_STATES.FOLDER_SELECTED);
		} else {
			WFMFolderSelected.set(false);
		}
	}

	function launchScanCheck() {
		currentView.set(VIEW_STATES.SCANNING);
		return;
	}

	function processDCScans() {
		DCScansProcessed = true;
		isDCProcessed.set(true);
		currentView.set(VIEW_STATES.DC_PROCESSING);
	}

	function processFFTs() {
		FFTsComputed = true;
		isFFTProcessed.set(true);
		currentView.set(VIEW_STATES.FFT_PROCESSING);
	}	

	async function pickWFMFolder() {
		try {
			const result = await invoke<string[]>('get_wfmdir');
			if(result.length != 0) {
				console.log('Found {0} WFM files in the directory.', result.length);
			}
			return result;
		} catch (error) {
			console.error('Failed to select folder:', error);
			WFMFolderSelected.set(false);
			return [];
		}
	}

	// Function to get the component to render based on current view
	function getViewComponent(view: string) {
		switch(view) {
			case VIEW_STATES.STARTUP:
				return StartupScreen;
			case VIEW_STATES.SCANNING:
				return ScanningScreen;
			case VIEW_STATES.FOLDER_SELECTED:
				return StartupScreen; // or create a FolderSelectedScreen component
			case VIEW_STATES.DC_PROCESSING:
				return ScanningScreen; // or create a DCProcessingScreen component
			case VIEW_STATES.FFT_PROCESSING:
				return ScanningScreen; // or create a FFTProcessingScreen component
			default:
				return StartupScreen;
		}
	}
</script>
	
<main class="flex w-screen h-screen bg-gray-200 text-gray-900 overflow-hidden">
	<aside class="w-1/3 bg-gray-600 h-full p-4 flex flex-col max-w-64 flex-shrink-0">
		<button on:click={selectWFMFolder} class="btn-sidebar">Open WFM Folder</button>
		<button on:click={launchScanCheck} class="{!WFMFolderSelected ? 'btn-sidebar-disabled' : 'btn-sidebar'}"
		disabled={!WFMFolderSelected}>
			Check Scans</button>
		<button on:click={processDCScans} class="{!WFMFolderSelected ? 'btn-sidebar-disabled' : 'btn-sidebar'}" 
		disabled={!WFMFolderSelected}>
			Process DC Maps</button>
		<button class="{!DCScansProcessed ? 'btn-sidebar-disabled' : 'btn-sidebar'}">
			Export DC CSV</button>
		<button on:click={processFFTs} class="{!DCScansProcessed ? 'btn-sidebar-disabled' : 'btn-sidebar'}" 
		disabled={!DCScansProcessed}>
			Process FFT Maps</button>
		<button class="{!FFTsComputed ? 'btn-sidebar-disabled' : 'btn-sidebar'}">Export FFT CSV</button>
		<button on:click={togglePropertieModal} class="{!WFMFolderSelected ? 'btn-sidebar-disabled' : 'btn-sidebar'}"
		disabled={!WFMFolderSelected}>Edit Scan Properties</button>			
	</aside>
	<section class="flex-1 h-full p-4 flex flex-col gap-2 overflow-hidden min-w-0">
		<div class="flex-1 overflow-y-auto overflow-x-hidden">
		<!-- Dynamic component rendering based on currentView store -->
		{#if $currentView}
			<svelte:component this={getViewComponent($currentView)} />
		{:else}
			<StartupScreen />
		{/if}
	</div>
	</section>
</main>
<Modal isOpen={isPropertiesOpen} onClose={togglePropertieModal}/>