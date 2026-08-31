type StatusIndicatorProps = {
    status: "connected" | "connecting" | "disconnected";
};

const statusStyles = {
    connected: {
        className: "bg-green-500",
        boxShadow:
            "inset 0 1px 1px rgba(255,255,255,0.55), inset 0 -1px 1px rgba(0,0,0,0.25), 0 0 6px rgba(34,197,94,0.5)",
        background: "radial-gradient(circle at 35% 35%, #4ade80, #16a34a)",
    },
    connecting: {
        className: "animate-pulse bg-orange-400",
        boxShadow:
            "inset 0 1px 1px rgba(255,255,255,0.5), inset 0 -1px 1px rgba(0,0,0,0.2), 0 0 5px rgba(251,146,60,0.4)",
        background: "radial-gradient(circle at 35% 35%, #fdba74, #ea580c)",
    },
    disconnected: {
        className: "bg-gray-400",
        boxShadow:
            "inset 0 1px 1px rgba(255,255,255,0.3), inset 0 -1px 1px rgba(0,0,0,0.3)",
        background: "radial-gradient(circle at 35% 35%, #9ca3af, #4b5563)",
    },
} satisfies Record<StatusIndicatorProps["status"], { className: string; boxShadow: string; background: string }>;

export default function StatusIndicator({ status }: StatusIndicatorProps) {
    const styles = statusStyles[status];

    return (
        <span
            className={`h-2.5 w-2.5 rounded-full ${styles.className}`}
            style={{
                boxShadow: styles.boxShadow,
                background: styles.background,
            }}
        />
    );
}
