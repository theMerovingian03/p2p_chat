import { useWebsocketStore } from "../../stores/webSocketStore"

export default function WsConnectionBox() {
    const status = useWebsocketStore((state) => state.status);

    const statusConfig = {
        connecting: {
            text: "Establishing connection...",
            // Solid colors fallback
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
                {/* Skeomorphic look */}
                <span
                    className={`h-2.5 w-2.5 rounded-full ${statusConfig.color}`}
                    style={{
                        boxShadow: status === 'connected'
                            ? 'inset 0 1px 1px rgba(255,255,255,0.55), inset 0 -1px 1px rgba(0,0,0,0.25), 0 0 6px rgba(34,197,94,0.5)'
                            : status === 'connecting'
                                ? 'inset 0 1px 1px rgba(255,255,255,0.5), inset 0 -1px 1px rgba(0,0,0,0.2), 0 0 5px rgba(251,146,60,0.4)'
                                : 'inset 0 1px 1px rgba(255,255,255,0.3), inset 0 -1px 1px rgba(0,0,0,0.3)',
                        background: status === 'connected'
                            ? 'radial-gradient(circle at 35% 35%, #4ade80, #16a34a)'
                            : status === 'connecting'
                                ? 'radial-gradient(circle at 35% 35%, #fdba74, #ea580c)'
                                : 'radial-gradient(circle at 35% 35%, #9ca3af, #4b5563)',
                    }}
                />
                <span>{statusConfig.text}</span>
            </div>
        </div>
    )
}