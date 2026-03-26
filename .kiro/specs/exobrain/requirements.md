# Requirements Document

## Introduction

ExoBrain is a personal productivity application combining task management (inspired by Fizzy) with a knowledge base/notes system. The application is built with Elixir/Phoenix 1.8.3+ with LiveView, integrates Gleam for business logic modules, uses SQLite for persistence, and is deployable via Kamal to a simple Linux VPS.

## Glossary

- **ExoBrain**: The application system being developed
- **Project**: A container entity that groups related Cards and Notes together
- **Card**: A task item with status tracking and decay logic for stale items
- **Note**: A Markdown/Rich Text document associated with a Project
- **Decay**: A mechanism that tracks how long a Card has been idle, indicating staleness
- **Context**: An Elixir module that encapsulates data access logic for a domain
- **Interop**: The integration layer allowing Gleam modules to be called from Elixir code
- **Kamal**: A deployment tool for containerized applications to VPS servers

## Requirements

### Requirement 1: Project Initialization with Gleam Support

**User Story:** As a developer, I want a properly configured Phoenix project with Gleam interop, so that I can write business logic in Gleam and call it from Elixir.

#### Acceptance Criteria

1. WHEN the project is initialized THEN the ExoBrain system SHALL include Phoenix 1.8.3+ with LiveView and TailwindCSS configured
2. WHEN the project is initialized THEN the ExoBrain system SHALL include mix_gleam dependency configured for Gleam compilation
3. WHEN a Gleam module is compiled THEN the ExoBrain system SHALL make the module callable from Elixir code
4. WHEN the developer runs mix compile THEN the ExoBrain system SHALL compile both Elixir and Gleam sources without errors

### Requirement 2: User Authentication

**User Story:** As a user, I want to register and authenticate, so that I can securely access my personal projects, cards, and notes.

#### Acceptance Criteria

1. WHEN a user registers THEN the ExoBrain system SHALL create a user account with email and hashed password
2. WHEN a user logs in with valid credentials THEN the ExoBrain system SHALL establish an authenticated session
3. WHEN a user logs in with invalid credentials THEN the ExoBrain system SHALL reject the authentication attempt and display an error message
4. WHEN a user logs out THEN the ExoBrain system SHALL terminate the session and redirect to the login page
5. WHEN an unauthenticated user accesses protected resources THEN the ExoBrain system SHALL redirect to the login page

### Requirement 3: Project Management

**User Story:** As a user, I want to create and manage projects, so that I can organize my tasks and notes into logical containers.

#### Acceptance Criteria

1. WHEN a user creates a project THEN the ExoBrain system SHALL store the project with a name, description, and association to the user
2. WHEN a user lists projects THEN the ExoBrain system SHALL display only projects belonging to that user
3. WHEN a user updates a project THEN the ExoBrain system SHALL persist the changes to name and description
4. WHEN a user deletes a project THEN the ExoBrain system SHALL remove the project and all associated Cards and Notes
5. WHEN a project is created THEN the ExoBrain system SHALL record the creation timestamp

### Requirement 4: Card (Task) Management

**User Story:** As a user, I want to create and manage cards within projects, so that I can track my tasks with status and decay information.

#### Acceptance Criteria

1. WHEN a user creates a card THEN the ExoBrain system SHALL store the card with title, description, status (defaulting to "To Do"), and association to a project
2. WHEN a user updates a card status THEN the ExoBrain system SHALL persist the new status (To Do, In Progress, or Done) and record the status change timestamp
3. WHEN a user lists cards for a project THEN the ExoBrain system SHALL return only cards belonging to that project
4. WHEN a user deletes a card THEN the ExoBrain system SHALL remove the card from the database
5. WHEN a card is created THEN the ExoBrain system SHALL record the creation timestamp and initialize the last_activity_at field
6. WHEN a card status changes THEN the ExoBrain system SHALL update the last_activity_at field to the current timestamp
7. WHEN a card remains in the same status THEN the ExoBrain system SHALL preserve the last_activity_at field for decay calculation

### Requirement 5: Note Management

**User Story:** As a user, I want to create and manage notes within projects, so that I can store knowledge and documentation in Markdown format.

#### Acceptance Criteria

1. WHEN a user creates a note THEN the ExoBrain system SHALL store the note with title, content (Markdown), and association to a project
2. WHEN a user updates a note THEN the ExoBrain system SHALL persist the changes to title and content
3. WHEN a user lists notes for a project THEN the ExoBrain system SHALL return only notes belonging to that project
4. WHEN a user deletes a note THEN the ExoBrain system SHALL remove the note from the database
5. WHEN a note is created or updated THEN the ExoBrain system SHALL record the timestamp of the operation

### Requirement 6: Database Schema with SQLite

**User Story:** As a developer, I want a properly structured SQLite database with Ecto, so that data is persisted reliably with correct relationships.

#### Acceptance Criteria

1. WHEN the database is migrated THEN the ExoBrain system SHALL create tables for users, projects, cards, and notes with appropriate columns
2. WHEN a project is created THEN the ExoBrain system SHALL enforce a foreign key relationship to the users table
3. WHEN a card is created THEN the ExoBrain system SHALL enforce a foreign key relationship to the projects table
4. WHEN a note is created THEN the ExoBrain system SHALL enforce a foreign key relationship to the projects table
5. WHEN a user is deleted THEN the ExoBrain system SHALL cascade delete all associated projects, cards, and notes
6. WHEN a project is deleted THEN the ExoBrain system SHALL cascade delete all associated cards and notes

### Requirement 7: Gleam Decay Calculator Module

**User Story:** As a developer, I want the Card decay logic implemented in Gleam, so that I can demonstrate real Gleam-Elixir interop with meaningful business logic.

#### Acceptance Criteria

1. WHEN a Gleam DecayCalculator module exists THEN the ExoBrain system SHALL expose a function that calculates card health status
2. WHEN the DecayCalculator receives a last_activity_at timestamp THEN the ExoBrain system SHALL return a health status of "Fresh", "Stale", or "Rotten" based on elapsed time
3. WHEN Elixir code calls the DecayCalculator THEN the ExoBrain system SHALL correctly invoke the Gleam function and receive the health status
4. WHEN a Card is displayed THEN the ExoBrain system SHALL call the Gleam DecayCalculator to determine and show the card's health status
5. WHEN the developer reviews the module THEN the ExoBrain system SHALL provide clear documentation on the decay thresholds and interop pattern

### Requirement 8: Kamal Deployment Configuration

**User Story:** As a developer, I want a Kamal deployment configuration, so that I can deploy the application to a Linux VPS with minimal setup and persistent data.

#### Acceptance Criteria

1. WHEN the deploy.yml is configured THEN the ExoBrain system SHALL specify the Docker image build configuration
2. WHEN the deploy.yml is configured THEN the ExoBrain system SHALL specify the target server configuration for a Linux VPS
3. WHEN the deploy.yml is configured THEN the ExoBrain system SHALL include environment variable placeholders for secrets
4. WHEN the deploy.yml is configured THEN the ExoBrain system SHALL include health check configuration for the Phoenix application
5. WHEN the deploy.yml is configured THEN the ExoBrain system SHALL configure Docker volumes mapping a host directory (e.g., /var/exobrain-data) to the container storage directory for SQLite persistence

### Requirement 9: Elixir Contexts for Data Access

**User Story:** As a developer, I want well-structured Elixir Contexts with CRUD operations, so that I can interact with the database through a clean API.

#### Acceptance Criteria

1. WHEN the Projects context is used THEN the ExoBrain system SHALL provide functions for create, read, update, and delete operations on projects
2. WHEN the Cards context is used THEN the ExoBrain system SHALL provide functions for create, read, update, and delete operations on cards
3. WHEN the Notes context is used THEN the ExoBrain system SHALL provide functions for create, read, update, and delete operations on notes
4. WHEN a context function fails validation THEN the ExoBrain system SHALL return an error changeset with descriptive messages
5. WHEN a context function succeeds THEN the ExoBrain system SHALL return the created or updated entity
