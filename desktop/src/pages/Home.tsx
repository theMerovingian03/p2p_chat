import { useAuthStore } from "../stores/authStore";
import { useState } from "react";
import { Link } from "react-router-dom";
import { loadRefreshToken } from "../stores/tokenStore";

export default function HomePage() {
    const user = useAuthStore((state) => state.user);
    const [refToken, setRefToken] = useState("")
    const [error, setError] = useState("");

    async function showRefreshToken() {
        // e.preventDefault()
        try {
            let refreshToken = await loadRefreshToken();
            setRefToken(refreshToken);
        } catch (err) {
            // setError(er)
            console.error("Load refresh token failed:", err);
        }
    }

    return (
        <main>
            <div className="items-center">
                <div className="flex flex-col min-h-screen items-center justify-center text-white">
                    <h2 className="text-center">Currently logged in as:</h2>
                    <ul className="text-s mt-2 list-disc pl-5">
                        <li>Display Name: {user?.display_name}</li>
                        <li>Username: {user?.username}</li>
                        <li>Email: {user?.email}</li>
                        <li>Refresh Token: {refToken}</li>
                    </ul>
                    <p className="text-center text-xs mt-2">
                        Don't have an account?{" "}
                        <Link to="/register" className="underline">
                            Create one here
                        </Link>{" "}
                        OR{" "}
                        <Link to="/guest" className="underline">
                            Use a guest account.
                        </Link>
                    </p>
                    <button onClick={showRefreshToken} className="mt-3 w-50 rounded-2xl border-1 p-2.5 text-center transition-colors duration-200 hover:bg-white hover:text-blue-900">
                        Show refresh token
                    </button>
                    {error && <p>{error}</p>}
                </div>
            </div>
        </main >
    )
}