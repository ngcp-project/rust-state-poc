# Rust State Management Proof of Concept
- Want to save state for Mission Info JSON
  - Be able to perform CRUD Operations

# Installation Instructions
1. Install Node.JS dependencies with your package manager of choice
```bash
npm install
bun install
pnpm install
```

2. Run tauri dev
```bash
npm run tauri dev
bun run tauri dev
pnpm run tauri dev
```
### Important Note: NEVER build the Rust codebase under the `src-tauri` directory using the command `cargo build`. This WILL cause problems!

3. Database Initialization

- Helpful resource: [PostGreSQL Connection issues](https://github.com/sameersbn/docker-postgresql/issues/112)


## Mission Info JSON
```json
{
    "missionName": "Search Area Time",
    "stageName": "Initialize",
    "vehicleKeys": [
        {
            "vehicleName": "ERU",
            "target": {
			    "latitude": 1.0,
			    "longitude": 2.0
		    },
            "searchArea": [
		        {
			        "latitude": 5.0,
			        "longitude": 10.0
		        },
		        {
			        "latitude": 5.0,
			        "longitude": 10.0
		        }
	        ]
        },
        {
            "vehicleName": "FRA",
            "target": {
			    "latitude": 1.5,
			    "longitude": 3.0
		    },
            "searchArea": [
		        {
			        "latitude": 1.0,
			        "longitude": 2.0
		        },
		        {
			        "latitude": 1.0,
			        "longitude": 2.0
		        }
	        ]
        }
    ]
}
```



## Libraries
- [Rust State Management with Tauri](https://v2.tauri.app/develop/state-management/)
- [Seralization and Deserization Rust Library](https://serde.rs/) 


