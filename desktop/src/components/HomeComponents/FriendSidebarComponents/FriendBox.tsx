import { useEffect, useRef, useState } from "react";
import SendChatRequestDialog from "./SendChatRequestDialog";
import StatusIndicator from "../../StatusIndicator";
import SnackBar from "../../SnackBar";
// import { useWebsocketStore } from "../../../stores/webSocketStore";

type FriendBoxProps = {
    username: string;
    userId: string;
    isOnline: boolean;
};

export default function FriendBox({ username, userId, isOnline }: FriendBoxProps) {
    // const isOnline = useWebsocketStore((state) => state.onlineUserIds.has(userId));
    const [dialog, setDialog] = useState<{ visible: boolean; x: number; y: number }>({
        visible: false,
        x: 0,
        y: 0,
    });
    const [showOfflineSnackbar, setShowOfflineSnackbar] = useState(false);
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
                setDialog({ visible: false, x: 0, y: 0 });
            }
        }

        function handleScroll() {
            setDialog({ visible: false, x: 0, y: 0 });
        }

        document.addEventListener("mousedown", handleClick);
        window.addEventListener("scroll", handleScroll, true);

        return () => {
            document.removeEventListener("mousedown", handleClick);
            window.removeEventListener("scroll", handleScroll, true);
        };
    }, []);

    useEffect(() => {
        if (!isOnline) {
            setDialog({ visible: false, x: 0, y: 0 });
        }
    }, [isOnline]);

    useEffect(() => {
        if (!showOfflineSnackbar) return;

        const timer = setTimeout(() => {
            setShowOfflineSnackbar(false);
        }, 3000);

        return () => clearTimeout(timer);
    }, [showOfflineSnackbar]);

    return (
        <div ref={menuRef} className="relative">
            <div className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10"
                onClick={(e) => {
                    if (isOnline) {
                        setDialog({ visible: true, x: e.clientX + 10, y: e.clientY });
                    } else {
                        setShowOfflineSnackbar(true);
                    }
                }}
            >
                <div className="flex items-center gap-3">
                    {/* <StatusIndicator status={"connecting"} /> */}
                    <StatusIndicator status={isOnline ? "connected" : "disconnected"} />
                    <span>{username}</span>
                </div>
            </div>

            {dialog.visible && (
                <div
                    className="fixed z-50"
                    style={{
                        left: dialog.x,
                        top: dialog.y,
                    }}
                >
                    <SendChatRequestDialog
                        username={username}
                        userId={userId}
                        isOnline={isOnline}
                        onClose={() => setDialog({ visible: false, x: 0, y: 0 })}
                    />
                </div>
            )}

            {showOfflineSnackbar && (
                <SnackBar message="This friend is offline. Chat requests can only be sent to online friends!" />
            )}
        </div>
    );
}