import { create } from "zustand";
import type { ServerEvent } from "../generated/bindings";

// Create state which can be accessed through the store
interface WebsocketState {
    connected: boolean;
    onlineUserIds: Set<string>;

    handleEvent: (event: ServerEvent) => void;
}

export const WebsocketStore = create<WebsocketState>((set) => ({
    connected: false,
    onlineUserIds: new Set<string>(),

    handleEvent: (event) => {
        switch (event.type) {
            case "PresenceOnline":
                set((state) => {
                    const onlineUserIds = new Set(state.onlineUserIds);
                    onlineUserIds.add(event.id);
                    return { onlineUserIds };
                })
                break;

            case "PresenceOffline": set((state) => {
                const onlineUserIds = new Set(state.onlineUserIds);
                onlineUserIds.delete(event.id);
                return { onlineUserIds };
            })
                break;

            case "ChatRequestIncoming":
                console.log("Incoming chat request from: ", event.from);
                break;

            case "ChatRequestAccepted":
                console.log("Your chat request was accepted by: ", event.from);
                break;

            case "WebRtcOffer":
                console.log("WebRtcOffer event: to be implemented");
                break;

            case "WebRtcAnswer":
                console.log("WebRtcAnswer event: to be implemented");
                break;

            case "IceCandidate":
                console.log("IceCandidate event: to be implemented");
                break;

            case "GenericMessage":
                console.log("Message received from server: ", event.message);
                break;

            case "Error":
                console.error(
                    `WebSocket error [${event.code}]: ${event.message}`,
                );
                break;
        }
    }
}))