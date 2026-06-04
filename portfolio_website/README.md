# Mohan Konugolu - Fullstack Rust Portfolio

A high-performance, mobile-responsive portfolio website built entirely in Rust. It utilizes **Yew** to generate a lightning-fast WebAssembly frontend, and **Axum** to serve the content.

## Code Flow & Architecture

The application is structured into a Cargo Workspace containing two distinct crates:
1. `frontend/`
2. `backend/`

### How the Code Flows
1. **User Request**: A visitor navigates to the domain (or `http://localhost:8080`).
2. **Axum Server (Backend)**: The Axum router intercepts the request. It uses `tower_http::services::ServeDir` to lookup the requested file in the `frontend/dist` folder.
3. **Serving the WASM**: The backend serves `index.html` along with the compiled `frontend.wasm` and `style.css` files.
4. **Yew Initialization**: The browser executes the WASM binary. Yew mounts the `App` component to the `div#yew-app` defined in the HTML.
5. **State-based Routing**: The user interacts with the navigation (e.g., clicks "Watch My Videos"). The Yew framework intercepts the click event, updates the internal `current_page` state, and seamlessly modifies the DOM to show the new page without communicating with the backend.

## Design Guidelines
- **CSS Modularity**: All styling is contained within `frontend/src/style.css`. We utilize CSS variables at the `:root` level to maintain a consistent warm aesthetic (Cream, Orange, Pink). Avoid hardcoding colors directly in elements.
- **Mobile First Approach**: The application uses media queries (max-width: 900px) to switch to mobile-friendly layouts, including a custom app-like floating bottom navigation bar.
- **Component Scalability**: The `main.rs` file manages the primary views. For larger scaling, consider moving the `Page::Home` and `Page::Videos` match arms into their own dedicated component files (e.g., `components/home.rs`).

## Development Instructions

### Prerequisites
- Install Rust via [rustup](https://rustup.rs/)
- Install the WASM target: `rustup target add wasm32-unknown-unknown`
- Install Trunk: `cargo install --locked trunk`

### 1. Building the Frontend
Anytime you modify `frontend/src/main.rs` or `frontend/src/style.css`, you must rebuild the WebAssembly bundle using Trunk.
```bash
cd frontend
trunk build --release
```

### 2. Running the Backend Server
The Axum server needs to be run to serve the `frontend/dist` directory.
```bash
cd backend
cargo run --release
```
Your portfolio will be live at: [http://localhost:8080](http://localhost:8080)

## Further Reading
For a detailed explanation of individual functions, state management techniques, and styling rules, please refer to the [docs.md](./docs.md) file included in this repository.
