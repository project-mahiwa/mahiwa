import { writable } from 'svelte/store';

// wasmファイルのパス
export const wasmFilePath = writable('');
export const boardName = writable('');
export const portName = writable('');
