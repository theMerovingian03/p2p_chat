import { useRef, useState } from "react";
import type { UserSearchModel } from "../../../generated/bindings";
import SendFriendRequestDialog from "./SendRequestDialog";
import { useEffect } from "react";

interface SearchResultProps {
    user: UserSearchModel;
}

export default function SearchResult({ user }: SearchResultProps) {
    const [dialog, setDialog] = useState<{
        visible: boolean,
        x: number;
        y: number;
    }>({
        visible: false,
        x: 0,
        y: 0,
    });
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (
                menuRef.current &&
                !menuRef.current.contains(e.target as Node)
            ) {
                setDialog({ visible: false, x: 0, y: 0 });
            }
        }

        document.addEventListener("mousedown", handleClick);

        return () => {
            document.removeEventListener("mousedown", handleClick);
        };
    }, []);

    return (
        <div ref={menuRef} className="relative">
            <div
                onClick={(e) => setDialog({
                    visible: true,
                    x: e.clientX,
                    y: e.clientY,
                })
                }
                className="w-full cursor-pointer border-b border-white/20 p-2 text-white hover:bg-white/10">
                <span>{user.username}</span>
            </div>

            {dialog.visible && (
                <div
                    className="fixed z-50"
                    style={{
                        left: dialog.x,
                        top: dialog.y,
                    }}
                >
                    <SendFriendRequestDialog
                        username={user.username}
                        userId={user.id}
                        onClose={() => setDialog({ visible: false, x: 0, y: 0 })}
                    />
                </div>
            )}
        </div>
    );
}