import ConnectedPeerBox from "./ConnectedPeerBox";
import { Cable } from "lucide-react";

export default function ConnectedPeers() {
    return (
        <div>
            <div className="flex flex-row gap-2 p-2 text-white border-b-0 border border-white/20 rounded-t-sm items-center">
                <Cable size={16} strokeWidth={2} />
                <span>Connected Peers</span>
            </div>
            <div className="no-scrollbar max-h-70 w-full flex-col overflow-y-auto rounded-b-sm border border-white/20 scroll-smooth">
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                <ConnectedPeerBox />
                {/* <div className="p-2">

                <span className="text-white/70 text-sm">Your peers aren't connected at the moment!</span>
            </div> */}
            </div>
        </div>
    )
}