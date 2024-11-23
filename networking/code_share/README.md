# Code Share

<https://youtu.be/XliKk4dC-cI>

## Running the Project

To run the project, you need to specify which features to enable. The project has two main features: `frontend` and `backend`. You can run each component separately using the following commands:

### Running the Backend

The backend handles WebSocket connections and manages the state of the shared code. To run the backend, use the following command:

```bash
cargo run --no-default-features --features backend --bin backend
```

### Running the Frontend

The frontend provides the user interface for editing and sharing code. To run the frontend, use the following command:

```bash
cargo run --no-default-features --features frontend --bin frontend
```
