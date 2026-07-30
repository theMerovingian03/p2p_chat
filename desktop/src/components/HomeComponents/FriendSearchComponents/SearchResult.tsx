import type { UserSearchModel } from "../../../generated/bindings";

interface SearchResultProps {
    user: UserSearchModel;
}

export default function SearchResult({ user }: SearchResultProps) {
    return (
        <div className="w-full cursor-pointer border-b border-white p-2 text-white hover:bg-white/10">
            <span>{user.username}</span>
        </div>
    );
}