import { api } from "./client";
import type { RegisterRequest, AuthResponse, LoginRequest, RefreshSessionRequest, RefreshSessionResponse } from "../generated/bindings";

export async function register(
    request: RegisterRequest,
): Promise<AuthResponse> {
    return api<AuthResponse>(
        "/auth/register", {
        method: "POST",
        body: JSON.stringify(request),
    }
    );
}

export async function login(
    request: LoginRequest
): Promise<AuthResponse> {
    return api<AuthResponse>(
        "/auth/login", {
        method: "POST",
        body: JSON.stringify(request)
    }
    );
}

export async function guestLogin(): Promise<AuthResponse> {
    return api<AuthResponse>(
        "/auth/guest", {
        method: "POST"
    }
    );
}

export async function refreshSession(request: RefreshSessionRequest): Promise<RefreshSessionResponse> {
    return api<RefreshSessionResponse>(
        "/auth/refresh", {
        method: "POST",
        body: JSON.stringify(request)
    }
    );
}