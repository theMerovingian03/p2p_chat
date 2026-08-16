import { useEffect, useRef, useState } from "react";
import SendChatRequestDialog from "./SendChatRequestDialog";

type FriendBoxProps = {
    username: string;
    userId: string;
};

export default function FriendBox({ username, userId }: FriendBoxProps) {
    const [dialog, setDialog] = useState<{ visible: boolean; x: number; y: number }>({
        visible: false,
        x: 0,
        y: 0,
    });
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

    return (
        <div ref={menuRef} className="relative">
            <div className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10">
                <div className="flex items-center gap-3">
                    <span>{username}</span>
                </div>
                <button
                    type="button"
                    onClick={(e) => setDialog({ visible: true, x: e.clientX, y: e.clientY })}
                    className="cursor-pointer rounded px-2 py-1 transition-colors duration-200 hover:bg-white hover:text-blue-900"
                >
                    ⋮
                </button>
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
                        onClose={() => setDialog({ visible: false, x: 0, y: 0 })}
                    />
                </div>
            )}
        </div>
    );
}