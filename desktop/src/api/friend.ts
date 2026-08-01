import { api } from "./client";
import { AcceptReqRequest, CreateFriendReqRequest } from "../generated/bindings";

export async function createFriendRequest(
    request: CreateFriendReqRequest
) {
    return api<CreateFriendReqRequest>(
        "/friend/create_request", {
        method: "POST",
        body: JSON.stringify(request),
    }
    );
}

export async function acceptFriendRequest(
    request: AcceptReqRequest
) {
    return api<AcceptReqRequest>(
        "/friend/accept_request", {
        method: "POST",
        body: JSON.stringify(request)
    }
    );
}