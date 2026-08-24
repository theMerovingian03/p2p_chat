import { listen } from "@tauri-apps/api/event";
import { create } from "zustand";
import type { ServerEvent } from "../generated/bindings";

type WebsocketStatus = "connected" | "connecting" | "disconnected";

type IncomingChatRequest = {
    id: string;
    from: string;
    createdAt: string;
    username: string;
};

// Create state which can be accessed through the store
interface WebsocketState {
    onlineUserIds: Set<string>;
    status: WebsocketStatus;
    incomingChatRequests: IncomingChatRequest[];

    handleEvent: (event: ServerEvent) => void;
    setStatus: (status: WebsocketStatus) => void;
    addIncomingChatRequest: (from: string, username: string) => void;
    removeIncomingChatRequest: (id: string) => void;
    initializeEventListeners: () => Promise<void>;
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
                                username: event.username
                            },
                        ],
                    };
                });
                break;
            }

            case "ChatRequestAccepted":
                console.log("Your chat request was accepted by: ", event.from);
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

    addIncomingChatRequest: (from, username) => {
        set((state) => ({
            incomingChatRequests: [
                ...state.incomingChatRequests,
                {
                    id: `${from}-${Date.now()}`,
                    from,
                    createdAt: new Date().toISOString(),
                    username,
                },
            ],
        }));
    },

    removeIncomingChatRequest: (id) => {
        set((state) => ({
            incomingChatRequests: state.incomingChatRequests.filter((request) => request.id !== id),
        }));
    },

    initializeEventListeners: async () => {
        try {
            // Listen for server events from Rust WebSocket manager
            // target is ws-event
            await listen<ServerEvent>("ws-event", (event) => {
                useWebsocketStore.getState().handleEvent(event.payload);
            });

            // Listen for status changes from Rust WebSocket manager
            await listen<{ status: WebsocketStatus }>(
                "ws-status-changed",
                (event) => {
                    useWebsocketStore.getState().setStatus(event.payload.status);
                }
            );

            // Listeners are active for the lifetime of the app.
            // Uncomment below to store unlisten functions for cleanup if needed:
            // return { _unlistenEvent, _unlistenStatus }
        } catch (error) {
            console.error("Failed to initialize WebSocket event listeners:", error);
        }
    },
}));
