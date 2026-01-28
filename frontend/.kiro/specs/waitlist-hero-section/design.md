# Design Document: Waitlist Hero Section

## Overview

The Waitlist Hero Section is a standalone React component that serves as a dedicated landing page for capturing user email addresses for the Paymesh waitlist. The component features a visually striking starfield background, gradient styling, and a clean email submission form. It is designed to be pixel-perfect according to Figma specifications while maintaining responsiveness across all device sizes.

The component will be built using Next.js 16, React 19, TypeScript, and Tailwind CSS, following the existing application architecture and component patterns.

## Architecture

### Component Hierarchy

```
WaitlistHeroSection (Main Container)
├── Starfield Background Layer
├── Top Navigation
│   ├── Logo (Icon + Text)
│   └── Backdrop Blur Container
├── Hero Content
│   ├── Heading with Gradient Text
│   ├── Social Icons Container
│   │   ├── X/Twitter Icon
│   │   └── Github Icon
│   ├── Large Paymesh Logo
│   └── Email Form
│       ├── Email Input Field
│       ├── Submit Button
│       └── Status Messages (Success/Error)
└── Bottom Navigation
    ├── Home Link
    ├── Launch App Button
    └── Waitlist Link
```

### State Management

The component will use React's built-in `useState` hook for managing:

- Email input value
- Form validation state
- Submission loading state
- Success/error message state

No external state management library is required for this component.

### Styling Approach

All styling will be implemented using Tailwind CSS utility classes with custom configuration for:

- Custom gradient colors
- Backdrop blur effects
- Custom font families (GT Walsheim Trial, DM Sans)
- Responsive breakpoints

## Components and Interfaces

### WaitlistHeroSection Component

**File:** `src/app/landing/components/WaitlistHeroSection.tsx`

**Props Interface:**

```typescript
interface WaitlistHeroSectionProps {
  onEmailSubmit?: (email: string) => Promise<void>;
}
```

**Internal State:**

```typescript
interface FormState {
  email: string;
  isSubmitting: boolean;
  status: "idle" | "success" | "error";
  errorMessage: string;
}
```

### Starfield Component

The starfield will be generated as an array of star objects with random positioning:

```typescript
interface Star {
  id: number;
  x: number; // percentage (0-100)
  y: number; // percentage (0-100)
  rotation: number; // degrees (0-360)
}
```

### Email Validation

```typescript
function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}
```

## Data Models

### Form Submission Data

```typescript
interface WaitlistSubmission {
  email: string;
  timestamp: Date;
  source: "waitlist-hero";
}
```

### Component Configuration

```typescript
interface DesignTokens {
  colors: {
    background: {
      gradientStart: "#030407";
      gradientEnd: "#111827";
    };
    stars: "#4B4B4B";
    text: {
      primary: "#FFFFFF";
      secondary: "#E2E2E2";
      tertiary: "#8398AD";
    };
    heading: {
      gradientStart: "#DFDFE0";
      gradientEnd: "#282B31";
    };
    input: {
      background: "rgba(255, 255, 255, 0.05)";
    };
    button: {
      background: "rgba(250, 250, 250, 0.1)";
    };
  };
  spacing: {
    navPadding: { x: "100px"; y: "20px" };
    heroPadding: { x: "100px"; y: "60px" };
    heroGap: "80px";
    textGap: "20px";
    iconGap: "12px";
  };
  typography: {
    heading: { size: "34px"; weight: 500 };
    navText: { size: "16px"; weight: 400 };
    buttonText: { size: "14px"; weight: 400 };
    logoSmall: { size: "18px"; weight: 700 };
    logoLarge: { size: "48.6px"; weight: 700 };
  };
  dimensions: {
    container: { maxWidth: "1440px"; height: "815px" };
    nav: { height: "63px" };
    input: { width: "500px"; height: "60px" };
    button: { width: "110px"; height: "40px" };
  };
}
```

## Correctness Properties

_A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees._

### Property Reflection

After analyzing all acceptance criteria, I've identified the following testable properties. Many of the visual styling requirements (colors, fonts, spacing) are best tested through visual regression testing or manual design review rather than property-based testing, as they represent specific examples rather than universal properties. The properties below focus on behavioral correctness that should hold across all inputs.

**Redundancy Analysis:**

- Properties 4.1 and 4.2 are related but test different aspects: 4.1 tests validation logic, 4.2 tests UI response to invalid input
- Properties 4.3, 4.4, 4.5, and 4.6 test different states of the submission flow and are not redundant
- Properties 9.2 and 9.3 both test bounds but for different attributes (position vs rotation)

### Property 1: Email Validation Correctness

_For any_ string input to the email field, the validation function should return true if and only if the string matches the standard email format (contains @ symbol, has characters before and after @, and has a domain extension).

**Validates: Requirements 4.1**

### Property 2: Invalid Email Submission Prevention

_For any_ invalid email string, when the submit button is clicked, the form should prevent submission, display an error message, and not trigger the onEmailSubmit callback.

**Validates: Requirements 4.2**

### Property 3: Valid Email Loading State

_For any_ valid email string, when the submit button is clicked, the form should immediately display a loading state (disabled button, loading indicator) before the submission completes.

**Validates: Requirements 4.3**

### Property 4: Successful Submission Feedback

_For any_ valid email that successfully submits, the form should display a success message and clear the input field.

**Validates: Requirements 4.4, 4.6**

### Property 5: Failed Submission Recovery

_For any_ email submission that fails, the form should display an error message, maintain the email value in the input field, and allow the user to retry submission.

**Validates: Requirements 4.5**

### Property 6: Star Position Bounds

_For any_ generated star in the starfield background, the x and y position values should be within the range of 0 to 100 (representing percentages of viewport dimensions).

**Validates: Requirements 9.2**

### Property 7: Star Rotation Bounds

_For any_ generated star in the starfield background, the rotation value should be within the range of 0 to 360 degrees.

**Validates: Requirements 9.3**

## Error Handling

### Email Validation Errors

**Invalid Format:**

- Display: "Please enter a valid email address"
- Trigger: Email doesn't match regex pattern
- Recovery: User can edit and resubmit

**Empty Input:**

- Display: "Email address is required"
- Trigger: Submit clicked with empty field
- Recovery: User can enter email and submit

### Submission Errors

**Network Failure:**

- Display: "Unable to submit. Please check your connection and try again."
- Trigger: Network request fails
- Recovery: Retry button available, email preserved

**Server Error:**

- Display: "Something went wrong. Please try again later."
- Trigger: Server returns error response
- Recovery: Retry button available, email preserved

**Timeout:**

- Display: "Request timed out. Please try again."
- Trigger: Submission takes longer than 30 seconds
- Recovery: Retry button available, email preserved

### Graceful Degradation

**JavaScript Disabled:**

- Form should still be functional with basic HTML form submission
- Use progressive enhancement approach

**Slow Network:**

- Show loading state immediately
- Provide feedback that submission is in progress
- Timeout after 30 seconds with error message

## Testing Strategy

### Dual Testing Approach

This feature requires both unit tests and property-based tests to ensure comprehensive coverage:

**Unit Tests** focus on:

- Specific visual styling examples (colors, fonts, spacing match Figma)
- Component rendering without errors
- Accessibility attributes presence
- Integration with Next.js routing
- Edge cases (empty input, special characters in email)

**Property Tests** focus on:

- Email validation logic across all possible inputs
- Form state management across all submission scenarios
- Star generation bounds across random values
- Responsive behavior across viewport ranges

### Property-Based Testing Configuration

**Library:** We will use `@fast-check/vitest` for property-based testing in this TypeScript/React project.

**Configuration:**

- Minimum 100 iterations per property test
- Each test must reference its design document property
- Tag format: `// Feature: waitlist-hero-section, Property {number}: {property_text}`

**Example Test Structure:**

```typescript
import { test } from "vitest";
import { fc } from "@fast-check/vitest";

// Feature: waitlist-hero-section, Property 1: Email Validation Correctness
test.prop([fc.string()])("validates email format correctly", (input) => {
  const isValid = validateEmail(input);
  const hasAtSymbol = input.includes("@");
  const parts = input.split("@");
  const hasValidStructure =
    parts.length === 2 &&
    parts[0].length > 0 &&
    parts[1].includes(".") &&
    parts[1].split(".").every((p) => p.length > 0);

  expect(isValid).toBe(hasAtSymbol && hasValidStructure);
});
```

### Unit Testing Focus Areas

1. **Visual Regression:**
   - Snapshot tests for component structure
   - Style verification for key design tokens
   - Responsive layout at breakpoints (640px, 1024px, 1440px)

2. **Accessibility:**
   - ARIA labels present on form elements
   - Keyboard navigation works (Tab, Enter, Escape)
   - Focus indicators visible
   - Color contrast ratios meet WCAG 2.1 AA standards

3. **Integration:**
   - Component imports and renders in page.tsx
   - Logo assets load correctly
   - Navigation links function properly

4. **Edge Cases:**
   - Empty email submission
   - Very long email addresses
   - Special characters in email
   - Rapid repeated submissions
   - Component unmounts during submission

### Test Coverage Goals

- Unit test coverage: >90% of component code
- Property test coverage: All behavioral properties (7 properties)
- Visual regression: Key breakpoints and states
- Accessibility: WCAG 2.1 AA compliance

### Testing Tools

- **Unit Testing:** Vitest + React Testing Library
- **Property Testing:** @fast-check/vitest
- **Accessibility Testing:** jest-axe or vitest-axe
- **Visual Regression:** Chromatic or Percy (optional)

## Implementation Notes

### Performance Considerations

1. **Starfield Optimization:**
   - Generate stars once on mount, not on every render
   - Use CSS transforms for rotation (GPU-accelerated)
   - Consider using `will-change` for animated elements
   - Limit to 100-150 stars for performance

2. **Form Optimization:**
   - Debounce email validation on input change
   - Prevent multiple simultaneous submissions
   - Use React.memo if component re-renders unnecessarily

3. **Image Optimization:**
   - Use Next.js Image component for logos
   - Ensure SVG assets are optimized
   - Lazy load non-critical assets

### Responsive Breakpoints

```typescript
const breakpoints = {
  mobile: "0px", // < 640px
  tablet: "640px", // 640px - 1024px
  desktop: "1024px", // > 1024px
};
```

### Tailwind Configuration Extensions

Add to `tailwind.config.ts`:

```typescript
{
  theme: {
    extend: {
      colors: {
        'waitlist-bg-start': '#030407',
        'waitlist-bg-end': '#111827',
        'waitlist-star': '#4B4B4B',
        'waitlist-heading-start': '#DFDFE0',
        'waitlist-heading-end': '#282B31',
      },
      fontFamily: {
        'gt-walsheim': ['GT Walsheim Trial', 'sans-serif'],
        'dm-sans': ['DM Sans', 'sans-serif'],
      },
      backdropBlur: {
        'waitlist': '10px',
      },
    },
  },
}
```

### Accessibility Implementation

1. **Semantic HTML:**
   - Use `<nav>` for navigation sections
   - Use `<main>` for hero content
   - Use `<form>` for email submission
   - Use `<button>` for interactive elements

2. **ARIA Attributes:**
   - `aria-label` on email input: "Email address"
   - `aria-label` on submit button: "Join waitlist"
   - `aria-live="polite"` on status messages
   - `aria-invalid` on email input when validation fails

3. **Keyboard Navigation:**
   - Tab order: Logo → Email input → Submit button → Nav links
   - Enter key submits form
   - Escape key clears error messages
   - Focus visible on all interactive elements

4. **Screen Reader Support:**
   - Status messages announced via aria-live regions
   - Form errors associated with inputs via aria-describedby
   - Loading state announced: "Submitting email address"

### Browser Compatibility

- Modern browsers (Chrome, Firefox, Safari, Edge) - last 2 versions
- Mobile browsers (iOS Safari, Chrome Mobile) - last 2 versions
- Backdrop blur fallback for older browsers
- CSS Grid and Flexbox for layout (widely supported)

### Code Organization

```
WaitlistHeroSection.tsx (< 150 lines)
├── Imports (React, Next.js, assets)
├── Type Definitions (Props, State)
├── Helper Functions (validateEmail, generateStars)
├── Component Definition
│   ├── State Management (useState hooks)
│   ├── Event Handlers (handleSubmit, handleChange)
│   ├── Effects (useEffect for star generation)
│   └── Render (JSX)
└── Export
```

Keep the component focused and single-purpose. Extract complex logic into helper functions. Use TypeScript for type safety throughout.
