import { Outlet } from "react-router-dom";
import { useEffect, useState } from "react";
import Spinner from "./Spinner";
import CommonButton from "./CommonButtons";
import { useAuthStore } from "../stores/authStore";
import { useWebsocketStore } from "../stores/webSocketStore";
import { getWsToken } from "../api/auth";
import { webSocketService } from "../services/websocketService";
import { useNavigate } from "react-router-dom";
import { me } from "../api/user";
import { env } from "../config/env";

export default function AuthComponent() {
    const logout = useAuthStore((state) => state.logout);
    const navigate = useNavigate();
    const user = useAuthStore((state) => state.user);
    const setUser = useAuthStore((state) => state.setUser);
    const initializeEventListeners = useWebsocketStore((state) => state.initializeEventListeners);

    const [initializing, setInitializing] = useState(true);
    const [error, setError] = useState<string | null>(null);

    async function handleLogOut() {
        try {
            logout();
            navigate("/login", { replace: true });
        } catch (err) {
            setError("An error occured!");
            console.error("Failed to delete refresh token!", err);
        }
    }

    useEffect(() => {
        let cancelled = false;

        async function initialize() {
            try {
                // Initialize event listeners to receive WebSocket events from Rust
                await initializeEventListeners();

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

                // Establish ws connection through Tauri.
                console.log("Attempting to connect WS");
                await webSocketService.connect(ws_token, env.wsUrl);
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
            void webSocketService.disconnect();
        };
    }, [initializeEventListeners, setUser, user]);

    if (initializing) {
        return (
            <div className="flex min-h-screen items-center justify-center text-white">
                <div className="flex flex-row gap-2">
                    <Spinner />
                    <span>Establishing secure peer link...</span>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="flex min-h-screen items-center justify-center text-white">
                <div className="flex flex-col gap-2">
                    <span>{error}</span>
                    <CommonButton onClick={handleLogOut}>Retry Login</CommonButton>
                </div>
            </div>
        );
    }

    return <Outlet />;
}