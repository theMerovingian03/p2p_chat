import { Outlet } from "react-router-dom";
import { useEffect, useState } from "react";
import Spinner from "./Spinner";
import { useAuthStore } from "../stores/authStore";
import { WebsocketStore } from "../stores/webSocketStore";
import { getWsToken } from "../api/auth";
import { webSocketService } from "../services/websocketService";
import me from "../api/user";

export default function AuthComponent() {
    const user = useAuthStore((state) => state.user);
    const setUser = useAuthStore((state) => state.setUser);
    const handleEvent = WebsocketStore((state) => state.handleEvent);

    const [initializing, setInitializing] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        let cancelled = false;

        async function initialize() {
            try {
                if (!user) {
                    console.log("Fetching user details");
                    const currentUser = await me();

                    if (cancelled) return;

                    setUser(currentUser);
                    console.log("Fetched user details successfully!");
                }

                // Get websocket token.
                console.log("Requesting WS Token")
                const { ws_token } = await getWsToken();

                if (cancelled) return;

                // Establish ws connection.
                console.log("Attempting to connect WS");
                webSocketService.connect(
                    ws_token,
                    handleEvent,
                );
            } catch (error) {
                if (cancelled) return;

                console.error("Failed to initialize application:", error);
                setError("Failed to initialize application.");
            } finally {
                if (!cancelled) {
                    setInitializing(false);
                }
            }
        }

        void initialize();

        return () => {
            cancelled = true;
            webSocketService.disconnect();
        };
    }, [handleEvent, setUser, user]);

    if (initializing) {
        return (
            <div className="flex min-h-screen items-center justify-center text-white">
                <div className="flex flex-row gap-2">
                    <Spinner />
                    <span className="mt-2">Establishing secure peer link...</span>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="flex min-h-screen items-center justify-center text-white">
                {error}
            </div>
        );
    }

    return <Outlet />;
}