import type { ClientEvent, ServerEvent } from "../generated/bindings";
import { WebsocketStore } from "../stores/webSocketStore";
import { env } from "../config/env";

type ServerEventHandler = (event: ServerEvent) => void;

// Centralized wrapper for websocket functions
class WebsocketService {
    private websocket: WebSocket | null = null;
    private handler: ServerEventHandler | null = null;

    // Wrapper for new Websocket
    // Handles creating a websocket and callback methods
    connect(wsToken: string, onEvent: ServerEventHandler) {
        const url = `${env.wsUrl}?ws_token=${encodeURIComponent(wsToken)}`;
        WebsocketStore.getState().setStatus("connecting");
        this.handler = onEvent;
        this.websocket = new WebSocket(url);

        this.websocket.onopen = () => {
            console.log("Websocket connected");
            WebsocketStore.getState().setStatus("connected");
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
            WebsocketStore.getState().setStatus("disconnected");
            this.websocket = null;
        }

        this.websocket.onerror = (error) => {
            console.error("WebSocket error:", error);
            WebsocketStore.getState().setStatus("disconnected");
        };
    }

    send(event: ClientEvent) {
        if (!this.websocket || this.websocket.readyState != WebSocket.OPEN) {
            console.error("Websocket is not connected!");
            return;
        }

        this.websocket?.send(JSON.stringify(event));
    }

    disconnect() {
        WebsocketStore.getState().setStatus("disconnected");
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