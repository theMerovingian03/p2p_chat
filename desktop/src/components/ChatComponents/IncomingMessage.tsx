type IncomingMessageProps = {
    message: string;
}

export default function IncomingMessage({ message }: IncomingMessageProps) {
    return (
        <div className="flex w-full justify-start">
            <div
                className="my-1 flex max-w-[80%] flex-col rounded-sm border-white/20 bg-blue-300/10 p-2 shadow-sm backdrop-blur-2xl">
                <span className="text-white">{message}</span>
            </div>
        </div>
    )
}