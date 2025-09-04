# Card Flip + Center + Expand Animation Implementation Guide

## Overall Strategy

The animation system transforms example cards from a grid layout into full-screen modals through a coordinated sequence of 3D rotation, position movement, and size expansion. The key innovation is **breaking free from the CSS Grid constraints** while maintaining layout stability and creating a smooth, visually appealing transition.

### Core Technical Approach
1. **Dual-face card structure** - Each card has a front (preview) and back (modal content) face
2. **Position capture** - Use `getBoundingClientRect()` to record exact card position before animation
3. **Fixed positioning escape** - Switch from grid-relative to viewport-relative positioning
4. **Simultaneous transformations** - Combine flip, move, and scale in single animation
5. **Reverse animation** - Smoothly return to original position on close

## Phase 1: Card Flip Animation Foundation
**Commit:** `f55d679` - "feat: implement card flip animation to modal transition"

### Objective
Establish the 3D flip mechanism and dual-face card structure while maintaining grid stability.

### Key Changes

#### 1. Global State Management (base.html:16-19)
```html
data-signals-active-card-id="''"
data-signals-active-card-rect="{top: 0, left: 0, width: 0, height: 0}"
data-signals-is-flipping="false"
```
**Purpose:** Track which card is active and store its original position for animation reference.

#### 2. Card HTML Structure (examples.html:35-97)
```html
<div class="example-card-wrapper">  <!-- Maintains grid position -->
    <div class="example-card">      <!-- Escapes via position:fixed -->
        <div class="card-flipper">  <!-- 3D rotation container -->
            <div class="card-front"><!-- Original card content --></div>
            <div class="card-back"> <!-- Modal content --></div>
        </div>
    </div>
</div>
```
**Purpose:** 
- Wrapper stays in grid, preventing layout collapse
- Card can escape grid via `position: fixed`
- Flipper enables 3D rotation
- Two faces allow content switching via rotation

#### 3. Position Capture Logic (examples.html:39-51)
```javascript
data-on-click="
  const rect = evt.currentTarget.getBoundingClientRect();
  $activeCardRect = {
    top: rect.top,
    left: rect.left,
    width: rect.width,
    height: rect.height
  };
  $activeCardId = '{{ example.id }}';
  $isFlipping = true;
"
```
**Purpose:** Capture exact viewport position before breaking free from grid, preventing position jump.

#### 4. Dynamic Positioning (examples.html:52-58)
```html
data-style-position="$activeCardId === '{{ example.id }}' ? 'fixed' : ''"
data-style-top="$activeCardId === '{{ example.id }}' ? $activeCardRect.top + 'px' : ''"
data-style-left="$activeCardId === '{{ example.id }}' ? $activeCardRect.left + 'px' : ''"
```
**Purpose:** Apply captured position as inline styles, ensuring card stays visually in same place when switching to fixed positioning.

#### 5. CSS 3D Setup (examples.html:311-345)
```css
.card-flipper {
    transform-style: preserve-3d;
    transition: transform 0.6s;
}

.card-front, .card-back {
    backface-visibility: hidden;
}

.card-back {
    transform: rotateY(180deg);  /* Pre-rotated, hidden initially */
}

.example-card.flipping .card-flipper {
    transform: rotateY(180deg);  /* Flip to show back */
}
```
**Purpose:** Enable 3D flip animation with smooth 180° rotation revealing modal content.

### Phase 1 Outcome
✅ Cards flip in place without leaving grid position
✅ Modal content displays on card back face
✅ Grid layout remains stable (no jumping/collapsing)
✅ Smooth 3D rotation animation

---

## Phase 2: Center Animation with Flip Transition
**Commit:** `786b6f3` - "feat: add smooth center animation with card flip transition"

### Objective
Combine the flip animation with movement to viewport center, creating a unified motion from grid to modal.

### Key Changes

#### 1. Animation State Management (base.html:19)
```html
data-signals-closing="false"
```
**Purpose:** Track closing state separately to enable reverse animation.

#### 2. Closing Animation Logic (examples.html:72-82)
```javascript
data-on-click__stop="
  $closing = true;
  setTimeout(() => {
    $activeCardId = '';
    $isFlipping = false;
    $closing = false;
    $activeCardRect = {top: 0, left: 0, width: 0, height: 0};
  }, 600);
"
```
**Purpose:** 
- Set closing flag to trigger reverse animation
- Wait 600ms (animation duration) before resetting state
- Ensures smooth animation completion before cleanup

#### 3. Transform Reset (examples.html:56)
```html
data-style-transform="$activeCardId === '{{ example.id }}' ? 'none' : ''"
```
**Purpose:** Explicitly reset transform to prevent interference with animation keyframes.

#### 4. Slide Animation Keyframes (examples.html:377-391)
```css
@keyframes slide-to-center {
    to {
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
}

@keyframes slide-from-center {
    from {
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
    /* End position comes from inline styles */
}
```
**Purpose:**
- `slide-to-center`: Animates from captured position (inline styles) to viewport center
- `slide-from-center`: Reverses from center back to original position
- `translate(-50%, -50%)` ensures perfect centering regardless of card size

#### 5. Animation Classes (examples.html:361-375)
```css
.example-card.flipping {
    animation: slide-to-center 0.6s cubic-bezier(0.4, 0.0, 0.2, 1) forwards;
    z-index: 9999 !important;
}

.example-card.closing {
    animation: slide-from-center 0.6s cubic-bezier(0.4, 0.0, 0.2, 1) forwards;
    z-index: 9999 !important;
}

.example-card.closing .card-flipper {
    transform: rotateY(0deg);  /* Flip back to front */
}
```
**Purpose:**
- Apply movement animation with smooth easing curve
- High z-index ensures card stays above all content
- Closing class reverses both position and flip

### Phase 2 Outcome
✅ Cards smoothly animate from grid position to viewport center
✅ Flip and movement happen simultaneously (not sequential)
✅ Reverse animation follows exact same path back
✅ No visual jumping or stuttering

---

## Phase 3: Full Viewport Expansion
**Commit:** `84d0ed4` - "feat: expand modal to full viewport size during flip animation"

### Objective
Complete the transformation by expanding cards to full viewport size, creating a true full-screen modal experience.

### Key Changes

#### Modal Size in Animation (examples.html:381-382, 389-390)
```css
@keyframes slide-to-center {
    to {
        /* ... existing position properties ... */
        width: 100vw;   /* Changed from 90vw */
        height: 100vh;  /* Changed from 90vh */
    }
}

@keyframes slide-from-center {
    from {
        /* ... existing position properties ... */
        width: 100vw;   /* Ensures smooth reverse */
        height: 100vh;
    }
}
```
**Purpose:**
- Expand to full viewport dimensions (100vw × 100vh)
- Creates immersive full-screen modal experience
- Maintains smooth size transition during animation

### Phase 3 Outcome
✅ Modal expands to complete full-screen coverage
✅ No viewport edges visible in modal state
✅ Size animation synchronized with flip and movement
✅ Professional, app-like modal presentation

---

## Complete Animation Sequence

### Opening (Card → Modal)
1. **User clicks card** in grid
2. **Position captured** via `getBoundingClientRect()`
3. **Card breaks free** from grid with `position: fixed`
4. **Simultaneous animations begin** (600ms duration):
   - **Flip:** Front face rotates away (0° → 180°)
   - **Move:** Card slides from grid position to viewport center
   - **Expand:** Card grows from original size to 100vw × 100vh
5. **Modal content revealed** on back face
6. **Animation completes** with full-screen modal

### Closing (Modal → Card)
1. **User clicks close button**
2. **Closing state activated** triggering reverse animations
3. **Simultaneous reverse animations** (600ms duration):
   - **Flip:** Back face rotates away (180° → 360°/0°)
   - **Move:** Modal slides from center back to original grid position
   - **Contract:** Modal shrinks from 100vw × 100vh to original card size
4. **Card returns** to exact grid position
5. **State cleanup** after animation completes

## Key Technical Insights

### Why This Approach Works
1. **getBoundingClientRect() prevents jumping** - Captures exact viewport coordinates before position change
2. **Wrapper maintains grid stability** - Placeholder prevents layout collapse
3. **Single animation for multiple transforms** - More performant and visually cohesive
4. **Datastar's reactive signals** - Clean state management without manual DOM manipulation
5. **CSS-only animations** - Hardware accelerated, smooth performance

### Critical Implementation Details
- **evt.currentTarget** not `event.target` - Ensures correct element reference in Datastar
- **__stop modifier** not `.stop` - Correct Datastar syntax for stopPropagation
- **z-index: 9999** - Ensures modal appears above all page content
- **600ms timeout** - Matches CSS animation duration for proper cleanup
- **transform: none** reset - Prevents transform conflicts during animation

### Performance Optimizations
- Uses CSS transforms (GPU accelerated)
- Single reflow for position change
- No JavaScript animation loops
- Minimal DOM manipulation

## Usage in Your Project

To implement this animation system:

1. **Add global signals** to your base template for state management
2. **Structure cards** with wrapper, flipper, and front/back faces
3. **Capture position** on click before any state changes
4. **Apply animations** via conditional classes
5. **Handle cleanup** with proper timing

The result is a professional, smooth animation that enhances user experience while maintaining code simplicity and performance.