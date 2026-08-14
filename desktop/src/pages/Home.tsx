import { useAuthStore } from "../stores/authStore";
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import me from "../api/user";
// import { getWsToken } from "../api/auth";

import FriendList from "../components/HomeComponents/FriendSidebarComponents/FriendList";
import FriendToggleButton from "../components/HomeComponents/FriendSidebarComponents/FriendToggle";
import IncomingFriendRequests from "../components/HomeComponents/IncomingFriendRequests";
import UserInfoBox from "../components/UserComponents/UserInfoBox";
import SearchBar from "../components/HomeComponents/FriendSearchComponents/SearchBar";
import SearchResultList from "../components/HomeComponents/FriendSearchComponents/SearchResultList";
import CommonButton from "../components/HomeComponents/CommonButtons";

export default function HomePage() {
    const logout = useAuthStore((state) => state.logout);
    const [error, setError] = useState("");
    // const [wsError, setWsError] = useState("");
    // const [wsToken, setWsToken] = useState("");
    // const [connectedToWs, setConnectedToWs] = useState(false);
    const [viewMode, setViewMode] = useState<"friends" | "requests">("friends");
    const navigate = useNavigate();
    const user = useAuthStore((state) => state.user);
    const setUser = useAuthStore((state) => state.setUser);

    async function handleLogOut() {
        try {
            logout();
            navigate("/login", { replace: true });
        } catch (err) {
            setError("An error occured!");
            console.error("Failed to delete refresh token!", err);
        }
    }

    async function loadUser() {
        try {
            const currentUser = await me();
            setUser(currentUser);
        } catch (err) {
            setError("An error occured!");
            console.error(err);
        }
    }

    // async function getWebsocketToken() {
    //     try {
    //         const response = await getWsToken()
    //         setWsToken(response.ws_token);
    //         setConnectedToWs(true);
    //     } catch (err) {
    //         setWsError("Error occured while establishing connection!");
    //         console.error(err);
    //     }
    // }

    // export async function connectWebSocket() {
    //     const { ws_token } = await getWsToken();

    //     websocketService.connect(
    //         ws_token,
    //         useWebSocketStore.getState().handleEvent,
    //     );
    // }

    useEffect(() => {
        if (!user) {
            loadUser();
        }
        // if (!connectedToWs) {
        //     getWebsocketToken();
        //     // TODO: Connect to WS, update dependency array
        // }
    }, [user, setUser]);

    return (
        <main>
            <div className="items-center">
                <div className="m-3 flex min-h-screen items-start justify-center">
                    <div className="m-5 flex w-full">
                        {/* Left sidebar */}
                        <div className="min-h-full w-1/3 p-2 gap-2">
                            <FriendToggleButton
                                activeView={viewMode}
                                onChange={(view) => setViewMode(view)}
                            />
                            <div className="flex flex-col gap-2">

                                <FriendList view={viewMode} />
                                <IncomingFriendRequests />
                            </div>
                        </div>
                        {/* Search bar + center portion */}
                        <div className="flex w-1/2 flex-col gap-2 p-2">
                            <SearchBar />
                            <SearchResultList />
                        </div>
                        {/* Right sidebar for settings, logout, etc */}
                        <div className="flex min-h-full w-1/4 flex-col p-2">
                            {!error && user && <UserInfoBox user={user} />}
                            <CommonButton>Change Alias</CommonButton>
                            <CommonButton onClick={handleLogOut}>Log Out</CommonButton>
                            {error && <p className="text-white m-2">{error}</p>}
                        </div>
                    </div>
                </div>
            </div>
        </main>
    )
}