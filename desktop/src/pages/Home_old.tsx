// import { useAuthStore } from "../stores/authStore";
// import { useState } from "react";
// import { Link, useNavigate } from "react-router-dom";
// import { loadRefreshToken } from "../stores/tokenStore";
// import { useEffect } from "react";
// import me from "../api/user";
// import UserInfoBox from "../components/UserComponents/UserInfoBox";

// export default function HomePage() {
//     // const user = useAuthStore((state) => state.user);
//     const logout = useAuthStore((state) => state.logout);
//     const [refToken, setRefToken] = useState("")
//     const [error, setError] = useState("");
//     const navigate = useNavigate();
//     const user = useAuthStore((state) => state.user);
//     const setUser = useAuthStore((state) => state.setUser);

//     async function showRefreshToken() {
//         try {
//             let refreshToken = await loadRefreshToken();
//             setRefToken(refreshToken);
//         } catch (err) {
//             setError("An error occured!");
//             console.error("Load refresh token failed:", err);
//         }
//     }

//     async function handleLogOut() {
//         try {
//             logout();
//             navigate("/login", { replace: true });
//         } catch (err) {
//             console.error("Failed to delete refresh token!", err);
//         }
//     }

//     useEffect(() => {
//         async function loadUser() {
//             try {
//                 const currentUser = await me();
//                 setUser(currentUser);
//             } catch (err) {
//                 console.error(err);
//             }
//         }

//         if (!user) {
//             loadUser();
//         }
//     }, [user, setUser]);

//     return (
//         <main>
//             <div className="items-center">
//                 <div className="flex flex-col min-h-screen items-center justify-center text-white">
//                     <UserInfoBox user={user} />
//                     <p className="text-center text-xs mt-2">
//                         Don't have an account?{" "}
//                         <Link to="/register" className="underline">
//                             Create one here
//                         </Link>{" "}
//                         OR{" "}
//                         <Link to="/guest" className="underline">
//                             Use a guest account.
//                         </Link>
//                     </p>
//                     <div className="flex justify-around p-2 w-full">

//                         <button onClick={showRefreshToken} className="m-3 w-50 rounded-2xl border p-2.5 text-center transition-colors duration-200 hover:bg-white hover:text-blue-900">
//                             Show refresh token
//                         </button>

//                         <button onClick={handleLogOut} className="m-3 w-50 rounded-2xl border p-2.5 text-center transition-colors duration-200 hover:bg-white hover:text-blue-900">
//                             Log Out
//                         </button>
//                     </div>
//                     {error && <p>{error}</p>}
//                 </div>
//             </div>
//         </main >
//     )
// }