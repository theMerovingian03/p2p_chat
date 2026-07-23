import { env } from "../config/env";
import { useAuthStore } from "../stores/authStore";

export async function api<T>(
    path: string,
    options?: RequestInit
): Promise<T> {

    const token = useAuthStore.getState().accessToken;
    const headers = new Headers(options?.headers);
    headers.set("Content-Type", "application/json");

    if (token) {
        headers.set("Authorization", `Bearer ${token}`);
    }

    const response = await fetch(`${env.apiUrl}${path}`, {
        ...options,
        headers,
    });

    if (!response.ok) {
        throw new Error(await response.text());
    }
    return response.json() as Promise<T>;
}