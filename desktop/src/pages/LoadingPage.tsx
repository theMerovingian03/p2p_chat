import Spinner from "../components/Spinner";
import { refreshSession } from "../api/auth";
import { useAuthStore } from "../stores/authStore";
import { saveRefreshToken, loadRefreshToken } from "../stores/tokenStore";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";

export default function LoadingPage() {
    const navigate = useNavigate();
    const setAccessToken = useAuthStore((state) => state.setAccessToken);
    const logout = useAuthStore((state) => state.logout);

    async function initializeAuth() {
        try {
            const refreshToken = await loadRefreshToken();
            if (!refreshToken) {
                navigate("/login");
            };

            const response = await refreshSession({
                refresh_token: refreshToken
            });

            setAccessToken(response.access_token);
            await saveRefreshToken(response.refresh_token);

            navigate("/home");
        } catch (err) {
            logout();
            console.log(err)
            navigate("/login");
        }
    }

    useEffect(() => {
        initializeAuth();
    }, []);


    return (
        <main>
            <div className="items-center">
                <div className="flex min-h-screen items-center justify-center">
                    <Spinner />
                </div>
            </div>
        </main>
    )
}