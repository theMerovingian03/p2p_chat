import { useWebsocketStore } from "../../stores/webSocketStore"
import StatusIndicator from "../StatusIndicator";

export default function WsConnectionBox() {
    const status = useWebsocketStore((state) => state.status);

    const statusText = {
        connecting: {
            text: "Establishing connection...",
        },
        connected: {
            text: "Online",
        },
        disconnected: {
            text: "Offline",
        },
    }[status];

    return (
        <div className="flex w-full transition-colors duration-200 flex-col gap-2 rounded-sm border border-white/20 p-3 text-white hover:bg-white/10">
            <div className="flex items-center gap-2">
                <StatusIndicator status={status} />
                <span>{statusText.text}</span>
            </div>
        </div>
    )
}