import { useState } from "react"
import { sendMessage } from "../../api/data_channel"
import { useDataChannelStore } from "../../stores/dataChannelStore";

interface ChatInputBoxProps {
    peerId: string;
}

export default function ChatInputBox({ peerId }: ChatInputBoxProps) {
    const [message, setMessage] = useState("");

    async function handleSend() {
        const content = message.trim;

        if (!content) {
            return;
        }

        try {
            await sendMessage(peerId, message);
            useDataChannelStore.getState().addOutgoingMessage(peerId, message);
            setMessage("");
        } catch (error) {
            console.error("Failed to send message: ", error);
        }
    }

    return (
        <div
            className="flex h-10 w-full cursor-text flex-row items-center rounded-sm border border-white/20 p-2 shadow-sm backdrop-blur-2xl transition-colors duration-200 hover:bg-white/10">
            <input className="ml-2 text-white/70" type="text" value={message} onChange={(e) => setMessage(e.target.value)}
                onKeyDown={(e) => {
                    if (e.key === "Enter") {
                        handleSend();
                    }
                }}
                placeholder={`Message ${peerId}`}
            />
        </div>
    )
}