# rust-state-poc
Rust State Management Proof of Concept
- Want to save state for Mission Info JSON
  - Be able to perform CRUD Operations

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


