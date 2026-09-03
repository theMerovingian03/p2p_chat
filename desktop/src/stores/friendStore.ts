import { create } from "zustand";
import { getFriends } from "../api/friend";
import { FriendRowDto } from "../generated/bindings";
import { useWebsocketStore } from "./webSocketStore";

interface Friend extends FriendRowDto {
    isOnline: boolean;
}

interface FriendStore {
    friends: Record<string, Friend>;

    initializeFriends: () => Promise<void>;
    setOnline: (friendId: string) => void;
    setOffline: (friendId: string) => void;
    getFriend: (friendId: string) => Friend | undefined;
    syncOnlineStatus: () => void;
}

export const useFriendStore = create<FriendStore>((set, get) => ({
    friends: {},

    initializeFriends: async () => {
        const friends = await getFriends();
        const friendMap: Record<string, Friend> = {};

        for (const friend of friends) {
            friendMap[friend.friend_id] = {
                ...friend,
                isOnline: false
            }
        }

        set({ friends: friendMap });
        
        // Sync online status with current WebSocket state
        get().syncOnlineStatus();
    },

    setOnline: (friendId) => {
        set((state) => {
            const friend = state.friends[friendId];
            if (!friend) {
                return state;
            }

            return {
                friends: {
                    ...state.friends,
                    [friendId]: {
                        ...friend,
                        isOnline: true
                    }
                }
            }
        })
    },
    setOffline: (friendId) => {
        set((state) => {
            const friend = state.friends[friendId];
            if (!friend) {
                return state;
            }

            return {
                friends: {
                    ...state.friends,
                    [friendId]: {
                        ...friend,
                        isOnline: false
                    }
                }
            }
        })
    },
    getFriend: (friendId) => {
        return get().friends[friendId];
    },
    syncOnlineStatus: () => {
        const onlineUserIds = useWebsocketStore.getState().onlineUserIds;
        set((state) => {
            const updatedFriends: Record<string, Friend> = {};
            for (const [friendId, friend] of Object.entries(state.friends)) {
                updatedFriends[friendId] = {
                    ...friend,
                    isOnline: onlineUserIds.has(friendId)
                };
            }
            return { friends: updatedFriends };
        });
    },
}))