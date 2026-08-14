import type { ClientEvent, ServerEvent } from "../generated/bindings";
import { env } from "../config/env";

type ServerEventHandler = (event: ServerEvent) => void;

// Centralized wrapper for websocket functions
class WebsocketService {
    private websocket: WebSocket | null = null;
    private handler: ServerEventHandler | null = null;

    // Wrapper for new Websocket
    // Handles creating a websocket and callback methods
    connect(wsToken: string, onEvent: ServerEventHandler) {
        this.handler = onEvent;
        this.websocket = new WebSocket(
            env.wsUrl,
            ["p2p_chat", wsToken]
        );

        this.websocket.onopen = () => {
            console.log("Websocket connected");
        }

        this.websocket.onmessage = (message) => {
            try {
                const event = JSON.parse(message.data) as ServerEvent;
                this.handler?.(event);
            } catch (error) {
                console.error("Invalid message type: ", error);
            }
        }

        this.websocket.onclose = () => {
            console.log("Websocket disconnected!");
            this.websocket = null;
        }

        this.websocket.onerror = (error) => {
            console.error("WebSocket error:", error);
        };
    }

    send(event: ClientEvent) {
        if (!this.websocket || this.websocket.readyState != WebSocket.OPEN) {
            console.error("Websocket is not connected!");
        }

        this.websocket?.send(JSON.stringify(event));
    }

    disconnect() {
        this.websocket?.close();
        this.websocket = null;
    }
}

export const webSocketService: WebsocketService = new WebsocketService();

// Avoid manual reconstruction of JSON payload within components
export function sendChatRequest(to: string) {
    webSocketService.send({
        type: "ChatRequestSend",
        to
    });
}

export function acceptChatRequest(from: string) {
    webSocketService.send({
        type: "ChatRequestAccept",
        from
    });
}