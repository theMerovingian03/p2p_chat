import { create } from "zustand";
import { UserSearchModel } from "../generated/bindings";
import { searchUsers } from "../api/user";

interface SearchState {
    query: string;
    results: UserSearchModel[];
    loading: boolean;

    setQuery: (query: string) => void;
    search: (query: string) => Promise<void>;
}

export const useSearchStore = create<SearchState>((set) => ({
    query: "",
    results: [],
    loading: false,

    setQuery: (query) => set({ query }),
    search: async (query) => {

        // Empty query
        if (query.trim() === "") {
            set({ results: [] });
            return;
        }

        set({ loading: true });

        const users = await searchUsers(query);

        set({
            results: users,
            loading: false,
        });
    }
}));