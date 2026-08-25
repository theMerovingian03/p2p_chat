import { invoke } from "@tauri-apps/api/core";

export async function sendMessage(peer_id: String, message: String) {
    try {
        await invoke("send_message", { peer_id, message });
    } catch (error) {
        console.log(error);
    }
}