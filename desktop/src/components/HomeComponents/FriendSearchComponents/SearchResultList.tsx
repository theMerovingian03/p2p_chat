import { useSearchStore } from "../../../stores/userSearchStore";
import SearchResult from "./SearchResult";

export default function SearchResultList() {
    const results = useSearchStore((state) => state.results);

    if (results.length === 0) {
        console.log("No results!")
        return null;
    }

    return (
        <div className="no-scrollbar max-h-40 overflow-y-auto scroll-smooth rounded-sm border border-white/20 text-sm">
            {results.map((user) => (
                <SearchResult key={user.id} user={user} />
            ))}
        </div>
    );
}