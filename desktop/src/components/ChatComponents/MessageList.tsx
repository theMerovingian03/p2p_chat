import IncomingMessage from "./IncomingMessage"
import OutgoingMessage from "./OutgoingMessage"
import { ChatMessage } from "../../stores/dataChannelStore"
import { useDataChannelStore } from "../../stores/dataChannelStore"


interface MessageListProps {
    peerId: string
}

const EMPTY_MESSAGES: ChatMessage[] = [];

export default function MessageList({ peerId }: MessageListProps) {

    const messages = useDataChannelStore(
        state => state.messages[peerId]
    ) ?? EMPTY_MESSAGES;

    return (
        <div className="h-120 shrink-0 scroll-smooth no-scrollbar w-full rounded-sm overflow-y-auto">
            {messages.map((message) => message.outgoing ? (
                <OutgoingMessage key={message.id} message={message.content} />
            ) : (
                <IncomingMessage key={message.id} message={message.content} />
            )
            )}
        </div>
    );
}