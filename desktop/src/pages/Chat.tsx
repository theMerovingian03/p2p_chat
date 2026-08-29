import OutgoingMessage from "../components/ChatComponents/OutgoingMessage";
import IncomingMessage from "../components/ChatComponents/IncomingMessage";
import ChatInputBox from "../components/ChatComponents/ChatInputBox";
import ChatHeader from "../components/ChatComponents/ChatHeader";
import { useNavigate } from "react-router-dom";

export default function ChatPage() {
    const navigate = useNavigate();

    function handleBack() {
        try {
            navigate("/home");
        } catch (err) {
            console.log(err);
        }
    }

    return (
        <div className="items-center">
            <div className="flex min-h-screen justify-center border-2">
                <div className="mx-5 my-15 flex w-full flex-col gap-1">
                    <ChatHeader onBack={handleBack} />
                    <div className="h-full w-full rounded-sm">
                        <IncomingMessage />
                        <OutgoingMessage />
                    </div>
                    <ChatInputBox />
                </div>
            </div>
        </div>
    )
}