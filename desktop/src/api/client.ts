import { env } from "../config/env";

let accessToken: string | null = null;

export async function api<T>(
    path: string,
    options?: RequestInit
): Promise<T> {
    const response = await fetch(`${env.apiUrl}${path}`, {
        headers: {
            "Content-Type": "application/json",
            ...(options?.headers ?? {}),
        },
        ...options,
    });

    if (!response.ok) {
        throw new Error(await response.text());
    }
    return response.json() as Promise<T>;
}

export function setAccessToken(token: string | null) {
    accessToken = token;
} 