import { api } from "./client";
import { AcceptReqRequest, CreateFriendReqRequest, FriendRequestRowDto } from "../generated/bindings";

export async function createFriendRequest(
    request: CreateFriendReqRequest
) {
    return api<CreateFriendReqRequest>(
        "/friend/create_request", {
        method: "POST",
        body: JSON.stringify(request)
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

export async function getSentFriendRequests() {
    return api<FriendRequestRowDto>(
        "/friend/sent", {
        "method": "GET"
    }
    );
}

export async function getReceivedFriendRequests() {
    return api<FriendRequestRowDto>(
        "/friend/received", {
        "method": "GET"
    }
    );
}