import { useRef, useState, useEffect } from "react";
import { acceptFriendRequest, deleteFriendRequest } from "../../api/friend";

type RequestBoxProps = {
    id: string;
    username: string;
    createdAt: string;
    onAction?: () => void;
};

export default function IncomingFriendRequestBox({ id, username, createdAt, onAction }: RequestBoxProps) {
    const [loadingAccept, setLoadingAccept] = useState(false);
    const [loadingDecline, setLoadingDecline] = useState(false);
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
        setLoadingAccept(true);
        try {
            await acceptFriendRequest({ request_id: id });
            onAction?.();
        } catch (err) {
            console.error(err);
        } finally {
            setLoadingAccept(false);
            setDialog({ visible: false, x: 0, y: 0 });
        }
    }

    async function handleDecline() {
        setLoadingDecline(true);
        try {
            await deleteFriendRequest({ request_id: id });
            onAction?.();
        } catch (err) {
            console.error(err);
        } finally {
            setLoadingDecline(false);
            setDialog({ visible: false, x: 0, y: 0 });
        }
    }

    return (
        <div ref={containerRef} className="relative">
            <div className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10"
                onClick={(e) => setDialog({ visible: true, x: e.clientX + 10, y: e.clientY })}
            >
                <div className="flex flex-col gap-1">
                    <span>{username}</span>
                    <span className="text-xs text-white/50">Received on {new Date(createdAt).toLocaleDateString()}</span>
                </div>
            </div>

            {dialog.visible && (
                <div className="fixed z-50" style={{ left: dialog.x, top: dialog.y }}>
                    <div className="w-32 rounded border border-white/20 text-sm bg-white/10 text-white backdrop-blur-md">
                        <button onClick={handleAccept} disabled={loadingAccept} className="w-full p-2 text-left transition-colors duration-200 hover:bg-white hover:text-blue-900">{loadingAccept ? 'Accepting...' : 'Accept'}</button>
                        <button onClick={handleDecline} disabled={loadingDecline} className="w-full p-2 text-left transition-colors duration-200 hover:bg-white hover:text-blue-900">{loadingDecline ? 'Declining...' : 'Decline'}</button>
                    </div>
                </div>
            )}
        </div>
    );
}
