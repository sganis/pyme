import { writable, get } from "svelte/store";

const initialState = {
  username: "",
  token: "",
};
const localState = localStorage.getItem("state");
const appState = localState ? JSON.parse(localState) : initialState;

export const state = writable(appState);

// export const updateLS = (callback) => {
//   const updatedState = callback(get(state));
//   state.update(() => updatedState);
//   localStorage.setItem("state", JSON.stringify(updatedState));
// };

export const working = writable(false);
export const apierror = writable("");
export const API_URL = import.meta.env.VITE_PUBLIC_BASE_URL;
