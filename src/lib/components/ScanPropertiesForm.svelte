<script lang="ts">
    export let scanTitle: string = "";
    export let scanCoordinates: number[];
    export let scanDeltas: number[];
    export let scansInCollection: number;
    export let cancelClick: () => void = () => {};
    let originalData = {scanTitle, scanCoordinates, scanDeltas, scansInCollection};
    let editedData = structuredClone(originalData);
    </script>
<div class="text-center p-4 mb-4 text-2xl">
    Scan Collection Metadata:
</div>
<div class="grid grid-cols-2">
        <label class="modal-label">Scan Title:</label>
        <input class="modal-inputbox" type="text" bind:value={editedData.scanTitle} />
       
        <label class="modal-label">XCoordinate [mm]:</label>
        <input class="modal-inputbox" type="number" bind:value={editedData.scanCoordinates[0]} />
       
        <label class="modal-label">YCoordinate [mm]:</label>
        <input class="modal-inputbox" type="number" bind:value={editedData.scanCoordinates[1]} />
       
        <label class="modal-label">X Delta [mm]:</label>
        <input class="modal-inputbox" type="number" bind:value={editedData.scanDeltas[0]} />
       
        <label class="modal-label">Y Delta [mm]:</label>
        <input class="modal-inputbox" type="number" bind:value={editedData.scanDeltas[1]} />
       
        <label class="modal-label">Scans in Collection:</label>
        <input class="modal-inputbox" type="number" bind:value={editedData.scansInCollection} />

        <button class="btn-modal"> Apply </button>
        <button class="btn-modal" on:click={cancelClick()}> Cancel </button>
</div>