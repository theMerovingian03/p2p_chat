import { guestLogin } from '../api/auth';
import { useState } from 'react';
import { useAuthStore } from '../stores/authStore';
import { Link, useNavigate } from 'react-router-dom';
import { saveRefreshToken } from '../stores/tokenStore';

export default function GuestLoginPage() {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const setAccessToken = useAuthStore((state) => state.setAccessToken);
    const setUser = useAuthStore((state) => state.setUser);
    const navigate = useNavigate();

    async function handleSubmit(e: React.SubmitEvent<HTMLElement>) {
        e.preventDefault()
        setError(null);
        setLoading(true);

        try {
            const response = await guestLogin();

            setAccessToken(response.access_token);
            setUser(response.user);
            await saveRefreshToken(response.refresh_token);

            navigate("/home");
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
            <div className='items-center'>
                <div className="flex min-h-screen items-center justify-center">
                    <form onSubmit={handleSubmit} className="w-full max-w-md rounded-sm p-6 text-white">
                        <div className="flex w-full flex-col">
                            <h2 className="mt-2 text-center">Guest Login</h2>
                            <div className="flex flex-col space-y-2 p-2 text-sm">
                                <div className="rounded-sm border-t border-r p-2.5 border-white/20">
                                    <p>Create a temporary account now!</p>
                                    <ul className="text-s mt-2 list-disc pl-5">
                                        <li>Full access to chat and messaging features</li>
                                        <li>No email, password, or signup needed</li>
                                        <li>Account automatically expires after 24 hours</li>
                                    </ul>
                                </div>
                                {error && <p>{error}</p>}
                                <button type="submit" className='border-white/20 mt-3 w-full rounded-sm border p-2.5 text-center transition-colors duration-200 hover:bg-white/10 cursor-pointer shadow-md'>
                                    {loading ? "Getting temporary credentials..." : "Let's Go!"}
                                </button>
                            </div>
                        </div>
                        <p className="text-center text-xs">
                            Already have an account?{" "}
                            <Link to={"/login"} className="underline">
                                Login Here{" "}
                            </Link>
                            OR{" "}
                            <Link to={"/register"} className="underline">
                                Create a new account
                            </Link>
                        </p>
                    </form>
                </div>
            </div>
        </main>
    )
}