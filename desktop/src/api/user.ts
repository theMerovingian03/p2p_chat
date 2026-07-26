import { UserDto } from "../generated/bindings";
import { api } from "./client";

export default function me(): Promise<UserDto> {
    return api<UserDto>(
        "/me", {
        method: "GET"
    }
    );
}