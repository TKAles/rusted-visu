<script lang="ts">
    import {onMount, onDestroy} from 'svelte';
    import {WFMFolderSelected, foundWFMFiles, sortedWFMFiles } from '$lib/stores';
    
    // Add guard to prevent sorting when foundWFMFiles is undefined or empty
    $: if ($foundWFMFiles && $foundWFMFiles.length > 0) {
        sortedWFMFiles.set(sortWFMFiles($foundWFMFiles));
    }
    
    // Function to get just the filename from full path
    function getFileName(fullPath: string): string {
        return fullPath.split('/').pop() || fullPath;
    }
    
    function sortWFMFiles(inputArray: string[]): string[][][] {
        // First pass: determine dimensions by scanning all files
        const prefixes = new Set<string>();
        const scanNums = new Set<number>();
        const fileNums = new Set<number>();
        
        inputArray.forEach(filename => {
            const [prefix, scanStr, fileStr] = filename.split('-');
            const fileNum = parseInt(fileStr.split('.')[0]); // Remove .WFM extension
            const scanNum = parseInt(scanStr);
            
            prefixes.add(prefix);
            scanNums.add(scanNum);
            fileNums.add(fileNum);
        });
        
        // Create mapping from prefix to index
        const prefixToIndex = new Map<string, number>();
        Array.from(prefixes).sort().forEach((prefix, index) => {
            prefixToIndex.set(prefix, index);
        });
        
        // Determine maximum dimensions
        const maxScanNum = Math.max(...scanNums);
        const maxFileNum = Math.max(...fileNums);
        const numPrefixes = prefixes.size;
        
        // Create dynamically sized 3D array: [prefix][scan][fileNum]
        const sortedFiles: string[][][] = Array.from({ length: numPrefixes }, () =>
            Array.from({ length: maxScanNum + 1 }, () =>
                Array.from({ length: maxFileNum + 1 }, () => '')
            )
        );
        
        // Populate the 3D array
        inputArray.forEach(filename => {
            const [prefix, scanStr, fileStr] = filename.split('-');
            const fileNum = parseInt(fileStr.split('.')[0]); // Remove .WFM extension
            const scanNum = parseInt(scanStr);
            const prefixIndex = prefixToIndex.get(prefix)!;
            
            sortedFiles[prefixIndex][scanNum][fileNum] = filename;
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
            There are {$foundWFMFiles?.length || 0} files that were found in the directory.
        </div>
        <div class="flex-1 overflow-y-auto overflow-x-hidden min-w-0">
            {#if $sortedWFMFiles && $sortedWFMFiles[0]}
                <div>Number of Scans detected was {$sortedWFMFiles[0].length}</div>
                {#each $sortedWFMFiles[0] as scan, scanIndex}
                    <div>Scan {scanIndex + 1} contains {scan.filter(file => file !== '').length} files.</div>
                {/each}
            {:else}
                <div>Loading scan data...</div>
            {/if}
        </div>
    {/if}
</div>