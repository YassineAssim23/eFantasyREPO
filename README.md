# eFantasy
A full-stack fantasy esports platform built with Rust and React, featuring real-time player statistics and league management.

## Key Features
- User System:
  * JWT-based authentication with Argon2 password hashing
  * Profile management and statistics tracking
  * Custom guards for route protection

- League Management:
  * Create and manage fantasy leagues
  * Custom scoring systems
  * Draft scheduling and team management
  * Public/private league options

- Pro Player Integration:
  * MongoDB integration for pro player statistics
  * Real-time player data updates
  * Comprehensive player statistics tracking (KDA, CS, vision scores, etc.)
  * Data pipeline for importing esports statistics

## Technical Stack
Backend:
- Rust with Rocket.rs framework
- PostgreSQL with SQLx for user/league data
- MongoDB for pro player statistics
- JWT for authentication
- Custom error handling system

Data Processing:
- Python scripts for data transformation
- ETL pipeline for player statistics
- Custom data validation and normalization

## Architecture
- RESTful API with structured error handling
- Multi-database architecture (SQL + NoSQL)
- Custom guards for route protection
- Comprehensive error types and handling
- Environment-based configuration

## Development Status
Currently implementing:
- League draft system
- Real-time statistics updates
- Enhanced data validation