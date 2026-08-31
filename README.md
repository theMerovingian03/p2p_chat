## p2p_chat

A hybrid Peer2Peer chat application powered by Rust and TypeScript. 

## Installation
### Prerequisites
1. Rust ```rustc 1.96.0```
2. Cargo ```cargo 1.96.0```
3. SQLx CLI ```sqlx-cli 0.9.0```
4. Deno ```2.9.2```

## Connect to central (managed) signaling server:

## Setup your own signaling server & client:
Make sure you're in ```p2p_chat/``` directory

1. Setup environment credentials in your root directory as:
    ```bash
    DATABASE_URL=<your_uri_here>
    JWT_SECRET=<your_jwt_secret>

    # Feel free to change this:
    JWT_EXPIRATION_HOURS=3 
    REFRESH_EXPIRATION_HOURS=48
    # Set according to your client configuration
    CLIENT_BASE_URL=http://127.0.0.1:1420
    WS_TOKEN_EXPIRATION_MINUTES=5
    ```

2. Setup environment credentials for the client in ```/desktop``` directory:

    ```bash
    # /p2p_chat/desktop/.env.development
    # /p2p_chat/desktop/.env.production

    VITE_API_URL = "<your_base_url>" # Use HTTP/HTTPs according to deployment environment
    VITE_WS_URL = "<your_base_url>/ws" # Use ws:// or wss:// according to deployment environment (make sure to include the /ws at the end!)
    ```

3. Run migrations using SQLx:
    ```bash
    cd server
    sqlx migrate run # DO NOT RUN IF YOU'RE CONNNECTING TO MANAGED SERVER
    ```

4. Fire up the signaling server:
    ```bash
    cargo run -p server
    ```

5. Run the desktop app
    ```bash
    cd desktop
    deno task desktop
    ```

## Usage

Once your Tauri app starts successfully, you can choose to:
1. Create an account.
2. Use a temporary guest account.

Creating a guest account does not restrict you from using any features. It only means this account will be automatically deleted after 24 hours of creation, and any data such as friends, requests, messages, etc. cannot be recovered. 

*NOTE: If you logout while using this account, you won't be able to access it! You can simply close the app and open it whenever you wish (within your account's 24 hours lifetime), instead of logging out.*

Creating a normal account means you can log in at any time. Although your chats only presist for a particular session, other resources such as sent and incoming requests, friends will be saved.

*Note: App currently does not support multi-device sign in!*

---