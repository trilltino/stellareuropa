# Stellar Europa

**Community event management platform for organizing Stellar ecosystem meetups, hackathons, and conferences**

- Event lifecycle management: creation, KPI planning, attendee tracking, post-event analytics, and SDF Community Fund project integration
- Role-based access control with JWT authentication, secure sessions, and PostgreSQL-backed user management with performance indexes
- Full-stack Rust: Axum backend with 9 database migrations, Yew frontend with component-based UI, shared DTOs for type safety
- Production-optimized WASM bundle with size-focused compilation (LTO, codegen-units=1, strip symbols) for fast page loads
- Rich media support: event photos, landing page assets, branded UI components, and responsive Discord/social integration
