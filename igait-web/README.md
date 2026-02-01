# iGait ASD - Frontend

## Prerequisites
- Bun: Ensure Bun is installed on your machine. [Download here](https://bun.sh)
- Docker Account: This project's frontend and backend are containerized using Docker. Setup [here](https://www.docker.com/products/docker-desktop/)

## Setup Guide (Local Development)
1. Clone the repository:
   ```bash
   git clone https://github.com/igait-niu/igait-frontend.git
   cd igait-frontend
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Start the development server:
   ```bash
   bun dev
   ```

The app will now be running locally, usually at [http://localhost:4173](http://localhost:4173).

---

## 🎯 New Features

### Enhanced API Helper (`src/utils/apiHelper.ts`)
- Full CORS support detection
- Automatic retry logic with exponential backoff
- File validation (size, type, extensions)
- Progress tracking for uploads
- Comprehensive error messages

### Design System (`src/styles/design-system.css`)
- CSS custom properties for theming
- Reusable component classes
- Animation utilities
- Responsive breakpoints
- Accessibility-ready focus states

### Component Updates
- **JobSubmission**: Modern form with progress bar and better UX
- **DataSubmission**: Enhanced tutorial cards and consent flow
- **Toast**: Beautiful notifications with icons and animations
- **Navbar**: Fixed position with glassmorphism effect
- **Landing Page**: Hero-driven design with better content hierarchy

---

## How to Update Frontend Container (Production)
1. Login to AWS
2. Navigate to our instance in EC2
3. `cd igait-backend`
4. `docker pull ghcr.io/igait-niu/igait-web:latest`
5. `docker compose down`
6. `docker compose up -d`

Changes will now be visible at [igaitapp.com](http://igaitapp.com)

---

## 🎨 Design Tokens

### Colors
- **Primary**: `#2563eb` (Modern Blue)
- **Primary Hover**: `#1d4ed8`
- **Success**: `#10b981`
- **Warning**: `#f59e0b`
- **Error**: `#ef4444`

### Spacing Scale
- `xs`: 0.25rem (4px)
- `sm`: 0.5rem (8px)
- `md`: 1rem (16px)
- `lg`: 1.5rem (24px)
- `xl`: 2rem (32px)
- `2xl`: 3rem (48px)
- `3xl`: 4rem (64px)

### Typography
- Font Family: System UI stack for native feel
- Base Size: 1rem (16px)
- Line Heights: 1.25 (tight), 1.5 (normal), 1.75 (relaxed)

---

### Development Team
* [John W](https://github.com/hiibolt) - Head Backend and Systems Engineer
* [Michael S](https://github.com/michaelslice) - Head Frontend Engineer

---

## 📝 Future Improvements
- [ ] Dark mode support
- [ ] Accessibility audit (WCAG AA compliance) - Planned for next commit
- [ ] Internationalization (i18n)
- [ ] Performance monitoring
- [ ] E2E testing suite
