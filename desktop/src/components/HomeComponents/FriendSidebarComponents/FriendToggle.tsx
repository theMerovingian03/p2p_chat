export default function FriendToggleButton() {
    return (
        <div className="flex w-full text-white">
            <button className="cursor-pointer w-50 items-center border border-b-0 rounded-tl-sm border-white/20 transition-colors duration-200 hover:bg-white/10 p-2">Friends</button>
            <button className="cursor-pointer w-50 items-center border border-b-0 border-l-0 rounded-tr-sm border-white/20 transition-colors duration-200 hover:bg-white/10 p-2">Requests</button>
        </div>
    )
}