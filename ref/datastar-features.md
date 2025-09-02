# Datastar Key Features for AI Coding Agents

## Core Hypermedia Principle
Datastar follows a **backend-driven hypermedia approach** where the server is the authoritative source of truth, sending HTML patches and state updates via SSE (Server-Sent Events). The frontend provides reactive UI capabilities without heavy client-side state management.

## Application Architecture: Multi-Page Application (MPA)
Datastar applications are fundamentally **Multi-Page Applications (MPA)** with enhanced interactivity:
- Each route/page is served by the backend with full HTML
- Navigation between pages involves server requests
- State transitions are driven by server responses
- Pages are enhanced with reactive behaviors, not replaced by client-side routing
- URL changes reflect actual server-side route changes

###  STRONG AVOID: SPA Thinking is Incompatible
**DO NOT approach Datastar with Single-Page Application (SPA) patterns:**
- L No client-side routing or history management
- L No frontend state stores (Redux, MobX, Zustand, etc.)
- L No component-based architecture with virtual DOM
- L No JSON APIs for data fetching - use HTML responses
- L No separating frontend and backend into different deployments
- L No thinking in terms of "app shell" or "client-side app"

**Instead, think in terms of:**
- Server-rendered HTML pages/elements
- Progressive enhancement with reactive attributes
- Form submissions and server redirects
- HTML-over-the-wire responses
- Backend owns all routing and state management

## State Management Guidelines

### Backend State (Server Roundtrip Required)
- Business logic, validation, authorization
- Persistent data and shared state between users
- Complex computations requiring server resources
- Navigation and routing decisions
- Any state that affects what the user can do next

### Frontend-Only State (No Server Roundtrip)
- UI toggles (modals, dropdowns, tabs)
- Form input during typing (before submission)
- Mouse/keyboard tracking for UI feedback
- Debounced/throttled interactions
- Local display preferences
- Signals prefixed with `_` stay client-side

## Essential Attributes & Patterns
###  Signal-Based State Management
- Use data-signals to initialize reactive state
- Access signals with $ prefix in expressions
- Create computed values with data-computed
- Two-way binding with data-bind for forms

### DOM Reactivity Attributes
  - data-text - Update text content
  - data-show - Conditional visibility
  - data-class - Dynamic CSS classes
  - data-style - Dynamic inline styles
  - data-attr - Dynamic attributes

### Event Handling
  - data-on-[event] with modifiers (.debounce, .prevent, .stop)
  - Server actions with @get, @post, @put, @patch, @delete

### Server Communication
  - Built-in SSE support for real-time updates
  - Server can patch signals or DOM elements
  - Multiple patch modes (inner, outer, replace, append, prepend)


## Key Implementation Patterns

### Debouncing
```html
<input data-on-input__debounce.300ms="@get('/search')">
```

### Keyboard Shortcuts
```html
<div data-on-keydown__window="evt.key === 'Enter' && @submit()">
```

### Loading States
```html
<button data-indicator-loading data-attr-disabled="$loading">
  <span data-show="!$loading">Submit</span>
  <span data-show="$loading">Loading...</span>
</button>
```

### Conditional Rendering
```html
<div data-show="$isLoggedIn && $hasPermission">
  Protected Content
</div>
```

### Server Patches
Backend sends SSE events:
- `datastar-patch-elements` - HTML fragments to update DOM
- `datastar-patch-signals` - JSON data to update signals
- `datastar-execute-script` - JavaScript to execute

Use the SDK instead of drop to this level of low level detail.

