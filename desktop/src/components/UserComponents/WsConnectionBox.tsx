import { useWebsocketStore } from "../../stores/webSocketStore"

export default function WsConnectionBox() {
    const status = useWebsocketStore((state) => state.status);

    const statusConfig = {
        connecting: {
            text: "Establishing connection...",
            color: "animate-pulse bg-orange-400",
        },
        connected: {
            text: "Connected",
            color: "bg-green-500"
        },
        disconnected: {
            text: "Disconnected",
            color: "bg-gray-400",
        },
    }[status];

    return (
        <div className="flex w-full transition-colors duration-200 flex-col gap-2 rounded-sm border border-white/20 p-3 text-white hover:bg-white/10">
            <div className="flex items-center gap-2">
                <span
                    className={`h-2.5 w-2.5 rounded-full ${statusConfig.color}`}
                />
                <span>{statusConfig.text}</span>
            </div>
        </div>
    )
}