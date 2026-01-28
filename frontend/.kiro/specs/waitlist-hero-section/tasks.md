# Implementation Plan: Waitlist Hero Section

## Overview

This implementation plan breaks down the waitlist hero section into discrete, incremental coding tasks. Each task builds on previous work, with testing integrated throughout to catch issues early. The component will be built using Next.js 16, React 19, TypeScript, and Tailwind CSS, following pixel-perfect Figma specifications.

## Tasks

- [x] 1. Set up component structure and TypeScript types
  - Create `src/app/landing/components/WaitlistHeroSection.tsx`
  - Define TypeScript interfaces for props, state, and data models
  - Set up basic component skeleton with exports
  - _Requirements: 7.1, 7.2, 8.4_

- [ ] 2. Implement starfield background generation
  - [ ] 2.1 Create star generation logic
    - Write `generateStars()` function that creates 100+ star objects
    - Each star should have random x, y (0-100%), and rotation (0-360°)
    - Use `useMemo` to generate stars once on mount
    - _Requirements: 9.1, 9.2, 9.3_
  - [ ]\* 2.2 Write property test for star position bounds
    - **Property 6: Star Position Bounds**
    - **Validates: Requirements 9.2**
  - [ ]\* 2.3 Write property test for star rotation bounds
    - **Property 7: Star Rotation Bounds**
    - **Validates: Requirements 9.3**
  - [ ] 2.4 Render starfield with absolute positioning
    - Map over stars array to render div elements
    - Apply absolute positioning, transforms for rotation
    - Use z-index to position behind content
    - Style: 4px x 4px, color #4B4B4B
    - _Requirements: 1.2, 9.4, 9.5, 9.6_

- [ ] 3. Implement gradient background and container layout
  - Apply linear gradient background (180deg, #030407 to #111827)
  - Set container max-width 1440px, height 815px
  - Create flex column layout for nav, content, and footer
  - _Requirements: 1.1, 3.8, 3.9_

- [ ] 4. Build top navigation component
  - [ ] 4.1 Create navigation structure
    - Render nav element with backdrop blur effect
    - Add Paymesh logo (import from `/public/paymeshLogo.svg`)
    - Add logo text "Paymesh" with proper styling
    - Apply padding: 20px vertical, 100px horizontal (desktop)
    - Height: 63px
    - _Requirements: 10.1, 10.2, 3.1, 3.10, 2.5_
  - [ ]\* 4.2 Write unit tests for top navigation
    - Test logo renders correctly
    - Test backdrop blur is applied
    - Test responsive padding adjustments
    - _Requirements: 10.1, 10.2, 5.6_

- [ ] 5. Build hero content section
  - [ ] 5.1 Create heading with gradient text
    - Render h1 with text "Be the first to hear from us!"
    - Apply gradient from #DFDFE0 to #282B31
    - Font size: 34px, weight: 500
    - _Requirements: 1.3, 2.2_
  - [ ] 5.2 Add social icons
    - Import X logo from `/public/XLogo.svg`
    - Import Github logo from `/public/GithubLogo.svg`
    - Render icons with 12px gap between them
    - Make icons clickable links
    - _Requirements: 3.7, 7.4_
  - [ ] 5.3 Add large Paymesh logo
    - Import and render large Paymesh logo
    - Font size: 48.6px, weight: 700
    - _Requirements: 2.6_
  - [ ] 5.4 Apply hero content layout and spacing
    - Padding: 60px vertical, 100px horizontal (desktop)
    - Gap: 80px between major sections
    - Gap: 20px between text elements
    - _Requirements: 3.2, 3.5, 3.6_

- [ ] 6. Implement email form with validation
  - [ ] 6.1 Create form state management
    - Set up useState for email, isSubmitting, status, errorMessage
    - Create FormState interface
    - _Requirements: 8.3_
  - [ ] 6.2 Implement email validation function
    - Write `validateEmail()` function with regex pattern
    - Pattern: `/^[^\s@]+@[^\s@]+\.[^\s@]+$/`
    - _Requirements: 4.1_
  - [ ]\* 6.3 Write property test for email validation
    - **Property 1: Email Validation Correctness**
    - **Validates: Requirements 4.1**
  - [ ] 6.4 Create email input field
    - Render input with type="email"
    - Dimensions: 500px width, 60px height
    - Padding: 12px/24px
    - Background: rgba(255, 255, 255, 0.05) with backdrop blur
    - Border radius: 4px
    - Add aria-label="Email address"
    - _Requirements: 1.4, 3.3, 6.1_
  - [ ] 6.5 Create submit button
    - Render button with text "Join"
    - Dimensions: 110px width, 40px height
    - Padding: 12px 24px
    - Background: rgba(250, 250, 250, 0.1) with backdrop blur
    - Border radius: 4px
    - Font size: 14px, weight: 400
    - Add aria-label="Join waitlist"
    - _Requirements: 1.5, 3.4, 2.4, 6.2_
  - [ ] 6.6 Implement form submission handler
    - Create `handleSubmit()` function
    - Validate email on submit
    - Show error for invalid/empty email
    - Set loading state during submission
    - Call optional `onEmailSubmit` prop if provided
    - Handle success/error states
    - Clear input on success
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_
  - [ ]\* 6.7 Write property tests for form behavior
    - **Property 2: Invalid Email Submission Prevention**
    - **Property 3: Valid Email Loading State**
    - **Property 4: Successful Submission Feedback**
    - **Property 5: Failed Submission Recovery**
    - **Validates: Requirements 4.2, 4.3, 4.4, 4.5, 4.6**
  - [ ] 6.8 Add status messages with ARIA live regions
    - Render success message: "Thanks for joining! Check your email."
    - Render error messages based on error type
    - Add aria-live="polite" for screen reader announcements
    - _Requirements: 6.6_

- [ ] 7. Build bottom navigation component
  - Create nav element with three items
  - Add "Home" link with underline styling
  - Add "Launch App" button with appropriate styling
  - Add "Waitlist" link
  - Apply consistent spacing and typography (16px, weight 400)
  - _Requirements: 10.3, 10.4, 10.5, 2.3_

- [ ] 8. Implement responsive design
  - [ ] 8.1 Add mobile styles (< 640px)
    - Stack content vertically
    - Adjust padding to smaller values
    - Make email input full-width
    - Adjust font sizes proportionally
    - _Requirements: 5.1, 5.7_
  - [ ] 8.2 Add tablet styles (640px - 1024px)
    - Adjust padding to medium values
    - Maintain readable layout
    - _Requirements: 5.2, 5.6_
  - [ ] 8.3 Ensure desktop styles (> 1024px)
    - Apply all Figma specifications
    - Max-width 1440px container
    - Full padding values
    - _Requirements: 5.3_
  - [ ]\* 8.4 Write unit tests for responsive behavior
    - Test layout at mobile breakpoint (375px)
    - Test layout at tablet breakpoint (768px)
    - Test layout at desktop breakpoint (1440px)
    - _Requirements: 5.1, 5.2, 5.3_

- [ ] 9. Implement accessibility features
  - [ ] 9.1 Add keyboard navigation support
    - Ensure proper tab order
    - Add visible focus indicators
    - Support Enter key for form submission
    - _Requirements: 6.3, 6.4_
  - [ ] 9.2 Verify ARIA attributes
    - Confirm all form elements have proper labels
    - Confirm error messages use aria-live
    - Confirm navigation links have accessible names
    - _Requirements: 6.1, 6.2, 6.6, 6.7_
  - [ ]\* 9.3 Write accessibility tests
    - Test color contrast ratios (4.5:1 minimum)
    - Test keyboard navigation flow
    - Test screen reader announcements
    - Run jest-axe or vitest-axe
    - _Requirements: 6.3, 6.4, 6.5, 6.6, 6.7_

- [ ] 10. Checkpoint - Ensure all tests pass
  - Run all unit tests and property tests
  - Verify component renders without console errors
  - Check TypeScript compilation
  - Ask the user if questions arise

- [ ] 11. Integrate component into landing page
  - Import WaitlistHeroSection in `src/app/page.tsx`
  - Add component to page layout (decide placement with user)
  - Test that component renders correctly in page context
  - Verify navigation links work properly
  - _Requirements: 7.7, 10.6_

- [ ] 12. Final polish and code review
  - [ ] 12.1 Verify code quality
    - Confirm component is under 150 lines
    - Check for meaningful variable names
    - Ensure semantic HTML is used
    - Remove any console.log statements
    - _Requirements: 8.1, 8.2, 8.5, 8.6_
  - [ ] 12.2 Visual verification
    - Compare rendered component to Figma design
    - Check all colors match specifications
    - Verify all spacing matches specifications
    - Confirm typography matches specifications
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6_
  - [ ]\* 12.3 Create visual regression snapshots
    - Take snapshots at key breakpoints
    - Document any intentional deviations from Figma
    - _Requirements: 1.1 through 3.10_

- [ ] 13. Final checkpoint - Complete implementation
  - Ensure all tests pass
  - Verify no TypeScript errors
  - Confirm accessibility compliance
  - Ask the user for final approval

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property tests validate universal correctness across all inputs
- Unit tests validate specific examples, edge cases, and visual specifications
- The component should be built incrementally with testing at each stage
- Checkpoints ensure validation before moving to next major section
