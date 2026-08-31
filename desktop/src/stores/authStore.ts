import { create } from "zustand";
import type { UserDto } from "../generated/bindings";
import { deleteRefreshToken } from "./tokenStore";
import { invoke } from "@tauri-apps/api/core";

interface AuthState {
    accessToken: string | null;
    user: UserDto | null;
    setAccessToken: (token: string | null) => void;
    setUser: (user: UserDto | null) => void;
    logout: () => void;
}

async function logoutCleanup() {
    try {
        await invoke("logout_cleanup");
    } catch (err) {
        console.log("Error occured while cleaning up on logout!", err);
    }
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
        await logoutCleanup()

        set({
            accessToken: null,
            user: null,
        });
    }
}));