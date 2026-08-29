import { ArrowLeft } from "lucide-react"

type ChatHeaderProps = {
    username: string;
    onBack: () => void;
}

export default function ChatHeader({ username, onBack }: ChatHeaderProps) {
    return (
        <div className="flex h-10 w-full gap-1">
            <div
                onClick={onBack}
                className="flex items-center justify-center h-full w-10 cursor-pointer rounded-sm border border-white/20 transition-colors duration-200 hover:bg-white/10">
                <ArrowLeft color="white" />
            </div>
            <div
                className="flex h-full w-full flex-row items-center rounded-sm border border-white/20 backdrop-blur-2xl">
                <span className="ml-2 text-white/70">{username}</span>
            </div>
        </div>
    )
}