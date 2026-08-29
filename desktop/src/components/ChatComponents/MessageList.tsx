import IncomingMessage from "./IncomingMessage"
import OutgoingMessage from "./OutgoingMessage"

export default function MessageList() {
    return (
        <div className="h-full w-full rounded-sm">
            <IncomingMessage message="Hey, how's it going?" />
            <OutgoingMessage message="Pretty good, working on this chat UI" />
        </div>
    )
}