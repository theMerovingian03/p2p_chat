import FriendBox from "./FriendBox";
import RequestBox from "./RequestBox";
import { useEffect, useState } from "react";
import { FriendRequestRowDto } from "../../../generated/bindings";
import { getSentFriendRequests } from "../../../api/friend";
import Spinner from "../../Spinner";
import { useFriendStore } from "../../../stores/friendStore";

type FriendListProps = {
    view: "friends" | "requests";
};

export default function FriendList({ view }: FriendListProps) {
    // const [friends, setFriends] = useState<FriendRowDto[]>([]);
    const friends = useFriendStore((state) => state.friends);

    const [requests, setRequests] = useState<FriendRequestRowDto[]>([]);
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        void handleLoadData();
    }, [view]);

    async function handleLoadData() {
        setLoading(true);
        setError("");

        try {
            if (view === "requests") {
                const sentRequests = await getSentFriendRequests();
                setRequests(sentRequests ?? []);
                // setFriends([]);
            }
            // else {
            //     const friendList = await getFriends();
            //     setFriends(friendList ?? []);
            //     setRequests([]);
            // }
        } catch (err) {
            console.error(err);
            setError("Unable to load data.");
        } finally {
            setLoading(false);
        }
    }

    const emptyMessage =
        view === "requests"
            ? "You have not sent any friend requests yet."
            : "Could not find active contacts. Try using the search bar to find friends.";

    return (
        <div className="no-scrollbar max-h-85 w-full flex-col overflow-y-auto rounded-b-sm border border-white/20 scroll-smooth">
            {loading ? (
                <div className="flex items-center justify-center p-6">
                    <Spinner />
                </div>
            ) : error ? (
                <div className="p-4 text-sm text-red-400">{error}</div>
            ) : view === "requests" ? (
                requests.length === 0 ? (
                    <div className="p-4 text-sm text-white/70">{emptyMessage}</div>
                ) : (
                    requests.map((request) => (
                        <RequestBox
                            key={request.id}
                            id={request.id}
                            username={request.username}
                            createdAt={request.created_at}
                            onDelete={() => void handleLoadData()}
                        />
                    ))
                )
            ) : Object.values(friends).length === 0 ? (
                <div className="p-4 text-sm text-white/70">{emptyMessage}</div>
            ) : (
                Object.values(friends).map((friend) => (
                    <FriendBox key={friend.friend_id} username={friend.username} userId={friend.friend_id} isOnline={friend.isOnline} />
                ))
            )}
        </div>
    );
}
