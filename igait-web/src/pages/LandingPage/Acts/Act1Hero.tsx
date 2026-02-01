import './Act1Hero.css';

function Act1Hero() {
  return (
    <section 
      className='viewport-section hero-section'
      data-cinematic-section
    >
      <div className='hero-content'>
        <h1 
          className='hero-statement'
          data-cinematic-step="0" 
          data-cinematic-duration="0.3"
          data-animation="fade"
        >
          2 Years Too Late
        </h1>
        <p 
          className='hero-subtext'
          data-cinematic-step="0.2" 
          data-cinematic-duration="0.3"
          data-animation="fade"
        >
          Children wait 27+ months for autism diagnosis after parents first notice signs. 
          iGAIT screens in minutes, giving families answers when they need them most.
        </p>
        <a 
          href='/signup'
          className='hero-cta'
          data-cinematic-step="0.4" 
          data-cinematic-duration="0.3"
          data-animation="scale"
        >
          Screen Your Child Today
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 16 16">
            <path fillRule="evenodd" d="M1 8a.5.5 0 0 1 .5-.5h11.793l-3.147-3.146a.5.5 0 0 1 .708-.708l4 4a.5.5 0 0 1 0 .708l-4 4a.5.5 0 0 1-.708-.708L13.293 8.5H1.5A.5.5 0 0 1 1 8"/>
          </svg>
        </a>
      </div>

      {/* Scroll indicator */}
      <div className='scroll-indicator'>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <path d="M12 5v14M19 12l-7 7-7-7"/>
        </svg>
      </div>
    </section>
  );
}

export default Act1Hero;
