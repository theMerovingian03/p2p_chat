import { type KeyboardEvent, useEffect } from "react";
import { useSearchStore } from "../../../stores/userSearchStore";

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
        <input
            className="flex cursor-text rounded-2xl border border-white p-2 text-sm text-white/40 transition-colors duration-200 hover:bg-white/10 focus:outline-none focus:ring-0"
            placeholder="Enter username/email.."
            type="search"
            name="query"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            onKeyDown={handleKeyDown}
        />
    );
}