# Stellar Europe - Project Context

## 🎯 Project Overview

**Stellar Europe** is a full-stack Rust web application for the Stellar Europe Ambassador community, featuring:
- Event management and KPI tracking
- SCF (Stellar Community Fund) project submission system
- XF Incubator portal for project showcase and funding
- Ambassador directory and project showcase
- Freighter wallet integration for authentication

## 🏗️ Architecture

### Technology Stack
```
Frontend:  Yew (Rust WASM) + Trunk
Backend:   Axum + SQLx + PostgreSQL
Shared:    Common DTOs and types
Assets:    Static files served via Trunk
```

### Directory Structure
```
stellareurope/
├── backend/                 # Axum REST API server
│   ├── src/
│   │   ├── handlers/       # HTTP request handlers
│   │   ├── database/
│   │   │   ├── models/     # Database models
│   │   │   └── repositories/ # Data access layer
│   │   └── main.rs
│   ├── migrations/         # SQLx database migrations
│   └── .claude/           # Backend-specific context
│
├── frontend/               # Yew WebAssembly frontend
│   ├── src/
│   │   ├── components/    # Reusable UI components
│   │   ├── pages/         # Route pages
│   │   ├── routing/       # Router configuration
│   │   ├── services/      # API client
│   │   ├── styles/        # External CSS files
│   │   └── hooks/         # Custom Yew hooks
│   ├── assets/            # Static assets (copied by Trunk)
│   └── .claude/          # Frontend-specific context
│
├── shared/                 # Shared types between frontend/backend
│   ├── src/dto/           # Data Transfer Objects
│   └── .claude/          # Shared patterns context
│
└── ref/                    # Reference implementations
    ├── yew/               # Yew examples for idiomatic patterns
    └── axum/              # Axum examples for idiomatic patterns
```

## 🚀 Key Features

### 1. Event Management System
- Create events with KPI planning
- Strategic focus areas tracking
- Post-event reporting
- Event costs and evaluation
- **Routes**: `/events/new`, `/events`

### 2. SCF Project Submission
- Comprehensive project submission form
- Validation for required fields (250-word description limit)
- Team member management
- Integration status tracking
- **Route**: `/scf-submit`
- **API**: `POST /api/scf-projects`

### 3. XF Incubator Portal 🌌
- **Special**: Isolated portal with custom black background (#0a0a0a)
- Overrides global landing page background
- Features project cards, voters, mentors, "How It Works"
- Animated submit button linking to SCF form
- **Route**: `/xf-incubator`
- **CRITICAL**: Uses `.xf-incubator-portal` class for background override

### 4. Project Showcase
- Display community projects
- Filter by category (DeFi, NFTs, Payments, etc.)
- "Create a Project" button routes to SCF form
- **Route**: `/projects`

### 5. Ambassador Directory
- Community member profiles
- Search and filtering
- Ambassador statistics
- **Route**: `/ambassadors`

## 🎨 Styling Architecture

### Global Styles (index.html)
- **Default Background**: `landing_page.webp` (space/ocean theme)
- **Font**: SCHABO Condensed for hero text, Inter for body
- **Color Scheme**: Stellar blue (#00d4ff) + yellow (#FFDA00)

### Page-Specific Styles
- `navbar.css` - Transparent navbar, fixed positioning
- `pages.css` - Unified page styles (signup, about, chapters, etc.)
- `xf_incubator.css` - Black portal theme with animations
- `scf_form.css` - Glassmorphism form design
- `newpages.css` - Project showcase and ambassador directory

### XF Incubator Portal Override
```css
/* Key mechanism for portal isolation */
.xf-incubator-portal {
    position: relative;  /* Allows scrolling */
}

.xf-incubator-portal::before {
    position: fixed;     /* Background stays fixed */
    z-index: -1;         /* Behind content */
    background: #0a0a0a; /* Black with glows */
}

body:has(.xf-incubator-portal) {
    background: #0a0a0a !important;
    background-image: none !important; /* Remove global background */
}
```

## 🔄 API Endpoints

### Events
```
POST   /api/events          # Create event
GET    /api/events          # List events (with pagination)
```

### SCF Projects
```
POST   /api/scf-projects              # Create project submission
GET    /api/scf-projects              # List projects
GET    /api/scf-projects/:id          # Get project by ID
PATCH  /api/scf-projects/:id/status   # Update project status
```

### Authentication
```
POST   /api/signup          # User registration (Freighter integration)
```

### Health
```
GET    /health              # Health check
```

## 📦 Asset Serving (CRITICAL)

### Trunk Asset Directives
```html
<!-- Copy single file to dist root -->
<link data-trunk rel="copy-file" href="assets/brandlogo.png"/>

<!-- Copy directory - PRESERVES DIRECTORY NAME -->
<link data-trunk rel="copy-dir" href="assets/projects"/>
<!-- Results in: dist/projects/ (NOT dist/assets/projects/) -->
```

### Path References in Components
```rust
// ❌ WRONG - Will 404
<img src="/assets/projects/fairwage.png" />

// ✅ CORRECT - Trunk copies to /projects/
<img src="/projects/fairwage.png" />

// URL-encode spaces in filenames
<img src="/Voters/Voter%201.jpg" />
```

### Current Asset Directories
- `/projects/` - Project images (fairwage.png, moneylab.png, etc.)
- `/page_assets/` - XF Incubator assets (body.png, gear.png, etc.)
- `/Voters/` - Voter profile images
- `/mentor/` - Mentor profile images
- `/` - Root files (brandlogo.png, landing_page.webp, etc.)

## 🎯 Idiomatic Patterns

### Yew (Frontend)
✅ **DO**: Use `use_reducer` for complex state
```rust
#[function_component(MyComponent)]
fn my_component() -> Html {
    let state = use_reducer(MyState::default);
    // Single source of truth
}
```

❌ **DON'T**: Use multiple `use_state` hooks
```rust
let field1 = use_state(|| String::new());
let field2 = use_state(|| String::new());
// Hard to manage, not idiomatic
```

✅ **DO**: Extract CSS to external files
✅ **DO**: Use Props with `#[prop_or_default]`
✅ **DO**: Create reusable component modules

### Axum (Backend)
✅ **DO**: Use proper extractors
```rust
pub async fn handler(
    State(pool): State<DbPool>,
    Json(req): Json<MyRequest>,
) -> (StatusCode, Json<String>)
```

✅ **DO**: Separate concerns (handlers → repositories → models)
✅ **DO**: Use SQLx compile-time query checking
✅ **DO**: Return proper HTTP status codes

## 🗄️ Database Schema

### Key Tables
- `users` - User accounts (Freighter public keys)
- `events` - Event management with KPI tracking
- `scf_projects` - SCF project submissions

### Migrations Location
`backend/migrations/*.sql` - Run with `sqlx migrate run`

## 🔗 Routing

### Frontend Routes (Yew Router)
```rust
#[derive(Routable)]
pub enum Route {
    #[at("/")] Home,
    #[at("/signup")] Signup,
    #[at("/events/new")] EventForm,
    #[at("/events")] Events,
    #[at("/scf-submit")] SCFForm,        // ⭐ SCF submission
    #[at("/xf-incubator")] XFIncubator,  // 🌌 Portal
    #[at("/projects")] ProjectShowcase,
    #[at("/ambassadors")] AmbassadorDirectory,
    // ... more routes
}
```

### Navigation Pattern
```rust
// Using Link component
<Link<Route> to={Route::SCFForm} classes="my-button">
    {"Submit Project"}
</Link<Route>>

// Using Navigator hook
let navigator = use_navigator().unwrap();
navigator.push(&Route::SCFForm);
```

## 🐛 Common Issues & Solutions

### Issue: Assets return 404
**Cause**: Including `/assets/` in path when Trunk already copies directories
**Fix**: Remove `/assets/` prefix - use `/projects/` not `/assets/projects/`

### Issue: Enum variant with space won't compile
**Cause**: Rust doesn't allow spaces in enum variants
**Fix**: Use camelCase: `DeveloperTools` not `Developer Tools`

### Issue: XF Incubator not scrollable
**Cause**: Using `position: fixed` on portal container
**Fix**: Use `position: relative` with fixed background layer (`::before`)

### Issue: Form validation not working
**Cause**: Not reading state in validation function
**Fix**: Ensure validation function takes `&FormState` parameter

### Issue: SQLx compile error "relation does not exist"
**Cause**: Migration not run
**Fix**: `cd backend && sqlx migrate run`

## 🚦 Development Workflow

### Starting the Application
```bash
# Terminal 1 - Backend (must start first for API)
cd backend
cargo run
# Runs on http://127.0.0.1:8080

# Terminal 2 - Frontend
cd frontend
trunk serve
# Runs on http://localhost:8080
```

### Building for Production
```bash
# Backend
cd backend
cargo build --release

# Frontend
cd frontend
trunk build --release
# Output: frontend/dist/
```

### Database Operations
```bash
cd backend

# Create new migration
sqlx migrate add migration_name

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## 📚 Reference Implementations

### Yew Examples (`ref/yew/`)
- `function_todomvc` - use_reducer pattern, custom hooks
- `function_router` - Routing with yew-router
- `contexts` - Context API for shared state
- `file_upload` - File handling patterns

### Axum Examples (`ref/axum/`)
- Full Axum source code for deep pattern understanding
- Extractor patterns
- State management
- Error handling

## 🔐 Authentication

Currently uses Freighter wallet integration:
- Public key stored in database
- Future: JWT tokens, session management
- Hook: `use_freighter` in `frontend/src/hooks/`

## 🎨 Design System

### Colors
- **Primary**: #00d4ff (Stellar blue)
- **Secondary**: #FFDA00 (Stellar yellow)
- **Accent**: #ff6b35 (Orange)
- **XF Portal**: #4a9eff (Blue), #ff4444 (Red)
- **Background**: #0a0a0a (Black - XF Portal)

### Typography
- **Hero**: SCHABO Condensed (custom font)
- **Body**: Inter, Segoe UI, sans-serif
- **Code**: Plus Jakarta Sans (XF Incubator)

### Component Patterns
- Glassmorphism for modern cards
- Gradient backgrounds for CTAs
- Animated hover states
- Fixed navbar with transparency

## 📝 Documentation Files

- `IDIOMATIC_YEW_PATTERNS.md` - Comprehensive Yew patterns guide
- `ASSET_SERVING_GUIDE.md` - Trunk asset serving patterns
- `backend/.claude/claude.md` - Backend-specific context
- `frontend/.claude/claude.md` - Frontend-specific context
- `shared/.claude/claude.md` - Shared DTO patterns

## 🎯 Key Principles

1. **Idiomatic Rust**: Follow Rust best practices (see ref/ examples)
2. **Type Safety**: Leverage Rust's type system for compile-time guarantees
3. **Separation of Concerns**: Clear layers (handlers → repos → models)
4. **Component Reusability**: Create small, focused components
5. **External Styling**: No inline CSS, use dedicated .css files
6. **Progressive Enhancement**: Works without JS, enhanced with WASM
7. **Responsive Design**: Mobile-first approach with breakpoints
8. **Portal Architecture**: Isolated environments (XF Incubator)

## 🚀 Next Steps for Development

When working on this project:

1. **Check nested context**: Read relevant `.claude/claude.md` in subdirectories
2. **Follow patterns**: Refer to `IDIOMATIC_YEW_PATTERNS.md` for Yew
3. **Check references**: Use `ref/yew/` and `ref/axum/` for examples
4. **Test asset paths**: Remember Trunk's directory preservation
5. **Validate forms**: Client-side validation before API calls
6. **Update migrations**: Always create migration for schema changes
7. **External CSS**: Never inline styles, use dedicated files
8. **Component Props**: Use `#[derive(Properties, PartialEq)]`

## 📞 Project Status

✅ Event management system complete
✅ SCF project submission form complete
✅ XF Incubator portal complete (with scrolling fix)
✅ Project showcase with "Create Project" button
✅ Idiomatic Yew patterns established
✅ Idiomatic Axum backend patterns established
✅ Asset serving documented and working
🔄 Authentication system (basic, needs enhancement)
🔄 File upload for project thumbnails (planned)
🔄 Admin dashboard (planned)

---

**For detailed context on specific areas, see:**
- Backend patterns: `backend/.claude/claude.md`
- Frontend patterns: `frontend/.claude/claude.md`
- Shared DTOs: `shared/.claude/claude.md`
