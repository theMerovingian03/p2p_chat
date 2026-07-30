import { useSearchStore } from "../../../stores/userSearchStore";
import SearchResult from "./SearchResult";

export default function SearchResultList() {
    const results = useSearchStore((state) => state.results);
    console.log(results);

    if (results.length === 0) {
        console.log("No results!")
        return null;
    }

    return (
        <div className="no-scrollbar max-h-40 overflow-y-auto scroll-smooth rounded-2xl border border-white text-sm">
            {results.map((user) => (
                <SearchResult key={user.id} user={user} />
            ))}
        </div>
    );
}