import FriendBox from "./FriendBox";
import { useEffect, useState } from "react";
import { FriendRowDto } from "../../../generated/bindings";
import { getFriends } from "../../../api/friend";
import Spinner from "../../Spinner";

export default function FriendList() {
    const [friends, setFriends] = useState<FriendRowDto[]>([]);
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        void handleGetFriends();
    }, []);

    async function handleGetFriends() {
        setLoading(true);
        setError("");

        try {
            const friend_list = await getFriends();
            setFriends(friend_list ?? []);
        } catch (err) {
            console.error(err);
            setError("Unable to load friends.");
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
            ) : friends.length === 0 ? (
                <div className="p-4 text-sm text-white/70">Could not find active contacts Try using the search bar to find friends.</div>
            ) : (
                friends.map((friend) => (
                    <FriendBox key={friend.friend_id} username={friend.username} />
                ))
            )}
        </div>
    );
}