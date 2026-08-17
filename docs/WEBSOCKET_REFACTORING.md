# WebSocket Refactoring Implementation Guide

## Overview
This document describes the refactored WebSocket architecture for the P2P Chat desktop application, where all WebSocket connection logic has been moved from React/TypeScript to the Rust Tauri backend.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    React / Zustand                              │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  • UI State Management                                   │  │
│  │  • Event Handling (via Tauri events)                     │  │
│  │  • Status Display                                        │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────┘
                           │ Tauri IPC
                           │ (invoke, listen, emit)
┌──────────────────────────▼──────────────────────────────────────┐
│                    Tauri Rust Backend                           │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  WebSocketManager                                        │  │
│  │  • Connection lifecycle                                  │  │
│  │  • ClientEvent serialization                             │  │
│  │  • ServerEvent deserialization                           │  │
│  │  • Token-based authentication                            │  │
│  │  • Error handling                                        │  │
│  │  • Tauri event emission                                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────┘
                           │ WebSocket (TCP)
                           │ ws:// or wss://
┌──────────────────────────▼──────────────────────────────────────┐
│            Axum Server (WebSocket Endpoint)                     │
└─────────────────────────────────────────────────────────────────┘
```

## Communication Flows

### 1. Connection Establishment
```
React (AuthComponent)
  │
  ├─ await getWsToken()  → Backend REST API
  │    returns { ws_token: "..." }
  │
  └─ await invoke("connect_websocket", {
       wsUrl: "ws://localhost:8000/ws",
       wsToken: "..."
     })
       │
       └─ Tauri Command Handler
            │
            └─ WebSocketManager::connect()
                 │
                 ├─ Updates status → "connecting"
                 ├─ Emits event → "ws-status-changed"
                 ├─ Initiates tokio-tungstenite connection
                 │    URL: ws://localhost:8000/ws?ws_token=...
                 │
                 └─ On Success:
                      ├─ Updates status → "connected"
                      ├─ Emits event → "ws-status-changed"
                      ├─ Spawns read task (server events → Tauri events)
                      └─ Spawns write task (Tauri channel → server)
```

### 2. Sending Events (Client → Server)
```
React Component
  │
  └─ await sendChatRequest(toUserId)
       │
       └─ await invoke("send_chat_request", { to: toUserId })
            │
            └─ Tauri Command Handler
                 │
                 └─ WebSocketManager::send_event(ClientEvent::ChatRequestSend { to })
                      │
                      ├─ Serializes event to JSON
                      ├─ Sends via WebSocket
                      └─ Returns success/error to React
```

### 3. Receiving Events (Server → React)
```
Axum Server
  │
  └─ Sends: ServerEvent::ChatRequestIncoming { from: "uuid" }
       │
       └─ tokio-tungstenite (Rust task)
            │
            └─ Deserializes to ServerEvent
                 │
                 └─ app.emit("ws-event", event)
                      │
                      └─ Tauri Event Channel
                           │
                           └─ React (via listen())
                                │
                                └─ useWebsocketStore.handleEvent()
                                     │
                                     └─ Update Zustand state
                                          │
                                          └─ Re-render UI
```

## File Structure

### Rust Layer (desktop/src-tauri/src/)

```
src-tauri/src/
├── lib.rs                           (Updated: manage WebSocketManager)
├── main.rs                          (Unchanged: delegates to lib.rs)
├── commands/
│   ├── mod.rs                       (Updated: added websocket module)
│   ├── auth.rs                      (Unchanged)
│   └── websocket.rs                 (NEW: WebSocket commands)
└── websocket/
    ├── mod.rs                       (NEW: module exports)
    └── manager.rs                   (NEW: WebSocketManager implementation)
```

### React/TypeScript Layer (desktop/src/)

```
src/
├── services/
│   └── websocketService.ts          (Updated: now delegates to Tauri)
├── stores/
│   └── webSocketStore.ts            (Updated: listens to Tauri events)
├── hooks/
│   └── useWebSocket.ts              (NEW: optional hook wrapper)
├── components/
│   ├── AuthComponent.tsx            (Updated: uses Tauri events)
│   └── HomeComponents/
│       └── ...                      (Updated: async event senders)
└── generated/
    └── bindings.ts                  (Auto-generated: unchanged)
```

## Tauri Commands API

All commands are exposed via `invoke()` from `@tauri-apps/api/core`:

### Connection Management
```typescript
// Initiate WebSocket connection
await invoke("connect_websocket", {
  wsUrl: string,      // e.g., "ws://localhost:8000/ws"
  wsToken: string     // JWT token from getWsToken()
});

// Close WebSocket connection
await invoke("disconnect_websocket");

// Get current connection status
const status: string = await invoke("get_websocket_status");
// Returns: "disconnected" | "connecting" | "connected"
```

### Chat Operations
```typescript
// Send a chat request
await invoke("send_chat_request", {
  to: string  // UUID of recipient
});

// Accept a chat request
await invoke("accept_chat_request", {
  from: string  // UUID of requester
});
```

### WebRTC Operations
```typescript
// Send WebRTC offer
await invoke("send_webrtc_offer", {
  to: string,    // UUID of peer
  sdp: string    // SDP offer string
});

// Send WebRTC answer
await invoke("send_webrtc_answer", {
  to: string,    // UUID of peer
  sdp: string    // SDP answer string
});

// Send ICE candidate
await invoke("send_ice_candidate", {
  to: string,        // UUID of peer
  candidate: string  // ICE candidate string
});
```

## Tauri Events API

Events are received via `listen()` from `@tauri-apps/api/event`:

### Status Changes
```typescript
await listen<{ status: string }>("ws-status-changed", (event) => {
  // event.payload.status: "disconnected" | "connecting" | "connected"
  console.log(`Status changed to: ${event.payload.status}`);
});
```

### Server Events
```typescript
await listen<ServerEvent>("ws-event", (event) => {
  // event.payload is a ServerEvent from the server
  // Types: ChatRequestIncoming, ChatRequestAccepted, PresenceOnline,
  //        PresenceOffline, WebRtcOffer, WebRtcAnswer, IceCandidate,
  //        GenericMessage, Error
  
  switch (event.payload.type) {
    case "ChatRequestIncoming":
      console.log(`New chat request from: ${event.payload.from}`);
      break;
    case "PresenceOnline":
      console.log(`User online: ${event.payload.id}`);
      break;
    // ... handle other event types
  }
});
```

### Error Events
```typescript
await listen<{ message: string }>("ws-error", (event) => {
  console.error(`WebSocket error: ${event.payload.message}`);
});
```

## Type System

### Shared Types (shared/src/models/websocket_models.rs)

```rust
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type")]
pub enum ServerEvent {
    ChatRequestIncoming { from: Uuid },
    ChatRequestAccepted { from: Uuid },
    PresenceOnline { id: Uuid },
    PresenceOffline { id: Uuid },
    WebRtcOffer { from: Uuid, sdp: String },
    WebRtcAnswer { from: Uuid, sdp: String },
    IceCandidate { from: Uuid, candidate: String },
    Error { code: WsErrorCode, message: String },
    GenericMessage { message: String },
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type")]
pub enum ClientEvent {
    ChatRequestSend { to: Uuid },
    ChatRequestAccept { from: Uuid },
    WebRtcOffer { to: Uuid, sdp: String },
    WebRtcAnswer { to: Uuid, sdp: String },
    IceCandidate { to: Uuid, candidate: String },
}
```

### Generated TypeScript Types (desktop/src/generated/bindings.ts)

Specta automatically generates TypeScript types from the Rust types:

```typescript
export type ServerEvent = 
  | { type: "ChatRequestIncoming"; from: string }
  | { type: "ChatRequestAccepted"; from: string }
  | { type: "PresenceOnline"; id: string }
  | { type: "PresenceOffline"; id: string }
  | { type: "WebRtcOffer"; from: string; sdp: string }
  | { type: "WebRtcAnswer"; from: string; sdp: string }
  | { type: "IceCandidate"; from: string; candidate: string }
  | { type: "Error"; code: WsErrorCode; message: string }
  | { type: "GenericMessage"; message: string };

export type ClientEvent = 
  | { type: "ChatRequestSend"; to: string }
  | { type: "ChatRequestAccept"; from: string }
  | { type: "WebRtcOffer"; to: string; sdp: string }
  | { type: "WebRtcAnswer"; to: string; sdp: string }
  | { type: "IceCandidate"; to: string; candidate: string };
```

## State Management Flow

### Zustand Store (webSocketStore.ts)

```typescript
interface WebsocketState {
  onlineUserIds: Set<string>;
  status: "connected" | "connecting" | "disconnected";
  incomingChatRequests: IncomingChatRequest[];
  
  // Methods
  handleEvent: (event: ServerEvent) => void;
  setStatus: (status: WebsocketStatus) => void;
  addIncomingChatRequest: (from: string) => void;
  removeIncomingChatRequest: (id: string) => void;
  initializeEventListeners: () => Promise<void>;  // NEW
}
```

### Initialization (AuthComponent.tsx)

```typescript
useEffect(() => {
  async function initialize() {
    // 1. Setup Tauri event listeners
    await initializeEventListeners();
    
    // 2. Get WebSocket token
    const { ws_token } = await getWsToken();
    
    // 3. Connect to WebSocket (Rust layer)
    await webSocketService.connect(ws_token, env.wsUrl);
    
    // Events from Rust automatically flow to Zustand store
  }
  
  initialize();
}, []);
```

## Error Handling

### Connection Errors
- Failed to connect: `ws-error` event emitted with message
- Store status set to "disconnected"
- React component displays error message

### Serialization Errors
- Logged to console
- Connection remains open
- Specific event may be lost, but connection continues

### Command Errors
- Returned as error from `invoke()` call
- Component catches and handles
- Can display error to user

## Usage Examples

### Example 1: Basic Connection in Component

```typescript
import { useWebsocketStore } from "../stores/webSocketStore";
import { webSocketService } from "../services/websocketService";

export function MyComponent() {
  const status = useWebsocketStore(state => state.status);
  
  useEffect(() => {
    // Initialize listeners (usually done in AuthComponent)
    useWebsocketStore.getState().initializeEventListeners();
  }, []);
  
  return (
    <div>
      WebSocket Status: <span>{status}</span>
    </div>
  );
}
```

### Example 2: Sending a Chat Request

```typescript
import { sendChatRequest } from "../services/websocketService";

export function SendRequestButton({ userId }: { userId: string }) {
  const [loading, setLoading] = useState(false);
  
  async function handleClick() {
    setLoading(true);
    try {
      await sendChatRequest(userId);  // Tauri command
      // Success - state will be updated via Tauri events
    } catch (error) {
      console.error("Failed to send request:", error);
    } finally {
      setLoading(false);
    }
  }
  
  return (
    <button onClick={handleClick} disabled={loading}>
      {loading ? "Sending..." : "Send Request"}
    </button>
  );
}
```

### Example 3: Handling Server Events

```typescript
// The store automatically handles incoming events
// But you can listen to specific events if needed:

useEffect(() => {
  listen<ServerEvent>("ws-event", (event) => {
    if (event.payload.type === "PresenceOnline") {
      console.log("User came online:", event.payload.id);
      // Update UI accordingly
    }
  });
}, []);
```

## Migration Checklist

If migrating from old implementation:

- [x] Move WebSocket connection to Rust
- [x] Implement Tauri commands for all WS operations
- [x] Emit Tauri events for status and server events
- [x] Update Zustand store to listen to Tauri events
- [x] Update websocketService to delegate to Tauri
- [x] Update components to use async/await for commands
- [x] Remove direct WebSocket management from React
- [ ] Test connection establishment
- [ ] Test sending/receiving chat requests
- [ ] Test connection failures and recovery
- [ ] Test WebRTC signaling (offer/answer/ICE)
- [ ] Performance profiling

## Debugging Tips

### Enable Rust Logging
```rust
// In websocket/manager.rs
debug!("WebSocket event: {:?}", event);
```

### Monitor Tauri Events
```typescript
import { debug } from "tauri-plugin-log-api";

listen<any>("ws-event", (event) => {
  debug(`Received ws-event: ${JSON.stringify(event)}`);
});
```

### Check Connection Status
```typescript
const status = await invoke("get_websocket_status");
console.log("Current WS status:", status);
```

### Monitor Console
- Rust logs appear in browser console when connected via Tauri dev
- Check "Console" tab in dev tools
- Look for patterns: "Websocket connected", "WebSocket error:", etc.

## Performance Considerations

1. **Connection Pooling**: Currently maintains single connection per app
2. **Message Buffering**: Tokio channel provides message buffering
3. **Async Tasks**: Read/write tasks run concurrently in Tokio runtime
4. **Event Emission**: Tauri events use efficient serialization
5. **Memory**: WebSocketManager uses Arc<Mutex> for thread-safe access

## Future Enhancements

1. **Automatic Reconnection**: Implement exponential backoff retry logic
2. **Connection Heartbeat**: Add ping/pong frames to detect stale connections
3. **Request Timeout**: Add timeout handling for commands
4. **Connection Pooling**: Support multiple connections if needed
5. **Metrics**: Add connection statistics (latency, message counts)
6. **Persistence**: Store connection state across app restarts
7. **TLS/SSL**: Support secure WebSocket (wss://)

## References

- [Tauri Commands](https://tauri.app/develop/calling-rust/)
- [Tauri Events](https://tauri.app/develop/events/)
- [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)
- [Specta TypeScript Export](https://github.com/oscartbeaumont/specta)
