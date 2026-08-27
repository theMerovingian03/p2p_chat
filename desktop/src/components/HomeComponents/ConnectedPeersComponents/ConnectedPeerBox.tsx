import { sendMessage } from "../../../api/data_channel";

type ConnectedPeerBoxProps = {
    peerId: string;
    // username: string;
}

export default function ConnectedPeerBox({ peerId }: ConnectedPeerBoxProps) {
    return (
        <div
            className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10"
            onClick={() => sendMessage(peerId, "Hello")}
        >
            <div className="flex items-center gap-3">
                <span>{peerId}</span>
            </div>
        </div>
    )
}