import { invoke } from "@tauri-apps/api/core";

export async function sendMessage(peerId: string, message: string) {
    try {
        await invoke("send_message", { peerId: peer_id, message });
    } catch (error) {
        console.log(error);
    }
}