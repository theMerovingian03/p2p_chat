import { useState } from "react"
import { sendMessage } from "../../api/data_channel"
import { useDataChannelStore } from "../../stores/dataChannelStore";

interface ChatInputBoxProps {
    peerId: string;
}

export default function ChatInputBox({ peerId }: ChatInputBoxProps) {
    const [message, setMessage] = useState("");

    async function handleSend() {
        const content = message.trim();

        if (!content) {
            return;
        }

        try {
            await sendMessage(peerId, content);
            useDataChannelStore.getState().addOutgoingMessage(peerId, content);
            setMessage("");
        } catch (error) {
            console.error("Failed to send message: ", error);
        }
    }

    return (
        <div className="flex h-10 w-full items-center rounded-sm border border-white/20 p-2 shadow-sm backdrop-blur-2xl">
            <input
                className="ml-2 flex-1 bg-transparent text-white outline-none"
                type="text"
                value={message}
                onChange={(e) => setMessage(e.target.value)}
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