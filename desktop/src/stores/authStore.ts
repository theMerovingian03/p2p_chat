import { create } from "zustand";
import type { UserDto } from "../generated/bindings";
import { deleteRefreshToken } from "./tokenStore";

interface AuthState {
    accessToken: string | null;
    user: UserDto | null;
    setAccessToken: (token: string | null) => void;
    setUser: (user: UserDto | null) => void;
    logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
    accessToken: null,
    user: null,

    setAccessToken: (token) =>
        set({
            accessToken: token,
        }),

    setUser: (user) =>
        set({
            user,
        }),

    logout: async () => {
        await deleteRefreshToken()

        set({
            accessToken: null,
            user: null,
        });
    }
}));