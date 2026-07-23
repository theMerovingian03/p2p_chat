import { useAuthStore } from "../stores/authStore";
import { Link } from "react-router-dom";

export default function HomePage() {
    const user = useAuthStore((state) => state.user);

    return (
        <main>
            <div className="items-center">
                <div className="flex flex-col min-h-screen items-center justify-center text-white">
                    <h2 className="text-center">Currently logged in as:</h2>
                    <ul className="text-s mt-2 list-disc pl-5">
                        <li>Display Name: {user?.display_name}</li>
                        <li>Username: {user?.username}</li>
                        <li>Email: {user?.email}</li>
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
                </div>
            </div>
        </main>
    )
}