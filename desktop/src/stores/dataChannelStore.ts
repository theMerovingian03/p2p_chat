import { create } from "zustand";
import { DataChannelAppEvent } from "../generated/bindings";
import { listen } from "@tauri-apps/api/event";

type ConnectedPeer = {
    peerId: string;
    // username: string;
}

type ChatMessage = {
    id: string;
    peerId: string;
    content: string;
    timestamp: string;
    outgoing: boolean;
}

interface DataChannelStore {
    connectedPeers: ConnectedPeer[];
    messages: Record<string, ChatMessage[]>;
    handleEvent: (event: DataChannelAppEvent) => Promise<void>;
    initializeEventListener: () => Promise<void>;
}

export const useDataChannelStore = create<DataChannelStore>((set) => ({
    connectedPeers: [],
    messages: {},

    handleEvent: async (event) => {
        switch (event.type) {
            case "PeerConnected":
                console.log(`Peer connected!: ${event.peer_id}`);
                set((state) => {
                    if (state.connectedPeers.some(p => p.peerId === event.peer_id)) {
                        return state;
                    }

                    return {
                        connectedPeers: [
                            ...state.connectedPeers,
                            {
                                peerId: event.peer_id,
                                // username: event.username,
                            },
                        ],
                    };
                });
                break;

            case "PeerDisconnected":
                set((state) => ({
                    connectedPeers: state.connectedPeers.filter(
                        p => p.peerId !== event.peer_id
                    ),
                }));
                break;

            case "MessageReceived":
                // TODO: add message to the appropriate conversation
                console.log("Message received:", event.message);
                break;
            // case "MessageSendError":
            //     // TODO: handle/display error
            //     break;
        }
    },

    initializeEventListener: async () => {
        try {
            await listen<DataChannelAppEvent>("dc-event", (event) => {
                useDataChannelStore.getState().handleEvent(event.payload);
            })

        } catch {
            console.error("Failed to initialize event listener for Data Channel");
        }
    }
}));