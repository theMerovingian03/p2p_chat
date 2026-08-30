import { create } from "zustand";
import { getFriends } from "../api/friend";
import { FriendRowDto } from "../generated/bindings";

interface Friend extends FriendRowDto {
    isOnline: boolean;
}

interface FriendStore {
    friends: Record<string, Friend>;

    initializeFriends: () => Promise<void>;
    setOnline: (friendId: string) => void;
    setOffline: (friendId: string) => void;
    getFriend: (friendId: string) => Friend | undefined;
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
}))