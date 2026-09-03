import { invoke } from "@tauri-apps/api/core";

/**
 * Centralized wrapper for WebSocket functions.
 * The actual WebSocket connection is managed in Rust via Tauri.
 * This service provides high-level functions for React components.
 */
class WebsocketService {
    /**
     * Connect to the WebSocket server
     */
    async connect(wsToken: string, wsUrl: string = "ws://localhost:8000/ws") {
        try {
            await invoke("connect_websocket", {
                wsUrl,
                wsToken,
            });
        } catch (error) {
            console.error("Failed to connect WebSocket:", error);
            throw error;
        }
    }

    /**
     * Disconnect from the WebSocket server
     */
    async disconnect() {
        try {
            await invoke("disconnect_websocket");
        } catch (error) {
            console.error("Failed to disconnect WebSocket:", error);
            throw error;
        }
    }

    /**
     * Get current connection status
     */
    async getStatus(): Promise<string> {
        try {
            return await invoke("get_websocket_status");
        } catch (error) {
            console.error("Failed to get WebSocket status:", error);
            throw error;
        }
    }
}

export const webSocketService = new WebsocketService();

/**
 * High-level functions for common WebSocket operations
 * These delegate to Rust-backed Tauri commands
 */
export async function sendChatRequest(to: string) {
    try {
        await invoke("send_chat_request", { to });
    } catch (error) {
        console.error("Failed to send chat request:", error);
        throw error;
    }
}

export async function acceptChatRequest(from: string) {
    try {
        await invoke("accept_chat_request", { from });
    } catch (error) {
        console.error("Failed to accept chat request:", error);
        throw error;
    }
}

export async function requestPresences(friendIds: string[]) {
    try {
        await invoke("request_presences", { friendIds });
    } catch (error) {
        console.error("Failed to request presences:", error);
        throw error;
    }
}