interface SendFriendRequestProps {
    username: string,
    userId: string,
    onClose: () => void;
}

export default function SendFriendRequestDialog({ username, userId, onClose }: SendFriendRequestProps) {
    return (
        <div className="flex w-80 flex-col rounded-sm border border-white/20 bg-white/10 text-white backdrop-blur-md
        shadow-xl">
            <span className="m-2">{username} is not in your contacts. Add as friend? {userId}</span>
            <div className="m-2 flex gap-5">
                <button
                    className="cursor-pointer rounded-sm border border-white/20 p-1.5 transition-colors duration-200 hover:bg-white hover:text-blue-900">Send
                    Request</button>
                <button onClick={onClose}
                    className="cursor-pointer rounded-sm border border-white/20 p-1.5 transition-colors duration-200 hover:bg-white hover:text-blue-900">Cancel</button>
            </div>
        </div>
    );
}