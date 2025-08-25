---
type: 'always_apply'
---

# Front-End Style Development Specifications

- **Component Priority**: Refer to [COMPONENT_LIBRARY_REFERENCE.md](mdc:COMPONENT_LIBRARY_REFERENCE.md) to select existing components and variants
- **Strictly Prohibit Reinventing the Wheel**: Do not hand-write implementations of existing components
- **Proactively Evaluate Component Suitability**: If existing components do not fit the current needs, proactively propose and explain your alternative solutions
- **Style Specifications**: [index.css](mdc:src/index.css) theme variables take precedence over Tailwind standard classes, use hard-coded color values cautiously
- **Structure Reuse**: Refer to the page structure of the most similar existing pages
- **Layout Components**: Use PageContainer + PageHeader + BlockLayout, refer to [LAYOUT_SYSTEM.md](mdc:LAYOUT_SYSTEM.md)
- **Split Files**: For complex pages, do not cram everything into one file, split files reasonably based on responsibilities.

## Layered Architecture for Complex Components

When creating complex components or refactoring existing ones, use **3-Layer Separation**:

### File Structure Pattern

```
ComponentName/
├── ComponentName.tsx    # UI Layer - Pure presentation
├── dataService.ts       # Data Service Layer - API abstraction
├── mockData.ts         # Data Layer - Mock data & constants
├── types.ts            # Type definitions
├── index.ts            # Clean exports
└── README.md           # Architecture documentation
```

### Layer Responsibilities

- **UI Layer**: Only handles rendering, user interactions, and component state
- **Data Service Layer**: Handles all data operations, API calls, business logic
- **Data Layer**: Contains mock data, constants, and data transformations

### Implementation Rules

- **UI Component**: Import only `DataService` methods, never direct data
- **Data Service**: Provide async methods with consistent interfaces
- **Mock Data**: Structure data exactly as real APIs would return
- **Types**: Define complete TypeScript interfaces for all data structures

### Benefits

- **Easy API Integration**: Replace mock data without touching UI
- **Clean Testing**: Test each layer independently
- **Reusable Logic**: Data service can be shared across components
- **Type Safety**: Full TypeScript coverage across all layers

### When to Apply

- Components with complex data requirements
- Pages that will integrate with APIs later
- Components with multiple data sources
- Reusable business logic across components

## Development Process

- **Development Planning**: Do not start development directly, first plan which components to use and how to layout; if there is missing information, ask the user to supplement it first
- **Code Review**: After development is completed, review which areas do not comply with the specifications
