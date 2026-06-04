# Documentation

This document thoroughly explains the codebase architecture, its functions, and component definitions used in the Mohan Konugolu Portfolio application.

## Overview

The application is built using a full-stack Rust approach:
1. **Frontend**: A WebAssembly (WASM) Single Page Application (SPA) written using the `Yew` framework.
2. **Backend**: An `Axum` HTTP server that serves the compiled WASM bundle and static assets securely and efficiently.

## Core Components

### 1. The Yew Frontend (`frontend/src/main.rs`)
The frontend defines the UI components and interactivity of the website. Because the application is a Single Page App, routing is managed internally through state rather than causing full page reloads.

#### State Management & Routing
```rust
enum Page {
    Home,
    Videos,
}
```
- **`Page` Enum**: Represents the two main views in the application.
- **`current_page`**: A `use_state` hook that holds the currently active `Page`. By switching this state via button clicks, Yew instantly re-renders the appropriate component tree (either the `Home` view or the `Videos` view).

#### Callbacks
- **`set_page_home` & `set_page_videos`**: These are Yew `Callback` objects attached to navigation buttons. When a user clicks a button (like the mobile floating action buttons or top navbar), the callback prevents the default link behavior and updates the `current_page` state.

#### UI Sections (Home)
- **Hero Section**: Displays the profile picture, primary tagline, and calls to action (LinkedIn, Contact, Watch Videos).
- **Core Skills**: Iterates over a static `Vec` of strings to dynamically render skill tags (e.g., PyTorch, ROS2, Isaac Sim).
- **Professional Projects**: Renders a vertical timeline representing professional experience and achievements chronologically.
- **Personal & Academic Projects**: A CSS grid section showcasing personal projects.

#### UI Sections (Videos)
- **YouTube Standard Videos**: Embeds 16:9 aspect ratio standard YouTube videos using standard `iframe` tags wrapped in a `.video-container`.
- **YouTube Shorts**: Embeds 9:16 aspect ratio YouTube shorts using standard `iframe` tags wrapped in a `.short-container` which properly constrains the height. Each short includes an informational description (`.video-info`).

### 2. Styling (`frontend/src/style.css`)
- **Variables**: Uses CSS Custom Properties for theming. The theme revolves around warm colors: Cream (`#FFF9F5`), Warm Orange (`#FF8C42`), and Light Pink (`#FF8FA3`).
- **Glassmorphism**: `.glass-card` classes utilize `backdrop-filter: blur(20px)` and rgba backgrounds to create frosted glass effects.
- **Mobile Bottom Navigation**: A fixed `.mobile-bottom-nav` element handles navigation on screens under `900px` width.
- **Animations**: Keyframe animations (e.g., `@keyframes float` and `@keyframes pulse`) drive the dynamic glowing orbs and button pulses without JavaScript overhead.

### 3. The Axum Backend (`backend/src/main.rs`)
The backend is a lightweight HTTP server serving static files.
- **`ServeDir`**: From `tower_http`, this middleware automatically serves static files from the `frontend/dist` directory built by Trunk.
- **Fallback Service**: Ensures that any unrecognized route naturally falls back to the frontend bundle.
- **Tracing**: Integrates `tracing` and `tracing-subscriber` for extensive application-level logging capabilities.
