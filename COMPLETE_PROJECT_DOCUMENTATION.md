# Stellar Europe - Complete Project Documentation

## Project Overview

A comprehensive platform for building and managing the Stellar blockchain community across Europe. This application enables Ambassadors and Chapter Leads to sign up, organize events, and foster the growth of the Stellar ecosystem throughout the continent.

## Features

### User Roles
- **Ambassadors**: Community representatives who organize local meetups, workshops, and educational sessions
- **Chapter Leads**: Regional coordinators who manage large-scale events and strategic initiatives

### Core Functionality
- **User Registration**: Comprehensive signup system with role selection
- **Event Management**: Create, organize, and discover community events with KPI planning
- **Community Hub**: Connect with other Stellar enthusiasts across Europe
- **Professional UI**: Modern, responsive design with Stellar-themed styling
- **KPI Tracking**: Strategic focus areas, quarterly goals, and measurable impact tracking

## Architecture

This is a full-stack Rust application built with:

### Backend
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: Role-based user system
- **API**: RESTful JSON API
- **Port**: 8080

### Frontend
- **Framework**: Yew (Rust WebAssembly framework)
- **Styling**: Custom CSS with Stellar theme and component library
- **Routing**: Client-side routing with yew-router
- **Build**: Trunk (WASM build tool)
- **Port**: 8000

### Shared
- **Data Types**: Common DTOs and types shared between frontend and backend
- **Serialization**: Serde for JSON handling

## Getting Started

### Prerequisites
- Rust (latest stable version)
- PostgreSQL database
- Trunk (for frontend builds): `cargo install trunk`

### Database Setup

#### Option 1: Docker (Recommended)

If you have Docker installed, run:

```bash
# Start PostgreSQL with Docker Compose
docker-compose up postgres -d

# The database will be available at:
# Host: localhost
# Port: 5432
# Database: stellareurope
# Username: postgres
# Password: password
```

#### Option 2: Local PostgreSQL Installation

**Windows**
1. Download PostgreSQL from https://www.postgresql.org/download/windows/
2. Install PostgreSQL with default settings
3. Set password as 'password' during installation
4. Create a database named 'stellareurope'

**macOS**
```bash
# Using Homebrew
brew install postgresql
brew services start postgresql

# Create database
createdb stellareurope
```

**Linux (Ubuntu/Debian)**
```bash
# Install PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# Set password and create database
sudo -u postgres psql
ALTER USER postgres PASSWORD 'password';
CREATE DATABASE stellareurope;
\q
```

#### Testing Database Connection

Once PostgreSQL is running, you can test the connection:

```bash
# Test connection
psql -h localhost -p 5432 -U postgres -d stellareurope

# Or using the connection string
psql postgresql://postgres:password@localhost:5432/stellareurope
```

#### Environment Configuration

Make sure your `.env` file contains:
```
DATABASE_URL=postgresql://postgres:password@localhost:5432/stellareurope
RUST_LOG=info
```

#### Running Migrations

The backend will automatically run migrations when started. You can also run them manually:

```bash
cd backend
cargo install sqlx-cli
sqlx migrate run
```

### Running the Application

#### Backend (Port 8080)
```bash
cd backend
cargo run
```

#### Frontend (Port 8000)
```bash
cd frontend
trunk serve
```

The application will be available at:
- Frontend: http://127.0.0.1:8000
- Backend API: http://127.0.0.1:8080

## Development

### Project Structure
```
stellareurope/
├── backend/           # Axum server
│   ├── src/
│   │   ├── database/  # Models, repositories, migrations
│   │   ├── handlers/  # API route handlers
│   │   └── main.rs    # Server entry point
│   └── migrations/    # SQL migration files
├── frontend/          # Yew frontend
│   ├── src/
│   │   ├── components/# UI components (refactored)
│   │   │   ├── ui/    # Basic UI primitives
│   │   │   └── forms/ # Form-specific components
│   │   ├── hooks/     # Custom hooks
│   │   ├── pages/     # Page components
│   │   ├── routing/   # Route definitions
│   │   ├── services/  # API client
│   │   └── styles/    # Design system
│   └── assets/        # Static assets
├── shared/            # Common types and DTOs
└── .env              # Environment configuration
```

### API Endpoints
- `POST /api/signup` - User registration
- `POST /api/events` - Create new event with KPI planning
- `GET /api/events` - List events with KPI data
- `GET /health` - Health check

### Design System

The application uses a professional dark theme with Stellar-inspired colors:
- **Primary**: `#00d4ff` (Stellar blue)
- **Secondary**: `#ff6b35` (Chapter Lead orange)
- **Background**: Dark gradient from `#000000` to `#1a1a1a`
- **Text**: White and light gray variants

## Frontend Architecture Refactoring

### Transformation Results

The frontend has been completely refactored from monolithic components to a modern, component-driven architecture:

| Component | Before | After | Reduction |
|-----------|--------|-------|-----------|
| **SignupPage** | 353 lines | ~100 lines | **71% smaller** |
| **EventForm** | 695 lines | ~150 lines | **78% smaller** |
| **Total Pages** | 1,048 lines | ~250 lines | **76% reduction** |

### New Architecture

```
frontend/src/
├── components/
│   ├── ui/              # 5 reusable UI primitives
│   │   ├── input.rs     # Smart input with validation
│   │   ├── button.rs    # Multi-variant button system
│   │   ├── card.rs      # Consistent card layouts
│   │   ├── textarea.rs  # Enhanced textarea
│   │   └── select.rs    # Styled select component
│   └── forms/           # 2 compound form components
│       ├── wallet_input.rs    # Freighter integration
│       └── form_section.rs   # Consistent sections
├── hooks/               # 2 custom hooks
│   ├── use_form.rs      # Generic form management
│   └── use_freighter.rs # Wallet integration
└── styles/
    └── components.css   # Design system & tokens
```

### Key Improvements Achieved

#### Code Quality & Maintainability
- 76% reduction in page component size
- Zero repetitive patterns - handled by hooks
- Single responsibility - each component has one job
- Type-safe props - compile-time validation
- Consistent error handling throughout

#### Developer Experience
- Declarative syntax - describe what, not how
- Reusable components - write once, use everywhere
- Custom hooks - encapsulated business logic
- Design system - consistent styling tokens
- Better IntelliSense - strong typing support

#### User Experience
- Consistent interactions across all forms
- Better accessibility - proper labels and ARIA
- Smooth animations - unified transition system
- Mobile responsive - built-in responsive design
- Error states - clear, helpful error messages

### Component Usage Examples

**Before: Repetitive Manual Code**
```rust
// 40+ lines of repetitive callback and state management
let username = use_state(|| String::new());
let on_username_change = {
    let username = username.clone();
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        username.set(input.value());
    })
};

html! {
    <div class="form-group">
        <label for="username">{"Username *"}</label>
        <input
            type="text"
            id="username"
            value={(*username).clone()}
            onchange={on_username_change}
            placeholder="Enter your username"
            required=true
        />
    </div>
}
```

**After: Clean Declarative Components**
```rust
// 6 lines - clean, reusable, type-safe
let form = use_form();

html! {
    <Input
        label="Username"
        value={form.get_value("username")}
        on_change={form.get_callback("username")}
        placeholder="Enter your username"
        required=true
        error={form.get_error("username")}
    />
}
```

## KPI Planning Implementation

### Implementation Overview

The backend and frontend have been successfully enhanced with comprehensive KPI planning features that align with the Stellar Ambassador Program's quarterly budget review requirements.

### Completed Features

#### Database Schema Updates
- **New Migration**: `003_add_kpi_planning_fields.sql`
- **Strategic Focus Areas**: Array field for multiple focus area selection
- **KPI Metrics**: Individual fields for all core and supporting metrics
- **Planning Fields**: Target audience, quarterly goals, strategic purpose, success metrics
- **Database Indexes**: Optimized for querying by focus areas and audience

#### Data Models Enhanced
- **EventRequest DTO**: Added all KPI planning fields
- **EventResponse DTO**: Complete KPI data in API responses
- **Event Model**: Updated database model with new fields
- **Strategic Focus Areas**: New enum with 5 standard categories
- **KPI Estimates**: Structured data type for metric tracking

#### Frontend Form Enhancements

**Comprehensive KPI Section**
- **Strategic Focus Areas**: Interactive checkbox grid with descriptions
  - Community Participation
  - On-Chain Activity
  - SCF Referrals
  - Ecosystem Collaboration
  - Developer Growth

**Primary KPI Estimates**
- **Monthly Active Ambassadors**: Number tracking
- **Monthly Active Accounts**: Wallet/account creation goals
- **SCF Referrals**: Builder pipeline tracking

**Supporting Metrics**
- **Content Produced**: Articles, videos, tutorials
- **Expected Attendance**: Event size planning
- **Social Growth Target**: Reach and engagement goals

**Strategic Planning Fields**
- **Strategic Purpose**: Required explanation of event impact
- **Target Audience**: Specific audience definition
- **Quarterly Goals**: Connection to broader strategy
- **Success Metrics**: Detailed measurement and test plan

### Guidelines Compliance

#### Chapter KPI Planning Requirements

1. **Quarterly Strategic Focus Areas**
   - Every activity connects to at least one focus area
   - Clear strategic purpose required
   - Interactive selection with explanations

2. **Primary KPIs**
   - Monthly Active Ambassadors estimation
   - Monthly Active Accounts tracking
   - SCF Referrals pipeline planning
   - Supporting metrics collection

3. **Events Timeline**
   - Date and timing information
   - Event type categorization
   - Audience targeting details
   - KPI connection explanation

4. **Minimum Requirements Met**
   - Strategic Focus Areas selection
   - Primary KPI estimates
   - Event timeline and details
   - Strategic purpose documentation

### Technical Implementation

#### Type Safety
- All KPI data is strongly typed in Rust
- Compile-time validation prevents runtime errors
- Shared types between frontend and backend ensure consistency

#### Database Design
- PostgreSQL arrays for multi-select focus areas
- Nullable integer fields for optional metrics
- Text fields for strategic planning content
- Proper indexing for performance

#### User Experience
- Interactive checkboxes with helpful descriptions
- Clear section organization and visual hierarchy
- Form validation and error handling
- Professional styling matching Stellar theme

### Form Structure

The enhanced event creation form now includes:

```
Event Details (Original)
├── Title, Description, Type
├── Date, Location
└── Registration Info

KPI Planning & Strategic Impact (NEW)
├── Strategic Focus Areas (required)
├── Strategic Purpose (required)
├── Target Audience (required)
├── Primary KPI Estimates
│   ├── Monthly Active Ambassadors
│   ├── Monthly Active Accounts
│   └── SCF Referrals
├── Supporting Metrics
│   ├── Content Produced
│   ├── Expected Attendance
│   └── Social Growth Target
├── Quarterly Goals (required)
└── Success Metrics & Test Plan
```

## Technical Implementation

### Type Safety
- Full TypeScript-like experience with Rust
- Shared types between frontend and backend
- Compile-time guarantees for API contracts

### Performance
- WebAssembly for near-native frontend performance
- Optimized database queries with SQLx
- Async/await throughout the stack
- Efficient CSS with minimal bundle size

### Security
- SQL injection prevention with prepared statements
- CORS configuration for cross-origin requests
- Input validation on both client and server
- Role-based access patterns

## Deployment

### Docker Support
The application can be containerized for easy deployment:

```bash
# Build and run with Docker Compose
docker-compose up --build
```

### Production Considerations
- Set up SSL/TLS termination
- Configure PostgreSQL with proper security
- Set up monitoring and logging
- Implement backup strategies
- Configure environment-specific settings

## Troubleshooting

### Database Issues

**Authentication Failed**
- Ensure PostgreSQL is running
- Verify the password is set correctly
- Check the connection string in `.env`

**Database Not Found**
- Create the database: `CREATE DATABASE stellareurope;`
- Verify the database name in the connection string

**Connection Refused**
- Check if PostgreSQL is running: `ps aux | grep postgres`
- Verify the port (default 5432) is not blocked
- Check firewall settings

### Port Configuration
- Backend runs on port 8080
- Frontend runs on port 8000
- Database runs on port 5432
- All components are configured to use these standard ports

## Next Steps for Production

### Database Setup
1. Run the PostgreSQL instance
2. Apply the new migration
3. Verify all fields are created correctly

### Testing
1. Create test events with KPI data
2. Verify data persistence and retrieval
3. Test form validation and submission

### Deployment
1. Update production database schema
2. Deploy enhanced backend API
3. Deploy updated frontend with new forms

## Community

Join the Stellar Europe community:
- Website: stellareurope.org
- Discord: Join our Discord
- Twitter: @StellarEurope

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Stellar Development Foundation for the amazing blockchain platform
- The Rust community for excellent tooling and libraries
- European blockchain communities for inspiration and feedback

Built with care for the Stellar community across Europe