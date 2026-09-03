import { useEffect, useState } from "react";
import { sendChatRequest } from "../../../services/websocketService";

interface SendChatRequestProps {
    username: string;
    userId: string;
    isOnline: boolean;
    onClose: () => void;
}

export default function SendChatRequestDialog({ username, userId, isOnline, onClose }: SendChatRequestProps) {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState("");
    const [success, setSuccess] = useState(false);

    async function handleSendRequest() {
        if (!isOnline) return;

        try {
            setLoading(true);
            setError("");
            await sendChatRequest(userId);
            setSuccess(true);
        } catch (err) {
            setError("An error occured!");
            console.error(err);
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
        <div
            className="flex w-60 flex-col rounded-sm  bg-white/10 text-white shadow-lg backdrop-blur-md text-sm p-1"
        >
            <span className="mx-2 mt-2">Send a chat request to {username}?</span>
            {error && <span className="mx-1 text-sm text-red-300">{error}</span>}
            <div className="m-2 flex gap-3">
                <button
                    disabled={loading || success || !isOnline}
                    onClick={handleSendRequest}
                    className="cursor-pointer rounded-sm border border-white/20 p-1.5 transition-colors duration-200 hover:bg-white hover:text-blue-900 disabled:cursor-default disabled:opacity-75"
                >
                    {loading ? "Sending..." : success ? "Sent!" : "Send"}
                </button>
                <button
                    onClick={onClose}
                    className="cursor-pointer rounded-sm border border-white/20 p-1.5 transition-colors duration-200 hover:bg-white hover:text-blue-900"
                >
                    Cancel
                </button>
            </div>
        </div>
    );
}
