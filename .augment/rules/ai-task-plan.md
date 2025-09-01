---
type: 'manual'
---

## Development Process

### System Design Phase

- **Complete Upfront Design**: Follow traditional software engineering practices for system design
  - Create comprehensive requirements documents with EARS syntax
  - Design complete technical architecture with all components and interactions
  - Plan full system structure, technology stack, and integration patterns
  - Document all requirements, acceptance criteria, and technical specifications

### Development Task Planning Phase

- **Incremental Development Approach**: Use incremental principles when planning implementation
  tasks
  - Start with core business logic that delivers immediate value
  - Add supporting infrastructure (configuration, error handling, logging) as required by business
    features
  - Implement authentication and authorization when protecting actual endpoints
  - Add observability and resilience patterns when handling real traffic

- **Task Sequencing**: Plan development in business-value-first order
  - Begin with minimal working functionality that can be tested and demonstrated
  - Build features organically based on actual implementation needs
  - Add configuration items only when required by specific features
  - Implement error handling patterns as they become necessary

### Code Implementation Standards

- **Architecture Compliance**: Maintain strict engineering standards throughout development
  - Follow 3-layer architecture (Handler/Service/Repository) consistently
  - Use existing modules and crates, avoid reinventing the wheel
  - Write clean, maintainable code while prioritizing working functionality
  - Ensure proper separation of concerns and dependency injection

- **Development Planning**: Plan development in working iterations, not complete systems upfront
  - Identify the minimal working version of each feature
  - Plan how each iteration builds naturally on the previous one
  - Focus on delivering working functionality that can be tested and demonstrated

- **Code Review**: After each iteration, review both functionality and architecture compliance
  - Ensure working functionality meets user requirements
  - Verify code follows 3-layer architecture and engineering standards
  - Identify technical debt and plan refactoring in future iterations
