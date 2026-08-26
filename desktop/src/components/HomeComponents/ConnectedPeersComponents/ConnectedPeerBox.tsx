import { sendMessage } from "../../../api/data_channel";

export default function ConnectedPeerBox() {
    return (
        <div
            className="flex cursor-pointer items-center justify-between border-b border-white/20 p-3 text-white transition-colors duration-200 hover:bg-white/10"
            onClick={() => sendMessage("5fdb840c-4397-4303-862a-0f50a7a6a8ab", "Hello")}
        >
            <div className="flex items-center gap-3">
                <span>username</span>
            </div>
        </div>
    )
}