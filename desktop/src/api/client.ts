import { env } from "../config/env";
import { useAuthStore } from "../stores/authStore";

interface ApiOptions extends RequestInit {
    query?: Record<string, string | number | boolean | undefined>;
}

export async function api<T>(
    path: string,
    options?: ApiOptions
): Promise<T> {
    const token = useAuthStore.getState().accessToken;
    const headers = new Headers(options?.headers);
    headers.set("Content-Type", "application/json");

    if (token) {
        headers.set("Authorization", `Bearer ${token}`);
    }

    const url = new URL(path, env.apiUrl);

    if (options?.query) {
        for (const [key, value] of Object.entries(options.query)) {
            if (value !== undefined) {
                url.searchParams.set(key, String(value));
            }
        }
    }

    const response = await fetch(url, {
        ...options,
        headers,
    });

    if (!response.ok) {
        throw new Error(await response.text());
    }

    // Handle case where body is empty
    const text = await response.text();
    return text ? (JSON.parse(text) as T) : (undefined as T);
}