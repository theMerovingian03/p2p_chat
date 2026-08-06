import { api } from "./client";
import { AcceptReqRequest, CreateFriendReqRequest, DeleteReqRequest, FriendRequestRowDto, FriendRowDto } from "../generated/bindings";

export async function createFriendRequest(
    request: CreateFriendReqRequest
) {
    return api<void>(
        "/friend/create_request", {
        method: "POST",
        body: JSON.stringify(request)
    }
    );
}

export async function acceptFriendRequest(
    request: AcceptReqRequest
) {
    return api<void>(
        "/friend/accept_request", {
        method: "POST",
        body: JSON.stringify(request)
    }
    );
}

export async function deleteFriendRequest(
    request: DeleteReqRequest
) {
    return api<void>(
        "/friend/delete_request", {
        method: "DELETE",
        body: JSON.stringify(request)
    }
    );
}

export async function getSentFriendRequests(): Promise<FriendRequestRowDto[]> {
    return api<FriendRequestRowDto[]>(
        "/friend/sent", {
        "method": "GET"
    }
    );
}

export async function getReceivedFriendRequests(): Promise<FriendRequestRowDto[]> {
    return api<FriendRequestRowDto[]>(
        "/friend/received", {
        "method": "GET"
    }
    );
}

export async function getFriends(): Promise<FriendRowDto[]> {
    return api<FriendRowDto[]>(
        "/friend", {
        "method": "GET"
    }
    );
}
