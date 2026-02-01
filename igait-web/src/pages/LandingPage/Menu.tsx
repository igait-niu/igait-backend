import { Link } from 'react-router-dom'
import './menu.css'
import { useEffect } from 'react';
import { initScrollAnimations, initParallaxEffects, initCinematicScroll } from '@/utils/scrollAnimations';
import Act1Hero from './Acts/Act1Hero';
import Act2Problem from './Acts/Act2Problem';
import Act3Solution from './Acts/Act3Solution';
import Act4HowItWorks from './Acts/Act4HowItWorks';
import Act4BehindTheScenes from './Acts/Act4BehindTheScenes';
import Act5Significance from './Acts/Act5Significance';
import Act6CTA from './Acts/Act6CTA';

function Menu() {
  // Initialize scroll animations on mount
  useEffect(() => {
    const observer = initScrollAnimations({
      threshold: 0.15,
      rootMargin: '0px 0px -100px 0px',
      triggerOnce: true
    });

    const cleanupParallax = initParallaxEffects();
    const cleanupCinematic = initCinematicScroll();

    return () => {
      observer.disconnect();
      cleanupParallax();
      cleanupCinematic();
    };
  }, []);
  
  window.scrollTo(0, 0);
  
  return (
    <div className='home'>
      {/* Act 1: Hero */}
      <Act1Hero />
      
      {/* Act 2: The Problem */}
      <Act2Problem />
      
      {/* Act 3: The Solution */}
      <Act3Solution />
      
      {/* Act 4: How It Works */}
      <Act4HowItWorks />
      
      {/* Act 4.5: Behind the Scenes - AI Transformation */}
      <Act4BehindTheScenes />
      
      {/* Act 5: Significance */}
      <Act5Significance />
      
      {/* Act 6: Team & CTA */}
      <Act6CTA />
      
      {/* Footer */}
      <footer className='footer'>
        <div className='footer-content'>
          <div className='footer-disclaimer'>
            <h3 className='footer-disclaimer-title'>Important Disclaimer</h3>
            <p className='footer-disclaimer-text'>
              iGAIT is an AI-based screening tool designed to assist in early autism detection. Results must not be interpreted as a diagnosis and should not substitute professional medical advice. Always consult your physician or qualified healthcare professional for comprehensive evaluation and guidance.
            </p>
          </div>

          <div className='footer-links'>
            <Link to="/terms" className='footer-link'>Terms of Service</Link>
            <Link to="/policy" className='footer-link'>Privacy Policy</Link>
            <Link to="/about" className='footer-link'>About Us</Link>
            <a href="mailto:GaitStudy@niu.edu" className='footer-link'>Contact Us</a>
          </div>

          <p className='footer-copyright'>
            © {new Date().getFullYear()} Northern Illinois University & Southern Illinois University Edwardsville. All rights reserved.
          </p>
        </div>
      </footer>
    </div>
  )
}

export default Menu