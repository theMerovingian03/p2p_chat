import ChatInputBox from "../components/ChatComponents/ChatInputBox";
import MessageList from "../components/ChatComponents/MessageList";
import ChatHeader from "../components/ChatComponents/ChatHeader";
import { useNavigate } from "react-router-dom";

export default function ChatPage() {
    const navigate = useNavigate();

    function handleBack() {
        navigate("/home");
    }

    return (
        <div className="items-center">
            <div className="flex min-h-screen justify-center border-2">
                <div className="mx-5 my-15 flex w-full flex-col gap-1">
                    <ChatHeader onBack={handleBack} username="username1" />
                    <MessageList />
                    <ChatInputBox />
                </div>
            </div>
        </div>
    )
}