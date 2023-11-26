# Changelog

All notable changes to this project will be documented in this file.

## [0.10.0] - 2023-11-26

### Features

- Split session as a feature, impl csrf for sessions
- Decouple auth and session

### Refactor

- Clean up feature splits, add testing and testing infra
- Change credential schema from id -> credentials_id and user_identity -> user_name
- Split out session feature from gateways
- Simplify lib exports
- Seperate function names for clients

### Testing

- Add tests to auth_client and session_client

### Add

- Match_csrf_token method to Session

### Break

- Rip out everything

### Wip

- Add csrf_token to sessions schema
- Lib structure

## [0.9.0] - 2023-11-07

### Bug Fixes

- Cfg macro errors, duplicate code, and lib exports

### Features

- Split off surreal db, major refactor on code design
- Split off surreal db, major refactor on code design

### Miscellaneous Tasks

- Update readme to reflect changes

### Add

- Auth builder to handle feature splitting

### Breaking

- Feature splitting

## [0.8.0] - 2023-11-06

### Bug Fixes

- Async trait Send and Sync

### Miscellaneous Tasks

- Version bump

### Add

- Port to database configs

### Wip

- Trying to split

## [0.7.0] - 2023-10-21

### Features

- Store the user_identity with the session, and return on validation

### Miscellaneous Tasks

- Add changelog

## [0.6.2] - 2023-10-18

### Miscellaneous Tasks

- Version bump

### Testing

- Update auth a gateway tests for surreal

### Change

- Table names for sessions and credentials repositories

## [0.6.1] - 2023-10-18

### Features

- Add example for surreal db

### Miscellaneous Tasks

- Update readme, version bump
- Update readme, version bump

## [0.6.0] - 2023-10-17

### Features

- Add examples for and test a planet_scale db

### Miscellaneous Tasks

- Bump version for rustls feat in sqlx

## [0.5.2] - 2023-10-17

### Miscellaneous Tasks

- Version bump

### Refactor

- Infrastructure directory
- Auth module

## [0.5.1] - 2023-10-15

### Miscellaneous Tasks

- Adjust lib file for proper re-exports
- Version bump

### Refactor

- Clean up unused fn
- Impl anyhow errors, improve file structure, cleanup domain and application modules

## [0.5.0] - 2023-10-14

### Features

- Update credentials
- Delete user credentials
- Password hasing with argon2

### Miscellaneous Tasks

- Readme and version updates

### Refactor

- Move update functions to a triat

## [0.4.0] - 2023-10-12

### Features

- Logout command and session deletion

### Testing

- Auth functions for surreal and mysql

## [0.3.9] - 2023-10-09

### Features

- Table sessions

### Miscellaneous Tasks

- Update Readme
- Update readme

### Refactor

- Move auth to application layer, general clean up

## [0.3.2] - 2023-10-09

### Features

- Register and login for sql and surreal db
- Auth builder pattern, registration, login, and jwt validation
- Create a DatabaseConfig struct

### Miscellaneous Tasks

- Readme
- Update Readme

### Wip

- Auth config class
- Auth config builder
- Bring commands into auth struct and start to build out library
- Comment out session configs and default to jwt

## [0.3.0] - 2023-10-07

### Bug Fixes

- Expired at was true
- Surreal db returning garbo

### Features

- Create user registration fn
- Login function
- Re-intorduce login logic and tests
- Start session
- Create user repository trait, change return types of queries to enums
- Auth controller
- Destroy session
- Handle session validation
- Token encoding and decoding
- Sql sessions
- Check for unique fields
- Move user struct and repo's over to credentials
- Mysql registration and login
- Surreal db uses credentials model

### Miscellaneous Tasks

- Add README
- Add license
- Update version

### Refactor

- Clean architecture pattern
- Remove Thing from user model
- Create a UserRecord wrapper
- Session repository and test
- Database file structure
- Session database and repo
- Remove db initialization fn
- Create repository errors
- Create gateway dir
- Made alias for session and user id
- Removed destroy_session
- Redis gateway to impl session repo
- Create RepoResult type
- Make session return type a generic result
- Return result with dyn error from user repo trait

### Testing

- User model test and script
- User creation and get
- User login, refactored test setup
- Re-organize tests
- Session entity
- Domain module
- Gateway tests
- Session commands
- Surreal user repo
- Mysql user repo
- Session repo with new return type

### Add

- Register user method mysql gateway
- QueryBuilder to the register method

### Clean

- General cleanup of session mod

### Improve

- Error handling for auth repo

### Pretty

- General cleanup
- General cleanup

### Remove

- Http

### Wip

- Data modeling for User
- Session creation
- Get session
- Http listener
- Redis sessions
- Reddis session unit testing
- Sql integration
- Recofigure the session domain, application, and repo for surreal db
- Get sql connection and basic queries working
- Use push_bind to push values into query
- User cred version
- Replace user with credentials struct
- Renamimg user traits to credentials traits

<!-- generated by git-cliff -->
