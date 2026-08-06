type FriendBoxProps = {
    username: string;
};

export default function FriendBox({ username }: FriendBoxProps) {
    return (
        <div className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10">
            <div className="flex items-center gap-3">
                <span>{username}</span>
            </div>
            <button className="cursor-pointer rounded px-2 py-1 transition-colors duration-200 hover:bg-white hover:text-blue-900">⋮</button>
        </div>
    );
}