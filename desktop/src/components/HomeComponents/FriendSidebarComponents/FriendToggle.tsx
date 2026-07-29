export default function FriendToggleButton() {
    return (
        <div className="flex w-full text-white">
            <button className="cursor-pointer w-50 items-center border border-b-0 rounded-tl-2xl border-white transition-colors duration-200 hover:bg-white hover:text-blue-900 p-2">Friends</button>
            <button className="cursor-pointer w-50 items-center border border-b-0 border-l-0 rounded-tr-2xl border-white transition-colors duration-200 hover:bg-white hover:text-blue-900 p-2">Requests</button>
        </div>
    )
}