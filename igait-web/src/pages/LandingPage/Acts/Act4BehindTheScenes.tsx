import './Act4BehindTheScenes.css';

function Act4BehindTheScenes() {
  return (
    <section 
      className='viewport-section behind-scenes-section'
      data-cinematic-section
    >
      <div className='behind-scenes-content'>
        <div className='section-header-center'>
          <h2 
            className='behind-scenes-title'
            data-cinematic-step="0" 
            data-cinematic-duration="0.3"
            data-animation="fade"
          >
            Behind the Scenes
          </h2>
          <p 
            className='behind-scenes-subtitle'
            data-cinematic-step="0.15" 
            data-cinematic-duration="0.3"
            data-animation="fade"
          >
            Watch how our AI transforms raw video into insights
          </p>
        </div>

        <div className='transformation-pipeline'>
          {/* Stage 1: Raw Video */}
          <div 
            className='transformation-stage'
            data-cinematic-step="0.2" 
            data-cinematic-duration="0.2"
            data-animation="slide-up"
          >
            <div className='stage-number'>01</div>
            <div className='stage-visual'>
              <img src='/Videos/110.gif' alt='Raw video of child walking' className='stage-gif' />
            </div>
            <div className='stage-info'>
              <h3 className='stage-title'>Raw Video</h3>
              <p className='stage-description'>
                Your smartphone captures natural walking patterns in everyday settings
              </p>
            </div>
          </div>

          {/* Arrow */}
          <div 
            className='pipeline-arrow'
            data-cinematic-step="0.35" 
            data-cinematic-duration="0.1"
            data-animation="scale"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          </div>

          {/* Stage 2: AI Processing */}
          <div 
            className='transformation-stage'
            data-cinematic-step="0.4" 
            data-cinematic-duration="0.2"
            data-animation="slide-up"
          >
            <div className='stage-number'>02</div>
            <div className='stage-visual'>
              <img src='/Videos/9S.gif' alt='AI processing video with tracking points' className='stage-gif' />
            </div>
            <div className='stage-info'>
              <h3 className='stage-title'>AI Processing</h3>
              <p className='stage-description'>
                Advanced computer vision tracks movement and identifies key biomechanical points
              </p>
            </div>
          </div>

          {/* Arrow */}
          <div 
            className='pipeline-arrow'
            data-cinematic-step="0.55" 
            data-cinematic-duration="0.1"
            data-animation="scale"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          </div>

          {/* Stage 3: Skeleton Analysis */}
          <div 
            className='transformation-stage'
            data-cinematic-step="0.6" 
            data-cinematic-duration="0.2"
            data-animation="slide-up"
          >
            <div className='stage-number'>03</div>
            <div className='stage-visual'>
              <img src='/Videos/114.gif' alt='Skeleton view showing gait analysis' className='stage-gif' />
            </div>
            <div className='stage-info'>
              <h3 className='stage-title'>Gait Analysis</h3>
              <p className='stage-description'>
                Machine learning analyzes skeletal patterns to detect subtle autism indicators
              </p>
            </div>
          </div>
        </div>

        {/* Technical highlights */}
        <div 
          className='tech-highlights'
          data-cinematic-step="0.75" 
          data-cinematic-duration="0.25"
          data-animation="fade"
        >
          <div className='tech-stat'>
            <div className='tech-icon'>⚡</div>
            <div className='tech-label'>Real-time Processing</div>
          </div>
          <div className='tech-stat'>
            <div className='tech-icon'>🎯</div>
            <div className='tech-label'>30+ Data Points</div>
          </div>
          <div className='tech-stat'>
            <div className='tech-icon'>🔬</div>
            <div className='tech-label'>Research-Validated</div>
          </div>
        </div>
      </div>
    </section>
  );
}

export default Act4BehindTheScenes;
