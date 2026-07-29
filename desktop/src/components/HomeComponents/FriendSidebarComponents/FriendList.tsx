import FriendBox from "./FriendBox";

export default function FriendList() {
    return (
        <div className="no-scrollbar max-h-70 w-full flex-col overflow-y-auto rounded-b-2xl border border-white scroll-smooth">
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
            <FriendBox />
        </div>
    )
}