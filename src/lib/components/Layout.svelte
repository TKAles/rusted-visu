<script lang="ts">
	

	import Modal from './Modal.svelte';
	import StartupScreen from './StartupScreen.svelte';
	import ScanningScreen from './ScanningScreen.svelte';
	import { writable } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/core';
	export let isPropertiesOpen: boolean = false;
	export let WFMFolderSelected: boolean = false;
	export let DCScansProcessed: boolean = false;
	export let FFTsComputed: boolean = false;
	export const applicationViewState = writable('justLoaded');
	const possibleApplicationViews = {
		'justLoaded': StartupScreen,
		'scanCheck': ScanningScreen
	}
	function togglePropertieModal(){
		isPropertiesOpen = !isPropertiesOpen;
	}
	function selectWFMFolder(){
		pickWFMFolder();
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
	async function pickWFMFolder()
	{
		try {
			const result = await invoke<string | null>('get_wfmdir');
			if(result) {
				console.log('Folder Selected Was: ', result);
			} else {
				console.log('Dialog was closed out');
			}
		} catch (error) {
			console.error('Failed to select folder:', error);
		}
	}
</script>
	
<main class="flex w-full h-screen bg-gray-200 text-gray-900">
	<aside class="w-1/3 bg-gray-600 h-full p-4 flex flex-col max-w-64">
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
	<section class="w-2/3 h-full p-4 flex flex-col gap-2">
		<slot />
	</section>
</main>
<Modal isOpen={isPropertiesOpen} onClose={togglePropertieModal}/>