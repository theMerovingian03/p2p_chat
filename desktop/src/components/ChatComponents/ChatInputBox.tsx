export default function ChatInputBox() {
    return (
        <div
            className="flex h-10 w-full cursor-text flex-row items-center rounded-sm border border-white/20 p-2 shadow-sm backdrop-blur-2xl transition-colors duration-200 hover:bg-white/10">
            <span className="ml-2 text-white/70">Message username...</span>
        </div>
    )
}