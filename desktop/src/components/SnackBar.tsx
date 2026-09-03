type SnackBarProps = {
    message: string
}

export default function SnackBar({ message }: SnackBarProps) {
    return (
        <div
            role="status"
            className="fixed bottom-5 left-1/2 z-50 -translate-x-1/2 rounded-sm bg-white/10 px-4 py-2 text-sm text-white shadow-sm backdrop-blur-2xl"
        >
            {message}
        </div>
    )
}