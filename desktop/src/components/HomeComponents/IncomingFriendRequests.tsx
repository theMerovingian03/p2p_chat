import { useEffect, useState } from "react";
import IncomingFriendRequestBox from "./IncomingFriendRequestBox";
import { FriendRequestRowDto } from "../../generated/bindings";
import { getReceivedFriendRequests } from "../../api/friend";
import Spinner from "../Spinner";

type IncomingFriendRequestsProps = {};

export default function IncomingFriendRequests({}: IncomingFriendRequestsProps) {
    const [requests, setRequests] = useState<FriendRequestRowDto[]>([]);
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        void loadIncomingRequests();
    }, []);

    async function loadIncomingRequests() {
        setLoading(true);
        setError("");

        try {
            const incomingRequests = await getReceivedFriendRequests();
            setRequests(incomingRequests ?? []);
        } catch (err) {
            console.error(err);
            setError("Unable to load incoming requests.");
        } finally {
            setLoading(false);
        }
    }

    return (
        <div className="no-scrollbar max-h-140 w-full flex-col overflow-y-auto rounded-b-sm border border-white/20 scroll-smooth">
            {loading ? (
                <div className="flex items-center justify-center p-6">
                    <Spinner />
                </div>
            ) : error ? (
                <div className="p-4 text-sm text-red-400">{error}</div>
            ) : requests.length === 0 ? (
                <div className="p-4 text-sm text-white/70">
                    You have no incoming friend requests.
                </div>
            ) : (
                requests.map((request) => (
                    <IncomingFriendRequestBox
                        key={request.id}
                        id={request.id}
                        username={request.username}
                        createdAt={request.created_at}
                        onAction={() => void loadIncomingRequests()}
                    />
                ))
            )}
        </div>
    );
}
