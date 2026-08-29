import ChatInputBox from "../components/ChatComponents/ChatInputBox";
import MessageList from "../components/ChatComponents/MessageList";
import ChatHeader from "../components/ChatComponents/ChatHeader";
import { useNavigate } from "react-router-dom";
import { useParams } from "react-router-dom";
import { useDataChannelStore } from "../stores/dataChannelStore";

export default function ChatPage() {
    const navigate = useNavigate();
    const { peerId } = useParams<{ peerId: string }>();
    const peer = useDataChannelStore(
        state => state.connectedPeers.find(
            peer => peer.peerId === peerId
        )
    );

    function handleBack() {
        navigate("/home");
    }

    if (!peerId || !peer) {
        return null;
    }

    return (
        <div className="items-center">
            <div className="flex min-h-screen justify-center border-2">
                <div className="mx-5 my-15 flex w-full flex-col gap-1">
                    <ChatHeader onBack={handleBack} username={`${peer.peerId}`} />
                    <MessageList peerId={peer.peerId} />
                    <ChatInputBox peerId={peer.peerId} />
                </div>
            </div>
        </div>
    )
}