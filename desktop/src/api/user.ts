import { UserDto, UserSearchModel } from "../generated/bindings";
import { api } from "./client";

export async function me(): Promise<UserDto> {
    return api<UserDto>(
        "/user/me", {
        method: "GET"
    }
    );
}

export async function searchUsers(query: string): Promise<UserSearchModel[]> {
    return api<UserSearchModel[]>(
        "/user/search",
        {
            method: "GET",
            query: {
                query,
            },
        }
    );
}