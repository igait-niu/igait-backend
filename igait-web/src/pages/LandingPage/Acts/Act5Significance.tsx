import './Act5Significance.css';

function Act5Significance() {
  return (
    <section 
      className='viewport-section significance-section'
      data-cinematic-section
    >
      <div className='significance-content'>
        <div 
          className='significance-header'
          data-cinematic-step="0" 
          data-cinematic-duration="0.2"
          data-animation="fade"
        >
          <div className='header-icon-wrapper'>
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" fill="currentColor" viewBox="0 0 16 16">
              <path fillRule="evenodd" d="M6 2a.5.5 0 0 1 .47.33L10 12.036l1.53-4.208A.5.5 0 0 1 12 7.5h3.5a.5.5 0 0 1 0 1h-3.15l-1.88 5.17a.5.5 0 0 1-.94 0L6 3.964 4.47 8.171A.5.5 0 0 1 4 8.5H.5a.5.5 0 0 1 0-1h3.15l1.88-5.17A.5.5 0 0 1 6 2"/>
            </svg>
          </div>
          <h2 className='significance-title'>Why This Matters</h2>
        </div>

        <div className='stats-highlight-grid'>
          <div 
            className='stat-highlight-card'
            data-cinematic-step="0.2" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='stat-highlight-number'>1 in 31</div>
            <p className='stat-highlight-text'>
              children aged 8 years are autistic (CDC's ADDM Network 2025)
            </p>
          </div>

          <div 
            className='stat-highlight-card'
            data-cinematic-step="0.3" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='stat-highlight-number'>2+ years</div>
            <p className='stat-highlight-text'>
              average wait time after initial screening to receive diagnosis
            </p>
          </div>

          <div 
            className='stat-highlight-card'
            data-cinematic-step="0.4" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='stat-highlight-number'>8 years</div>
            <p className='stat-highlight-text'>
              average diagnosis age for children from rural and under-resourced families
            </p>
          </div>
        </div>

        <div 
          className='significance-text-block'
          data-cinematic-step="0.5" 
          data-cinematic-duration="0.2"
          data-animation="fade"
        >
          <p>
            The US is currently experiencing a <strong>public health crisis</strong> regarding access to timely autism diagnosis.
          </p>
          
          <p>
            Delayed diagnosis prevents children from receiving <strong>early intervention</strong> at the crucial time of their optimal developmental impact, leading to reduced quality of life and increased costs at individual, family, and societal levels.
          </p>

          <div className='barriers-box'>
            <h3 className='barriers-title'>Major Barriers:</h3>
            <ul>
              <li>Shortage of qualified professionals, especially in rural areas</li>
              <li>Unaffordable diagnostic services for many families</li>
              <li>Current practices rely on subjective observations</li>
              <li>Existing tools have demographic and language limitations</li>
            </ul>
          </div>

          <p className='call-to-action-text'>
            There is an <strong>urgent need</strong> for technologically innovative, widely accessible, low-cost, objective approaches to autism detection.
          </p>
        </div>
      </div>
    </section>
  );
}

export default Act5Significance;
