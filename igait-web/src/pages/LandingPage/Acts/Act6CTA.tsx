import { Link } from 'react-router-dom';
import { getAuth, onAuthStateChanged } from "firebase/auth";
import { useState, useEffect } from 'react';
import './Act6CTA.css';

function Act6CTA() {
  const [loggedIn, setLoggedIn] = useState(false);

  useEffect(() => {
    const auth = getAuth();
    const unsubscribe = onAuthStateChanged(auth, (user) => {
      setLoggedIn(!!user);
    });
    return () => unsubscribe();
  }, []);

  return (
    <section 
      className='viewport-section cta-section'
      data-cinematic-section
    >
      <div className='cta-content'>
        <div 
          className='team-section'
          data-cinematic-step="0" 
          data-cinematic-duration="0.2"
          data-animation="fade"
        >
          <div className='section-header'>
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" fill="currentColor" viewBox="0 0 16 16">
              <path d="M6 6.207v9.043a.75.75 0 0 0 1.5 0V10.5a.5.5 0 0 1 1 0v4.75a.75.75 0 0 0 1.5 0v-8.5a.25.25 0 1 1 .5 0v2.5a.75.75 0 0 0 1.5 0V6.5a3 3 0 0 0-3-3H6.236a1 1 0 0 1-.447-.106l-.33-.165A.83.83 0 0 1 5 2.488V.75a.75.75 0 0 0-1.5 0v2.083c0 .715.404 1.37 1.044 1.689L5.5 5c.32.32.5.754.5 1.207"/>
              <path d="M8 3a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3"/>
            </svg>
            <h2 className='section-title'>Meet the Team</h2>
          </div>

          <div className='team-grid'>
            <div 
              className='team-member-card'
              data-cinematic-step="0.15" 
              data-cinematic-duration="0.1"
              data-animation="slide-up"
            >
              <img src='/Images/Team Photos/sonal.jpg' alt='Sinan Onal' className='member-photo' />
              <h3 className='member-name'>
                <a href='http://www.sinanonal.com/' target='_blank' rel='noopener noreferrer'>
                  Sinan Onal, PhD
                </a>
              </h3>
            </div>

            <div 
              className='team-member-card'
              data-cinematic-step="0.25" 
              data-cinematic-duration="0.1"
              data-animation="slide-up"
            >
              <img src='/Images/Team Photos/wang.jpg' alt='Ziteng Wang' className='member-photo' />
              <h3 className='member-name'>
                <a href='https://www.wang-zt.com/' target='_blank' rel='noopener noreferrer'>
                  Ziteng Wang, PhD
                </a>
              </h3>
            </div>

            <div 
              className='team-member-card'
              data-cinematic-step="0.35" 
              data-cinematic-duration="0.1"
              data-animation="slide-up"
            >
              <img src='/Images/Team Photos/gladfelter.jpg' alt='Allison Gladfelter' className='member-photo' />
              <h3 className='member-name'>
                <a href='https://www.chhs.niu.edu/about/staff/gladfelter.shtml' target='_blank' rel='noopener noreferrer'>
                  Allison Gladfelter, PhD, CCC-SLP
                </a>
              </h3>
            </div>

            <div 
              className='team-member-card'
              data-cinematic-step="0.45" 
              data-cinematic-duration="0.1"
              data-animation="slide-up"
            >
              <img src='/Images/Team Photos/buac.jpg' alt='Milijana Buac' className='member-photo' />
              <h3 className='member-name'>
                <a href='https://www.chhs.niu.edu/about/staff/buac.shtml' target='_blank' rel='noopener noreferrer'>
                  Milijana Buac, PhD, CCC-SLP
                </a>
              </h3>
            </div>
          </div>

          <p 
            className='team-acknowledgment'
            data-cinematic-step="0.55" 
            data-cinematic-duration="0.1"
            data-animation="fade"
          >
            This research would be impossible without the talent and endeavors of the{' '}
            <a href='/about#student-team' className='link-accent'>student team</a>. 
            We thank their contributions and hard work!
          </p>
        </div>

        <div 
          className='cta-box'
          data-cinematic-step="0.65" 
          data-cinematic-duration="0.2"
          data-animation="scale-up"
        >
          <h2 className='cta-title'>Ready to Get Started?</h2>
          <p className='cta-description'>
            {loggedIn 
              ? "Start screening your child today with our AI-powered gait analysis tool."
              : "Join thousands of families using iGAIT for early autism screening. Fast, free, and accessible from home."
            }
          </p>
          <div className='cta-buttons'>
            {loggedIn ? (
              <>
                <Link to="/home" className='cta-button primary'>
                  Try iGAIT Now
                </Link>
                <Link to="/about" className='cta-button secondary'>
                  Learn More
                </Link>
              </>
            ) : (
              <>
                <Link to="/signup" className='cta-button primary'>
                  Sign Up Today
                </Link>
                <Link to="/login" className='cta-button secondary'>
                  Log In
                </Link>
              </>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}

export default Act6CTA;
