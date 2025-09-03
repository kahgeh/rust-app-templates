# CSS Guidance

## Spacing and Sizing

### Always Use Fluid Sizes for Responsive Spacing
When applying padding, margins, gaps, or any spacing that should adapt to viewport size, **always use `--size-fluid-*` variables** from Open Props instead of fixed `--size-*` variables or media queries.

#### Why Fluid Sizes?
- **No breakpoints needed**: Smoothly scales with viewport width
- **Better UX**: Eliminates jarring jumps at breakpoint boundaries  
- **Cleaner code**: One declaration instead of multiple media queries
- **Accessibility**: Respects user zoom and font size preferences

#### Example
```css
/* ❌ Avoid: Multiple breakpoints */
.container {
  padding: var(--size-2);
}
@media (min-width: 768px) {
  .container {
    padding: var(--size-3);
  }
}
@media (min-width: 1024px) {
  .container {
    padding: var(--size-4);
  }
}

/* ✅ Preferred: Fluid sizing */
.container {
  padding: var(--size-fluid-2);
}
```

#### How Fluid Sizes Work
Open Props fluid sizes use CSS `clamp()` function:
- `--size-fluid-1`: `clamp(0.5rem, 1vw, 1rem)` - Small spacing
- `--size-fluid-2`: `clamp(1rem, 2vw, 1.5rem)` - Medium spacing
- `--size-fluid-3`: `clamp(1.5rem, 3vw, 2rem)` - Large spacing
- `--size-fluid-4`: `clamp(2rem, 4vw, 3rem)` - Extra large spacing

The value smoothly scales between the minimum (mobile) and maximum (desktop) based on viewport width.

### When to Use Fixed Sizes
Use fixed `--size-*` variables only for:
- Elements that should maintain consistent size across viewports
- Minimum touch target sizes
- Fixed UI components like icons or badges
- When you need precise, non-scaling measurements

## Units: rem vs px

### Use `rem` for:
- **Spacing**: margins, padding, gaps (scales with user font preferences)
- **Typography**: font sizes, line heights
- **Layout dimensions**: widths, heights that should scale

### Use `px` for:
- **Borders**: typically 1px regardless of zoom
- **Fine details**: shadows, decorative elements
- **Fixed elements**: icons that shouldn't scale with text
- **Breakpoints**: viewport-based media query boundaries

## Theme Variables

### Color Scheme
Open Props provides semantic color variables that automatically adapt to light/dark themes:
- `--text-1`: Primary text color
- `--text-2`: Secondary/muted text color
- `--surface-1` through `--surface-4`: Background layers
- `--gray-0` through `--gray-12`: Grayscale spectrum

### Dark Mode
Instead of media queries for dark mode, use:
- CSS `color-scheme` property
- Data attributes for explicit theme selection
- Server-side theme detection via Datastar signals