import type { UserSearchModel } from "../../../generated/bindings";

interface SearchResultProps {
    user: UserSearchModel;
}

export default function SearchResult({ user }: SearchResultProps) {
    function handleClick() {
        console.log("Clicked SearchResult!")
    }
    return (
        <div onClick={handleClick} className="w-full cursor-pointer border-b border-white p-2 text-white hover:bg-white/10">
            <span>{user.username}</span>
        </div>
    );
}