import { type KeyboardEvent, useEffect } from "react";
import { useSearchStore } from "../../../stores/userSearchStore";
import { Search, X } from "lucide-react";

export default function SearchBar() {
    const { query, setQuery, search } = useSearchStore();

    useEffect(() => {
        const timeout = window.setTimeout(() => {
            void search(query);
        }, 300);

        return () => window.clearTimeout(timeout);
    }, [query, search]);

    const handleKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
        if (event.key === "Enter") {
            event.preventDefault();
            void search(query);
        }
    };

    return (
        <div className="relative w-full">
            <Search
                size={18}
                className="absolute left-3 top-1/2 -translate-y-1/2 text-white/40"
            />

            <input
                type="search"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Enter username/email..."
                className="w-full rounded-sm border border-white/20 bg-transparent py-2 pl-10 pr-10 text-white placeholder:text-white/40 transition-colors duration-200 hover:bg-white/10 focus:outline-none"
            />

            {query && (
                <button
                    type="button"
                    onClick={() => setQuery("")}
                    className="absolute right-3 top-1/2 -translate-y-1/2 rounded-full p-1 text-white/50 transition-colors duration-200 hover:bg-white/10 hover:text-white"
                >
                    <X size={16} strokeWidth={2.5} />
                </button>
            )}
        </div>
    );
}