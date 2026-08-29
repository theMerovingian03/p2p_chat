import IncomingMessage from "./IncomingMessage"
import OutgoingMessage from "./OutgoingMessage"
import { useDataChannelStore } from "../../stores/dataChannelStore"

interface MessageListProps {
    peerId: string
}

export default function MessageList({ peerId }: MessageListProps) {

    const messages = useDataChannelStore(
        state => state.messages[peerId ?? ""] ?? []
    );

    return (
        <div className="h-full w-full rounded-sm overflow-y-auto">
            {messages.map((message) => message.outgoing ? (
                <OutgoingMessage key={message.id} message={message.content} />
            ) : (
                <IncomingMessage key={message.id} message={message.content} />
            )
            )}
        </div>
    );
}