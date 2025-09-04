# Implementation Plan

Following incremental development principles: start with core business logic that delivers immediate
value, then add supporting infrastructure organically as needed.

## Iteration 1: Protect Existing Endpoint (Immediate Business Value)

### Task 1: Add Authentication to Ingress Endpoint

- [ ] 1.1 Create minimal authentication middleware
  - Create `src/middleware/auth.rs` with basic auth function
  - Implement X-API-Key header extraction and validation
  - Hardcode a test API key for validation (e.g., "test-api-key-123")
  - Return 401 Unauthorized for missing/invalid keys
  - _Focus: Protect real business functionality immediately_

- [ ] 1.2 Apply middleware to existing ingress endpoint
  - Apply auth middleware to `/api/v1/route` endpoint in main.rs
  - Create basic AuthContext struct with client_id
  - Inject mock client context for valid API keys
  - _Deliverable: /api/v1/route endpoint now requires API key authentication_

- [ ] 1.3 Test protected endpoint
  - Test with valid API key: `curl -H "X-API-Key: test-api-key-123" /api/v1/route`
  - Test without API key: should return 401 Unauthorized
  - Verify existing ingress functionality still works with valid key
  - _Requirement: Requirement 1 - API Key Authentication_

## Iteration 2: Add Database Persistence (Replace Hardcoded Data)

### Task 2: Implement 3-Layer Architecture with Real Data

- [ ] 2.1 Add minimal dependencies
  - Add only required dependencies: supabase client, sha2 for hashing
  - Avoid adding caching, uuid, or other dependencies until needed
  - _Principle: Add dependencies when features require them_

- [ ] 2.2 Create auth feature module structure
  - Create `src/features/auth/` directory
  - Create `src/features/auth/mod.rs` with basic exports
  - Create `src/features/auth/service.rs` for business logic
  - Create `src/features/auth/repository.rs` for data access
  - Create `src/features/auth/models.rs` for data structures
  - _Build 3-layer architecture as needed_

- [ ] 2.3 Create service layer
  - Move hardcoded validation logic from middleware to service
  - Implement AuthService with validate_api_key method
  - _Deliverable: Same functionality, better architecture_

- [ ] 2.4 Add database repository
  - Implement Supabase connection for API key storage
  - Create api_keys table schema in Supabase
  - Store hashed API keys (SHA-256)
  - Implement find_api_key_by_hash method
  - _Deliverable: Persistent API key storage_

- [ ] 2.5 Add data models
  - Create ApiKeyInfo struct with necessary fields
  - Create AuthContext struct for request context
  - Add only models that are actually used
  - _Requirement: Requirement 5 - Database Integration_

- [ ] 2.6 Update middleware to use service
  - Refactor middleware to use AuthService for validation
  - Maintain same API behavior
  - _Deliverable: Database-backed authentication_

## Iteration 3: Add Development Mode and Error Handling (Production Readiness)

### Task 3: Add Development Support and Error Handling

- [ ] 3.1 Add development mode bypass
  - Add AUTH_DEVELOPMENT_MODE environment variable
  - Allow bypassing auth in development
  - Inject mock context when in dev mode
  - Add clear warnings when dev mode is enabled
  - _Requirement: Requirement 4 - Development Environment Support_

- [ ] 3.2 Add error handling as needed
  - Add `src/features/auth/error.rs` with errors encountered during development
  - Implement only error types that are actually needed
  - Add proper error responses for authentication failures
  - _Principle: Add error handling when errors occur, not preemptively_
  - _Requirement: Requirement 7 - Layered Error Handling_

- [ ] 3.3 Add basic configuration management
  - Create `src/features/auth/config.rs`
  - Add environment variable support for discovered configuration needs
  - _Add only configuration that is actually needed by this point_

## Iteration 4: Production Optimizations (Performance and Monitoring)

### Task 4: Add Production Support Features

- [ ] 4.1 Add performance optimizations
  - Add moka dependency for caching
  - Implement API key validation caching
  - Add performance monitoring
  - _Add caching now that we know the performance bottlenecks_

- [ ] 4.2 Add comprehensive error handling
  - Expand error types based on real errors encountered
  - Add proper error logging and monitoring
  - Implement error recovery strategies
  - _Now we know what errors actually occur in practice_

- [ ] 4.3 Add comprehensive testing
  - Add unit tests for service layer
  - Add integration tests for middleware
  - Add repository tests with mock data
  - _Test based on actual implementation_

## Future Iterations: API Key Management and Extended Authentication

### Iteration 5: API Key Management (Future - Client Self-Service)

- [ ] 5.1 Add API key management endpoints
  - Create `src/features/auth/handler.rs` for HTTP endpoints
  - Add POST `/api/v1/auth/keys` (create API key)
  - Add GET `/api/v1/auth/keys` (list API keys)
  - Add DELETE `/api/v1/auth/keys/{id}` (revoke API key)
  - _Enable client self-service API key management_

### Iteration 6: JWT Authentication (Phase 2)

- [ ] 6.1 Add JWT support for dashboard users
  - Extend middleware to detect JWT tokens
  - Add JWT validation service
  - Integrate with existing unified middleware
  - _Requirement: Requirement 8 - Future Extensibility_

### Iteration 7: Internal Service Authentication (Phase 3)

- [ ] 7.1 Add lightweight service-to-service authentication
  - Implement internal service tokens
  - Add service context extraction
  - _Requirement: Requirement 8 - Future Extensibility_

## Development Principles Applied

### Business Value First

- **Iteration 1**: Protect real business endpoint from day 1
- **Iteration 2**: Persistent data without losing functionality
- **Iteration 3**: Production-ready features
- **Iteration 4**: Performance and monitoring optimizations

### Organic Infrastructure Growth

- **Dependencies**: Added only when features require them
- **Error Handling**: Added when errors are encountered
- **Configuration**: Added when configuration is needed
- **Caching**: Added when performance becomes an issue

### Incremental Architecture

- **Start Simple**: Hardcoded validation in middleware
- **Add Layers**: Service layer, then repository layer
- **Maintain Functionality**: Each iteration builds on working foundation
- **Test Continuously**: Every iteration is testable and demonstrable

## Success Criteria for Each Iteration

### Iteration 1 Success

- `/api/v1/route` endpoint requires valid API key
- Returns 401 for missing/invalid keys
- Existing ingress functionality works with valid key

### Iteration 2 Success

- Same API functionality with database persistence
- Clean 3-layer architecture
- API keys stored securely (hashed)

### Iteration 3 Success

- Development mode bypass functional
- Proper error handling for all scenarios
- Configuration management in place

### Iteration 4 Success

- Production-ready performance and monitoring
- Comprehensive error handling and logging
- Full caching and optimization

## Key Differences from Traditional Approach

### ❌ Traditional (Infrastructure First)

```
1. Set up all dependencies
2. Create complete error handling
3. Build configuration system
4. Design all data models
5. Implement all layers
6. Finally protect endpoints
```

### ✅ Incremental (Business Value First)

```
1. Protect real endpoint (hardcoded auth)
2. Add persistence (same API)
3. Add production features
4. Add optimizations
5. Add management features (later)
```

### Benefits of This Approach

- **Immediate Security**: Business endpoint protected from day 1
- **Immediate Value**: Authentication working immediately
- **Reduced Risk**: Each iteration is small and testable
- **Better Design**: Infrastructure grows based on actual needs
- **Faster Time to Value**: Real security delivered early
- **Focused Development**: Skip non-essential features initially
