import { login } from "../api/auth";
import { useState } from "react";
import { useAuthStore } from "../stores/authStore";
import { Link, useNavigate } from "react-router-dom";

export default function LoginPage() {
    const setAccessToken = useAuthStore((state) => state.setAccessToken);
    const setUser = useAuthStore((state) => state.setUser);
    const [identifier, setIdentifier] = useState("");
    const [password, setPassword] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    async function handleSubmit(e: React.SubmitEvent<HTMLElement>) {
        e.preventDefault()
        setError(null);
        setLoading(true);

        try {
            const response = await login({
                identifier,
                password
            });

            setAccessToken(response.access_token);
            setUser(response.user);

            navigate("/");
        } catch (err) {
            if (err instanceof Error) {
                setError(err.message);
            } else {
                setError("Something went wrong!");
            }
        } finally {
            setLoading(false);
        }
    }

    return (
        <main>
            <div className="items-center">
                <div className="flex min-h-screen items-center justify-center">
                    <form onSubmit={handleSubmit} className="w-full max-w-md p-6 text-white">
                        <div className="flex w-full flex-col">
                            <h2 className="mt-2 text-center">Login</h2>
                            <div className="flex flex-col space-y-2 p-2 text-sm">

                                <label htmlFor="identifier">Email or Username</label>
                                <input id="identifier" type="identifier" value={identifier} onChange={(e) => setIdentifier(e.target.value)} required className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" />

                                <label htmlFor="password">Password</label>
                                <input id="password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} required className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" />

                                {error && <p>{error}</p>}

                                <button type="submit" className="mt-3 w-full rounded-2xl border-1 p-2.5 text-center transition-colors duration-200 hover:bg-white hover:text-blue-900" disabled={loading}>
                                    {loading ? "Logging in..." : "Log In"}
                                </button>
                            </div>
                        </div>
                        <p className="text-center text-xs">
                            Don't have an account?{" "}
                            <Link to="/register" className="underline">
                                Create one here
                            </Link>{" "}
                            OR{" "}
                            <Link to="/guest" className="underline">
                                Use a guest account.
                            </Link>
                        </p>
                    </form>
                </div>
            </div>
        </main>
    )
}