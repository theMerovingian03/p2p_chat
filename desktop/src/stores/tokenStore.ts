import { invoke } from "@tauri-apps/api/core";

export async function saveRefreshToken(token: String) {
    await invoke("save_refresh_token", { token });
}

export async function loadRefreshToken() {
    return await invoke<string>("load_refresh_token");
}

export async function deleteRefreshToken() {
    await invoke("delete_refresh_token");
}