import { useNavigate } from "react-router-dom";

type ConnectedPeerBoxProps = {
    peerId: string;
    // username: string;
}

export default function ConnectedPeerBox({ peerId }: ConnectedPeerBoxProps) {
    const navigate = useNavigate();
    return (
        <div
            className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10"
            onClick={() => navigate(`/chat/${peerId}`)}
        >
            <div className="flex items-center gap-3">
                <span>{peerId}</span>
            </div>
        </div>
    )
}