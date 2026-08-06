type FriendToggleButtonProps = {
    activeView: "friends" | "requests";
    onChange: (view: "friends" | "requests") => void;
};

export default function FriendToggleButton({ activeView, onChange }: FriendToggleButtonProps) {
    return (
        <div className="flex w-full text-white">
            <button
                type="button"
                className={`cursor-pointer w-1/2 items-center border border-b-0 rounded-tl-sm border-white/20 transition-colors duration-200 p-2 ${
                    activeView === "friends" ? "bg-white/10" : "hover:bg-white/10"
                }`}
                onClick={() => onChange("friends")}
            >
                Friends
            </button>
            <button
                type="button"
                className={`cursor-pointer w-1/2 items-center border border-b-0 border-l-0 rounded-tr-sm border-white/20 transition-colors duration-200 p-2 ${
                    activeView === "requests" ? "bg-white/10" : "hover:bg-white/10"
                }`}
                onClick={() => onChange("requests")}
            >
                Requests
            </button>
        </div>
    );
}
