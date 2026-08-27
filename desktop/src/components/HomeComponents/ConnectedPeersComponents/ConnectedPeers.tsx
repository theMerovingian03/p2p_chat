import { useDataChannelStore } from "../../../stores/dataChannelStore";
import ConnectedPeerBox from "./ConnectedPeerBox";
import { Cable } from "lucide-react";

export default function ConnectedPeers() {

    const connectedPeers = useDataChannelStore(state => state.connectedPeers);

    return (
        <div>
            <div className="flex flex-row gap-2 p-2 text-white border-b-0 border border-white/20 rounded-t-sm items-center">
                <Cable size={16} strokeWidth={2} />
                <span>Connected Peers</span>
            </div>
            <div className="no-scrollbar max-h-70 w-full flex-col overflow-y-auto rounded-b-sm border border-white/20 scroll-smooth">
                {
                    connectedPeers.length === 0 ? (
                        <span className="block p-2 text-white/70">
                            Your peers aren't connected at the moment
                        </span>
                    ) : (
                        connectedPeers.map(peer => (
                            <ConnectedPeerBox
                                key={peer.peerId}
                                peerId={peer.peerId}
                            // username={peer.username}
                            />
                        ))
                    )
                }
            </div>
        </div>
    )
}