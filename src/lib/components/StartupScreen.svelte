<script lang="ts">
    import {onMount, onDestroy} from 'svelte';
    import {WFMFolderSelected, foundWFMFiles } from '$lib/stores';
    
    // Function to get just the filename from full path
    function getFileName(fullPath: string): string {
        return fullPath.split('/').pop() || fullPath;
    }
    
    function sortWFMFiles(inputArray: string[]): string[][][] {
		// Create 3D array: [prefix][scan][fileNum]
		// Index 0 = DC, Index 1 = RF
		const sortedFiles: string[][][] = [
		  // DC files: [scan 0-9][file 0-380]
		  Array.from({ length: 10 }, () => Array.from({ length: 381 }, () => '')),
		  // RF files: [scan 0-9][file 0-380] 
		  Array.from({ length: 10 }, () => Array.from({ length: 381 }, () => ''))
		];
		
		// Populate the 3D array
		inputArray.forEach(filename => {
		  const [prefix, scanStr, fileStr] = filename.split('-');
		  const fileNum = parseInt(fileStr.split('.')[0]); // Remove .WFM extension
		  const scanNum = parseInt(scanStr);
		  
		  if (prefix === 'DC') {
		    sortedFiles[0][scanNum][fileNum] = filename;
		  } else if (prefix === 'RF') {
		    sortedFiles[1][scanNum][fileNum] = filename;
		  }
		});
		
		return sortedFiles;
	}
</script>

<div class="h-full flex flex-col overflow-hidden">
    {#if $WFMFolderSelected == false}
        <h2 class="text-3xl">Please use the 'Open WFM Folder' button to select the folder where your scans are.</h2>
    {/if}
    {#if $WFMFolderSelected == true}
        <div class="mb-4 flex-shrink-0">
            There are {$foundWFMFiles.length} files that were found in the directory.
        </div>
        <div class="flex-1 overflow-y-auto overflow-x-hidden min-w-0">
            {#each $foundWFMFiles as fname}
                <div class="break-all py-1 max-w-full">
                    {getFileName(fname)}
                </div>
            {/each}
        </div>
    {/if}
</div>