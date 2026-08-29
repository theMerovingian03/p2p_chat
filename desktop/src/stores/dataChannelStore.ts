import { create } from "zustand";
import { DataChannelAppEvent } from "../generated/bindings";
import { listen } from "@tauri-apps/api/event";

type ConnectedPeer = {
    peerId: string;
    // username: string;
}

export type ChatMessage = {
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
    addMessage: (message: ChatMessage) => void;
    addOutgoingMessage: (peerId: string, content: string) => void;
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
                set((state) => ({
                    messages: {
                        ...state.messages,
                        [event.peer_id]: [...(state.messages[event.peer_id] ?? []), {
                            id: crypto.randomUUID(),
                            peerId: event.peer_id,
                            content: new TextDecoder().decode(new Uint8Array(event.message)), timestamp: new Date().toISOString(), outgoing: false,
                        }]
                    }
                }))
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
    },

    addMessage: (message) => {
        set((state) => ({
            messages: {
                ...state.messages,
                [message.peerId]: [
                    ...(state.messages[message.peerId] ?? []),
                    message,
                ]
            }
        }))
    },

    addOutgoingMessage: (peerId, content) => {
        set((state) => ({
            messages: {
                ...state.messages,
                [peerId]: [
                    ...(state.messages[peerId] ?? []),
                    {
                        id: crypto.randomUUID(),
                        peerId,
                        content,
                        timestamp: new Date().toISOString(),
                        outgoing: true,
                    },
                ],
            },
        }));
    },
}));