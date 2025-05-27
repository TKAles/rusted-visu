import { writable } from 'svelte/store';

// Define view states as constants for type safety and consistency
export const VIEW_STATES = {
    STARTUP: 'startup',
    FOLDER_SELECTED: 'folder-selected',
    SCANNING: 'scanning',
    DC_PROCESSING: 'dc-processing',
    FFT_PROCESSING: 'fft-processing',
    RESULTS: 'results',
    ERROR: 'error'
} as const;

// Type for view states
export type ViewState = typeof VIEW_STATES[keyof typeof VIEW_STATES];

// Current view store - starts with startup screen
export const currentView = writable<ViewState>(VIEW_STATES.STARTUP);

// Your existing stores
export const selectedPath = writable<string>('');
export const isDCProcessed = writable<boolean>(false);
export const isFFTProcessed = writable<boolean>(false);
export const isVMapProcessed = writable<boolean>(false);

// Optional: Helper functions for common view transitions
export const viewHelpers = {
    goToStartup: () => currentView.set(VIEW_STATES.STARTUP),
    goToScanning: () => currentView.set(VIEW_STATES.SCANNING),
    goToDCProcessing: () => currentView.set(VIEW_STATES.DC_PROCESSING),
    goToFFTProcessing: () => currentView.set(VIEW_STATES.FFT_PROCESSING),
    goToResults: () => currentView.set(VIEW_STATES.RESULTS),
    goToError: () => currentView.set(VIEW_STATES.ERROR)
};

// Optional: Derived store for checking current view state
export const isCurrentView = (viewState: ViewState) => {
    return writable(false);
};