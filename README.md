# Workflow Dashboard

A modern, high-performance workflow visualization dashboard built with Yew (Rust) and compiled to WebAssembly. Features interactive flow diagrams with D3.js rendering and ELK graph layout algorithms.

## Features

- 🦀 **Rust + WASM**: Built with Yew framework for blazing-fast performance
- 📊 **Flow Visualization**: Interactive workflow diagrams using D3.js
- 🎯 **Smart Layout**: Automatic graph layout with ELK.js
- 🎨 **Modern UI**: Responsive design with Tailwind CSS
- 🔐 **Authentication**: Built-in login system (stub implementation)
- 📱 **Responsive**: Works on desktop and mobile devices
- 🎭 **Multiple Pages**: Dashboard, Flows, Profiles, Users, Settings

## Tech Stack

- **Frontend Framework**: [Yew](https://yew.rs/) 0.22 (Rust)
- **Build Tool**: [Trunk](https://trunkrs.dev/)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **Graph Visualization**: [D3.js](https://d3js.org/)
- **Graph Layout**: [ELK.js](https://eclipse.dev/elk/)
- **Router**: Yew Router
- **State Management**: Yew hooks and context

## Project Structure

```
workflow-dashboard/
├── src/
│   ├── main.rs              # App entry point & routing
│   ├── api.rs               # API client (stub - customize for your backend)
│   ├── components/          # Reusable UI components
│   │   ├── button.rs
│   │   ├── card.rs
│   │   ├── input.rs
│   │   ├── sidebar.rs
│   │   ├── flow_viewer.rs   # D3-based flow visualization
│   │   ├── nodes/           # Flow node components
│   │   └── profiles/        # Persona management components
│   ├── hooks/               # Custom Yew hooks
│   │   └── use_auth.rs      # Authentication hook
│   ├── layouts/             # Layout components
│   │   └── main_layout.rs
│   ├── pages/               # Page components
│   │   ├── dashboard.rs
│   │   ├── flows.rs
│   │   ├── profiles.rs
│   │   ├── users.rs
│   │   └── settings.rs
│   └── utils/               # Utility functions
│       └── elk_layout.rs    # ELK layout integration
├── js/
│   └── d3-flow-renderer.js  # D3 flow rendering logic
├── index.html               # HTML template
├── input.css                # Tailwind input file
├── tailwind.config.js       # Tailwind configuration
├── Trunk.toml               # Trunk configuration
└── Cargo.toml               # Rust dependencies
```

## Prerequisites

- **Rust** (latest stable): [Install Rust](https://rustup.rs/)
- **wasm32-unknown-unknown target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install trunk`
- **Node.js & npm**: For Tailwind CSS compilation

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/lucaoskaique/workflow-dashboard.git
cd workflow-dashboard
```

### 2. Install Node Dependencies

```bash
npm install
```

### 3. Build Tailwind CSS

```bash
npm run build:css
```

### 4. Run Development Server

```bash
trunk serve --open
```

This will:
- Compile Rust to WebAssembly
- Start a development server at `http://127.0.0.1:8080`
- Enable hot-reload on file changes

### 5. Watch Tailwind (Optional)

In a separate terminal:

```bash
npm run watch:css
```

## Building for Production

```bash
trunk build --release
```

The optimized build will be in the `dist/` directory.

## Customization

### Backend Integration

The `src/api.rs` file contains stubbed API functions. To integrate with your backend:

1. Define your data types (User, Flow, Persona, etc.)
2. Implement HTTP client calls (using `reqwest` or `gloo-net`)
3. Update the authentication logic in `hooks/use_auth.rs`
4. Replace the mock data in pages with real API calls

Example:

```rust
// src/api.rs
pub async fn login(email: &str, password: &str) -> Result<User, String> {
    // Implement your authentication logic
    // e.g., POST to /api/login
}
```

### Styling

- Edit `input.css` for custom Tailwind directives
- Modify `tailwind.config.js` for theme customization
- Component styles are in `src/components/`

### Flow Visualization

The flow viewer integrates with D3.js for rendering:

- **Flow data structure**: Defined in your data types
- **Layout**: ELK.js computes positions (`src/utils/elk_layout.rs`)
- **Rendering**: D3 renders nodes/edges (`js/d3-flow-renderer.js`)

Customize the visualization by modifying:
- Node components in `src/components/nodes/`
- D3 rendering logic in `js/d3-flow-renderer.js`
- Layout options in `src/utils/elk_layout.rs`

## Pages

- **Dashboard**: Overview with statistics
- **Flows**: View and manage workflow diagrams
- **Profiles**: Manage profiles (characters, agents, etc.)
- **Users**: User management (admin functionality)
- **Settings**: Application settings

## Authentication

The app includes a stub authentication system:

- Login page with email/password
- Session stored in browser local storage
- Protected routes (requires login)
- Logout functionality

Replace the stub implementation in `src/api.rs` and `hooks/use_auth.rs` with your actual auth system.

## Development Tips

### Hot Reload

Trunk provides hot reload for Rust changes. For CSS changes, run `npm run watch:css` in parallel.

### Debugging

Use browser dev tools:
- Console for Rust logs (via `gloo-console`)
- Network tab for API requests
- WASM debugging with source maps

### Performance

- Yew uses virtual DOM for efficient updates
- WASM provides near-native performance
- Use `trunk build --release` for production builds

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

This project demonstrates:
- Rust/WASM web development with Yew
- Integration of JavaScript libraries (D3, ELK) with Rust
- Modern UI patterns with Tailwind CSS
- WebAssembly performance benefits

Built as a showcase of Yew framework capabilities and Rust web development patterns.
