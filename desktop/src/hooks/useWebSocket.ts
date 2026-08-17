import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useCallback, useRef } from "react";
import type { ServerEvent } from "../generated/bindings";

type WebSocketEventHandler = (event: ServerEvent) => void;

/**
 * Hook to manage WebSocket connection and events through Tauri
 */
export function useWebSocket() {
    const eventListenersRef = useRef<UnlistenFn[]>([]);

    /**
     * Initialize WebSocket connection
     */
    const connect = useCallback(async (wsUrl: string, wsToken: string) => {
        try {
            await invoke("connect_websocket", {
                wsUrl,
                wsToken,
            });
        } catch (error) {
            console.error("Failed to connect WebSocket:", error);
            throw error;
        }
    }, []);

    /**
     * Disconnect from WebSocket
     */
    const disconnect = useCallback(async () => {
        try {
            await invoke("disconnect_websocket");
        } catch (error) {
            console.error("Failed to disconnect WebSocket:", error);
            throw error;
        }
    }, []);

    /**
     * Get current WebSocket connection status
     */
    const getStatus = useCallback(async (): Promise<string> => {
        try {
            return await invoke("get_websocket_status");
        } catch (error) {
            console.error("Failed to get WebSocket status:", error);
            throw error;
        }
    }, []);

    /**
     * Send a chat request
     */
    const sendChatRequest = useCallback(async (to: string) => {
        try {
            return await invoke("send_chat_request", { to });
        } catch (error) {
            console.error("Failed to send chat request:", error);
            throw error;
        }
    }, []);

    /**
     * Accept a chat request
     */
    const acceptChatRequest = useCallback(async (from: string) => {
        try {
            return await invoke("accept_chat_request", { from });
        } catch (error) {
            console.error("Failed to accept chat request:", error);
            throw error;
        }
    }, []);

    /**
     * Send WebRTC offer
     */
    const sendWebRtcOffer = useCallback(async (to: string, sdp: string) => {
        try {
            return await invoke("send_webrtc_offer", { to, sdp });
        } catch (error) {
            console.error("Failed to send WebRTC offer:", error);
            throw error;
        }
    }, []);

    /**
     * Send WebRTC answer
     */
    const sendWebRtcAnswer = useCallback(async (to: string, sdp: string) => {
        try {
            return await invoke("send_webrtc_answer", { to, sdp });
        } catch (error) {
            console.error("Failed to send WebRTC answer:", error);
            throw error;
        }
    }, []);

    /**
     * Send ICE candidate
     */
    const sendIceCandidate = useCallback(async (to: string, candidate: string) => {
        try {
            return await invoke("send_ice_candidate", { to, candidate });
        } catch (error) {
            console.error("Failed to send ICE candidate:", error);
            throw error;
        }
    }, []);

    /**
     * Setup event listeners for WebSocket events
     */
    const setupEventListeners = useCallback(async (onEvent: WebSocketEventHandler, onStatusChange: (status: string) => void) => {
        try {
            // Listen for server events
            const unlistenEvent = await listen<ServerEvent>("ws-event", (event) => {
                console.log("WebSocket event received:", event.payload);
                onEvent(event.payload);
            });

            // Listen for status changes
            const unlistenStatus = await listen<{ status: string }>("ws-status-changed", (event) => {
                console.log("WebSocket status changed:", event.payload.status);
                onStatusChange(event.payload.status);
            });

            // Listen for errors
            const unlistenError = await listen<{ message: string }>("ws-error", (event) => {
                console.error("WebSocket error:", event.payload.message);
            });

            eventListenersRef.current = [unlistenEvent, unlistenStatus, unlistenError];
        } catch (error) {
            console.error("Failed to setup WebSocket event listeners:", error);
        }
    }, []);

    /**
     * Cleanup event listeners
     */
    const cleanup = useCallback(() => {
        eventListenersRef.current.forEach((unlisten) => unlisten());
        eventListenersRef.current = [];
    }, []);

    return {
        connect,
        disconnect,
        getStatus,
        sendChatRequest,
        acceptChatRequest,
        sendWebRtcOffer,
        sendWebRtcAnswer,
        sendIceCandidate,
        setupEventListeners,
        cleanup,
    };
}
