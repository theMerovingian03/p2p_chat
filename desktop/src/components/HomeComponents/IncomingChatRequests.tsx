import { useWebsocketStore } from "../../stores/webSocketStore";
import IncomingChatRequestBox from "./IncomingChatRequestBox";

export default function IncomingChatRequests() {
    const incomingChatRequests = useWebsocketStore((state) => state.incomingChatRequests);
    const removeIncomingChatRequest = useWebsocketStore((state) => state.removeIncomingChatRequest);

    return (
        <div>
            <div className="w-full border border-white/20 p-2">
                <span className="text-white">Incoming Chat Requests</span>
            </div>
            <div className="no-scrollbar max-h-40 w-full flex-col overflow-y-auto rounded-b-sm border border-white/20 scroll-smooth">
                {incomingChatRequests.length === 0 ? (
                    <div className="p-4 text-sm text-white/70">
                        You have no incoming chat requests.
                    </div>
                ) : (
                    incomingChatRequests.map((request) => (
                        <IncomingChatRequestBox
                            key={request.id}
                            id={request.id}
                            from={request.from}
                            username={request.username}
                            createdAt={request.createdAt}
                            onAction={() => removeIncomingChatRequest(request.id)}
                        />
                    ))
                )}
            </div>
        </div>
    );
}
