import { useRef, useState, useEffect } from "react";
import { deleteFriendRequest } from "../../../api/friend";

type RequestBoxProps = {
    id: string;
    username: string;
    createdAt: string;
    onDelete?: () => void;
};

export default function RequestBox({ id, username, createdAt, onDelete }: RequestBoxProps) {
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

    async function handleDecline() {
        setLoading(true);
        try {
            await deleteFriendRequest({ request_id: id });
            onDelete?.();
        } catch (err) {
            console.error(err);
        } finally {
            setLoading(false);
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
                    <span className="text-xs text-white/50">Sent on {new Date(createdAt).toLocaleDateString()}</span>
                </div>
            </div>

            {dialog.visible && (
                <div
                    className="fixed z-50"
                    style={{ left: dialog.x, top: dialog.y }}
                >
                    <div className="w-32 rounded border border-white/20 text-sm bg-white/10 p-1 text-white transition-colors duration-200 hover:bg-white hover:text-blue-900 backdrop-blur-md">
                        <button onClick={handleDecline} disabled={loading} className="w-full text-left px-2 py-1">{loading ? 'Cancelling...' : 'Cancel'}</button>
                    </div>
                </div>
            )}
        </div>
    );
}
