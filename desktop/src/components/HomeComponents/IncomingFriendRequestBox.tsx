import { useRef, useState, useEffect } from "react";
import { acceptFriendRequest, deleteFriendRequest } from "../../api/friend";

type RequestBoxProps = {
    id: string;
    username: string;
    createdAt: string;
    onAction?: () => void;
};

export default function IncomingFriendRequestBox({ id, username, createdAt, onAction }: RequestBoxProps) {
    const [loading, setLoading] = useState(false);
    const [dialog, setDialog] = useState<{ visible: boolean; x: number; y: number }>({ visible: false, x: 0, y: 0 });
    const containerRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
                setDialog({ visible: false, x: 0, y: 0 });
            }
        }

        document.addEventListener("mousedown", handleClick);
        return () => document.removeEventListener("mousedown", handleClick);
    }, []);

    async function handleAccept() {
        setLoading(true);
        try {
            await acceptFriendRequest({ request_id: id });
            onAction?.();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading(false);
            setDialog({ visible: false, x: 0, y: 0 });
        }
    }

    async function handleDecline() {
        setLoading(true);
        try {
            await deleteFriendRequest({ request_id: id });
            onAction?.();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading(false);
            setDialog({ visible: false, x: 0, y: 0 });
        }
    }

    return (
        <div ref={containerRef} className="relative">
            <div className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10">
                <div className="flex flex-col gap-1">
                    <span>{username}</span>
                    <span className="text-xs text-white/50">Received on {new Date(createdAt).toLocaleDateString()}</span>
                </div>
                <button onClick={(e) => setDialog({ visible: true, x: e.clientX, y: e.clientY })} className="cursor-pointer rounded px-2 py-1 text-sm transition-colors duration-200 hover:bg-white hover:text-blue-900">⋮</button>
            </div>

            {dialog.visible && (
                <div className="fixed z-50" style={{ left: dialog.x, top: dialog.y }}>
                    <div className="w-32 rounded border border-white/20 text-sm bg-white/10 text-white backdrop-blur-md">
                        <button onClick={handleAccept} disabled={loading} className="w-full p-2 text-left transition-colors duration-200 hover:bg-white hover:text-blue-900">{loading ? 'Accepting...' : 'Accept'}</button>
                        <button onClick={handleDecline} disabled={loading} className="w-full p-2 text-left transition-colors duration-200 hover:bg-white hover:text-blue-900">{loading ? 'Declining...' : 'Decline'}</button>
                    </div>
                </div>
            )}
        </div>
    );
}
