import { useAuthStore } from "../stores/authStore";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";
import me from "../api/user";

import FriendList from "../components/HomeComponents/FriendSidebarComponents/FriendList"
import FriendToggleButton from "../components/HomeComponents/FriendSidebarComponents/FriendToggle"
import UserInfoBox from "../components/UserComponents/UserInfoBox";
import SearchBar from "../components/HomeComponents/SearchBar";
import CommonButton from "../components/HomeComponents/CommonButtons";

export default function HomePage() {
    const logout = useAuthStore((state) => state.logout);
    const [error, setError] = useState("");
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

    useEffect(() => {
        async function loadUser() {
            try {
                const currentUser = await me();
                setUser(currentUser);
            } catch (err) {
                console.error(err);
            }
        }

        if (!user) {
            loadUser();
        }
    }, [user, setUser]);

    return (
        <main>
            <div className="items-center">
                <div className="m-3 flex min-h-screen items-start justify-center">
                    <div className="m-5 flex w-full">
                        {/* Left sidebar */}
                        <div className="min-h-full w-1/3 p-2">
                            <FriendToggleButton />
                            <FriendList />
                        </div>
                        {/* Search bar + center portion */}
                        <div className="flex min-h-full w-1/2 flex-col gap-10">
                            <SearchBar />
                        </div>
                        {/* Right sidebar for settings, logout, etc */}
                        <div className="flex flex-col min-h-full w-1/4 p-2">
                            <UserInfoBox user={user} />
                            <CommonButton>Change Alias</CommonButton>
                            <CommonButton onClick={handleLogOut}>Log Out</CommonButton>
                            {error && <p>{error}</p>}
                        </div>
                    </div>
                </div>
            </div>
        </main>
    )
}