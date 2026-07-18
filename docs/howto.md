## Steps to install, setup and run

### Prerequisites
1. Rust ```rustc 1.96.0```
2. Cargo ```cargo 1.96.0```
3. SQLx CLI ```sqlx-cli 0.9.0```

### Tauri App
1. Install ```deno``` from ```deno.land```
2. In dir ```p2p_chat```, run ```cargo install create-tauri-app```
3. Run: ```cargo create-tauri-app```
4. Make sure to name the package ```desktop```

### Database
1. Add your PostgreSQL ```DATABASE_URL``` to ```.env```
2. Create migrations (if not created already in ```server/migrations```): 
```sqlx migrate add <migration_name>```
3. Migrate ```sqlx migrate run```

### Run indvidual packages (```server```, ```shared```, ```desktop```, etc.)

```cargo run -p <package_name>```