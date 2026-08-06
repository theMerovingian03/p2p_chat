type RequestBoxProps = {
    username: string;
    createdAt: string;
};

export default function RequestBox({ username, createdAt }: RequestBoxProps) {
    return (
        <div className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10">
            <div className="flex flex-col gap-1">
                <span>{username}</span>
                <span className="text-xs text-white/50">Sent on {new Date(createdAt).toLocaleDateString()}</span>
            </div>
            <button className="cursor-pointer rounded px-2 py-1 text-sm transition-colors duration-200 hover:bg-white hover:text-blue-900">⋮</button>
        </div>
    );
}
