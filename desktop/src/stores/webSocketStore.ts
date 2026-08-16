import { create } from "zustand";
import type { ServerEvent } from "../generated/bindings";

type WebsocketStatus = "connected" | "connecting" | "disconnected";

type IncomingChatRequest = {
    id: string;
    from: string;
    createdAt: string;
};

// Create state which can be accessed through the store
interface WebsocketState {
    onlineUserIds: Set<string>;
    status: WebsocketStatus;
    incomingChatRequests: IncomingChatRequest[];

    handleEvent: (event: ServerEvent) => void;
    setStatus: (status: WebsocketStatus) => void;
    addIncomingChatRequest: (from: string) => void;
    removeIncomingChatRequest: (id: string) => void;
}

export const useWebsocketStore = create<WebsocketState>((set) => ({
    status: "disconnected",
    onlineUserIds: new Set<string>(),
    incomingChatRequests: [],

    handleEvent: (event) => {
        switch (event.type) {
            case "PresenceOnline":
                set((state) => {
                    const onlineUserIds = new Set(state.onlineUserIds);
                    onlineUserIds.add(event.id);
                    return { onlineUserIds };
                });
                break;

            case "PresenceOffline":
                set((state) => {
                    const onlineUserIds = new Set(state.onlineUserIds);
                    onlineUserIds.delete(event.id);
                    return { onlineUserIds };
                });
                break;

            case "ChatRequestIncoming": {
                set((state) => {
                    const exists = state.incomingChatRequests.some((request) => request.from === event.from);
                    if (exists) {
                        return state;
                    }

                    return {
                        incomingChatRequests: [
                            ...state.incomingChatRequests,
                            {
                                id: `${event.from}-${Date.now()}`,
                                from: event.from,
                                createdAt: new Date().toISOString(),
                            },
                        ],
                    };
                });
                break;
            }

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
    },

    setStatus: (status) => {
        set({ status });
    },

    addIncomingChatRequest: (from) => {
        set((state) => ({
            incomingChatRequests: [
                ...state.incomingChatRequests,
                {
                    id: `${from}-${Date.now()}`,
                    from,
                    createdAt: new Date().toISOString(),
                },
            ],
        }));
    },

    removeIncomingChatRequest: (id) => {
        set((state) => ({
            incomingChatRequests: state.incomingChatRequests.filter((request) => request.id !== id),
        }));
    },
}));