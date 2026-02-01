/**
 * Scroll Animation Utilities - Overhauled for Smooth Performance
 * Uses CSS scroll-snap and IntersectionObserver for buttery smooth animations
 */

export interface ScrollAnimationOptions {
  threshold?: number;
  rootMargin?: string;
  triggerOnce?: boolean;
}

/**
 * Modern Cinematic Scroll System
 * Uses native scroll with snap points and intersection observers
 */
export function initCinematicScroll() {
  const acts = Array.from(document.querySelectorAll('[data-cinematic-section]'));

  if (acts.length === 0) {
    console.warn('⚠️ No cinematic sections found!');
    return () => {};
  }

  console.log(`🎬 Cinematic scroll initialized with ${acts.length} acts`);

  // Setup IntersectionObserver for smooth section transitions
  const sectionObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        const section = entry.target as HTMLElement;
        
        if (entry.isIntersecting) {
          // Section is visible
          section.setAttribute('data-phase', 'active');
          
          // Animate all elements in this section
          const elements = section.querySelectorAll('[data-cinematic-step]');
          elements.forEach((element) => {
            const step = parseFloat(element.getAttribute('data-cinematic-step') || '0');
            const delay = step * 300; // Convert step to milliseconds
            
            setTimeout(() => {
              element.classList.add('cinematic-active');
              element.classList.add('cinematic-complete');
            }, delay);
          });
        } else {
          // Section is leaving viewport
          section.setAttribute('data-phase', 'inactive');
          
          // Reset animations when scrolling away (optional, can be removed for persistence)
          const elements = section.querySelectorAll('[data-cinematic-step]');
          elements.forEach((element) => {
            element.classList.remove('cinematic-active', 'cinematic-complete');
          });
        }
      });
    },
    {
      threshold: 0.1, // Trigger when only 10% of section is visible (top 10%)
      rootMargin: '-80px 0px 0px 0px' // Account for navbar
    }
  );

  // Observe all sections
  acts.forEach((act) => {
    sectionObserver.observe(act);
  });

  // Initialize first section immediately
  const firstAct = acts[0];
  if (firstAct) {
    firstAct.setAttribute('data-phase', 'active');
    const elements = firstAct.querySelectorAll('[data-cinematic-step]');
    elements.forEach((element) => {
      const step = parseFloat(element.getAttribute('data-cinematic-step') || '0');
      const delay = step * 300;
      
      setTimeout(() => {
        element.classList.add('cinematic-active');
        element.classList.add('cinematic-complete');
      }, delay);
    });
  }

  console.log('✅ Cinematic scroll system ready');

  // Cleanup function
  return () => {
    console.log('🧹 Cleaning up cinematic scroll');
    sectionObserver.disconnect();
  };
}

/**
 * Initialize scroll animations for elements with data-scroll attribute
 */
export function initScrollAnimations(options: ScrollAnimationOptions = {}) {
  const {
    threshold = 0.1,
    rootMargin = '0px 0px -100px 0px',
    triggerOnce = true
  } = options;

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add('is-visible');
          
          if (triggerOnce) {
            observer.unobserve(entry.target);
          }
        } else if (!triggerOnce) {
          entry.target.classList.remove('is-visible');
        }
      });
    },
    { threshold, rootMargin }
  );

  const elements = document.querySelectorAll('[data-scroll]');
  elements.forEach((el) => observer.observe(el));

  return observer;
}

/**
 * Parallax effects (disabled for performance)
 */
export function initParallaxEffects() {
  return () => {};
}

/**
 * Initialize counter animations for numbers
 */
export function animateNumber(element: HTMLElement, target: number, duration: number = 2000) {
  const start = 0;
  const increment = target / (duration / 16);
  let current = start;

  const timer = setInterval(() => {
    current += increment;
    if (current >= target) {
      element.textContent = Math.round(target).toLocaleString();
      clearInterval(timer);
    } else {
      element.textContent = Math.round(current).toLocaleString();
    }
  }, 16);
}
