import { useState } from "react";
import { createFriendRequest } from "../../../api/friend";
import { useEffect } from "react";

interface SendFriendRequestProps {
    username: string,
    userId: string,
    onClose: () => void;
}

export default function SendFriendRequestDialog({ username, userId, onClose }: SendFriendRequestProps) {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState("");
    const [success, setSuccess] = useState(false);

    async function handleSendRequest() {
        try {
            setLoading(true);
            await createFriendRequest({
                receiver_id: userId,
            });
            setSuccess(true);
        } catch (err) {
            setError("An error occured!");
            console.log(err);
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        if (!success) return;

        const timer = setTimeout(() => {
            onClose();
        }, 1500);

        return () => clearTimeout(timer);
    }, [success, onClose]);

    return (
        <div className="flex w-80 flex-col rounded-sm border border-white/20 bg-white/10 text-white backdrop-blur-md
        shadow-xl">
            <span className="m-2">{username} is not in your contacts. Add as friend?</span>
            <div className="m-2 flex gap-5">
                <button
                    disabled={loading || success}
                    onClick={handleSendRequest}
                    className="cursor-pointer rounded-sm border border-white/20 p-1.5 transition-colors duration-200 hover:bg-white hover:text-blue-900 disabled:cursor-default disabled:opacity-75"
                >
                    {loading
                        ? "Sending..."
                        : success
                            ? "Sent!"
                            : "Send Request"
                    }
                </button>
                <button onClick={onClose}
                    className="cursor-pointer rounded-sm border border-white/20 p-1.5 transition-colors duration-200 hover:bg-white hover:text-blue-900">Cancel</button>
            </div>
        </div>
    );
}