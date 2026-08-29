type OutgoingMessageProps = {
    message: string;
}

export default function OutgoingMessage({ message }: OutgoingMessageProps) {
    return (
        <div className="flex w-full justify-end">
            <div className="my-1 max-w-[80%] rounded-sm bg-blue-400/10 p-2 shadow-sm backdrop-blur-2xl">
                <span className="text-white">{message}</span>
            </div>
        </div>
    )
}