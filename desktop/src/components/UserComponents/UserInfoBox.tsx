import { ProfileCardProps } from "./Props";

export default function UserInfoBox({ user }: ProfileCardProps) {
    if (!user) {
        return <p>Loading...</p>;
    }
    return (
        <div className="flex w-full flex-col gap-2 rounded-2xl border border-white p-2 text-white">
            <span>Welcome, {user.username}</span>
            <span className="text-xs">Alias: {user.display_name}</span>
            <span className="text-xs">{user.email}</span>
            {/* <span className="text-xs">Last used: DDMMYYY</span>
            <span className="text-xs">Guest: NO</span> */}
        </div>
    )
}