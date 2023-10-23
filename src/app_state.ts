import { writable } from "svelte/store";

export enum AppState {
    ChoosePath,
    Scanning,
    FlameView,
  }
export let appState = writable(AppState.ChoosePath);
export let appPath = writable("");