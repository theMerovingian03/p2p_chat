## Steps to install, setup and run (ONLY FOR DEV REFERENCE)

### Prerequisites
1. Rust ```rustc 1.96.0```
2. Cargo ```cargo 1.96.0```
3. SQLx CLI ```sqlx-cli 0.9.0```

### Tauri App
1. Install ```deno``` from ```deno.land```
2. In dir ```p2p_chat```, run ```cargo install create-tauri-app```
3. Run: ```cargo create-tauri-app```
4. Make sure to name the package ```desktop```
5. Install Tailwind: ```deno add npm:tailwindcss npm:@tailwindcss/vite```
6. Add ```VITE_API_URL``` to ```desktop/.env```
7. Run as webapp: ```deno run dev``` in ```desktop/``` directory

### Database
1. Add your PostgreSQL ```DATABASE_URL``` to ```.env```
2. Create migrations (if not created already in ```server/migrations```): 
```sqlx migrate add <migration_name>```
3. Migrate ```sqlx migrate run```

### Run indvidual packages (```server```, ```shared```, ```desktop```, etc.)

* ```cargo run -p <package_name>```

### Create shared types from for the frontend

* ```deno task gen:types```

OR 

* ```deno task dev```

Edit ```desktop/src/deno.json``` to implement additional tasks.

### Server Deployment
1. Build docker image: ```docker build -t p2p-chat-server .```
2. Run: ```docker run -p 8080:8080 --env-file .env p2p-chat-server ```