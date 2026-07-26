import { Minus, Square, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

export default function TitleBar() {
    return (
        <header
            data-tauri-drag-region
            className="flex group absolute justify-end top-0 left-0 right-0 h-10 z-50"
        >
            {/* <span className="text-white m-2 opacity-0 hover:opacity-100 duration-200">P2P Chat</span> */}
            <div className="flex items-center mr-1 gap-1 opacity-0 transition-all duration-200 group-hover:opacity-100">
                <button
                    onClick={() => appWindow.minimize()}
                    className="rounded p-2 text-white transition-colors duration-200 hover:bg-white hover:text-blue-900"
                >
                    <Minus size={14} />
                </button>

                <button
                    onClick={() => appWindow.toggleMaximize()}
                    className="rounded p-2 text-white transition-colors duration-200 hover:bg-white hover:text-blue-900"
                >
                    <Square size={12} />
                </button>

                <button
                    onClick={() => appWindow.close()}
                    className="rounded p-2 text-white transition-colors duration-200 hover:bg-red-500"
                >
                    <X size={14} />
                </button>
            </div>
        </header>

    )
}