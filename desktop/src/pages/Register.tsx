import { useState } from "react";

import { register } from "../api/auth";

export default function RegisterPage() {
    const [username, setUsername] = useState("");
    const [display_name, setDisplayName] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    async function handleSubmit(e: React.SubmitEvent<HTMLFormElement>) {
        e.preventDefault()

        setLoading(true);
        setError(null);

        try {
            const response = await register({
                username,
                display_name,
                email,
                password,
            });

            console.log("Registered:", response);

            // TODO:
            // - Save access token
            // - Save user
            // - Navigate to home page
        } catch (err) {
            if (err instanceof Error) {
                setError(err.message);
            } else {
                setError("Something went wrong.");
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
                            <h2 className="mt-2 text-center">Create Account</h2>
                            <div className="flex flex-col space-y-2 p-2 text-sm">

                                <label htmlFor="email">Email</label>
                                <input id="email" type="email" value={email} onChange={(e) => setEmail(e.target.value)} required className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" />

                                <label htmlFor="username">Username</label>
                                <input id="username" className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" value={username} onChange={(e) => setUsername(e.target.value)} required />

                                <label htmlFor="displayName">Display Name</label>
                                <input id="displayName" className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" value={display_name} onChange={(e) => setDisplayName(e.target.value)} required />


                                <label htmlFor="password">Password</label>
                                <input id="password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} required className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" />

                                <label htmlFor="confirmPassword">Confirm Password</label>
                                <input id="confirmPassword" type="password" value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} required className="rounded-bl-2xl border-b-1 border-l p-1 pb-2 pl-2 focus:outline-0" />

                                {error && <p>{error}</p>}

                                <button type="submit" className="mt-3 w-full rounded-2xl border-1 p-2.5 text-center" disabled={loading}>
                                    {loading ? "Creating account..." : "Register"}
                                </button>
                            </div>
                        </div>
                        <p className="text-center text-xs">Already have an account? Login Here</p>
                    </form>
                </div>
            </div>
        </main>
    );
}