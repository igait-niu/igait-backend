import './Act4HowItWorks.css';

function Act4HowItWorks() {
  return (
    <section 
      className='viewport-section how-it-works-section'
      data-cinematic-section
    >
      <div className='how-it-works-content'>
        <h2 
          className='how-it-works-title'
          data-cinematic-step="0" 
          data-cinematic-duration="0.2"
          data-animation="fade"
        >
          How iGAIT Works
        </h2>

        <div className='steps-container'>
          <div 
            className='step-card'
            data-cinematic-step="0.15" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='step-number'>1</div>
            <div className='step-icon'>📱</div>
            <h3 className='step-title'>Record Videos</h3>
            <p className='step-description'>
              Use your smartphone to record two short videos: one of your child walking toward the camera, and one from the side.
            </p>
          </div>

          <div 
            className='step-arrow'
            data-cinematic-step="0.25" 
            data-cinematic-duration="0.1"
            data-animation="fade"
          >
            →
          </div>

          <div 
            className='step-card'
            data-cinematic-step="0.3" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='step-number'>2</div>
            <div className='step-icon'>🤖</div>
            <h3 className='step-title'>AI Analysis</h3>
            <p className='step-description'>
              Our advanced AI analyzes gait patterns, identifying subtle biomechanical markers associated with autism spectrum characteristics.
            </p>
          </div>

          <div 
            className='step-arrow'
            data-cinematic-step="0.4" 
            data-cinematic-duration="0.1"
            data-animation="fade"
          >
            →
          </div>

          <div 
            className='step-card'
            data-cinematic-step="0.45" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='step-number'>3</div>
            <div className='step-icon'>📊</div>
            <h3 className='step-title'>Get Results</h3>
            <p className='step-description'>
              Receive a detailed screening report within minutes, helping you decide whether to pursue formal diagnostic evaluation.
            </p>
          </div>

          <div 
            className='step-arrow'
            data-cinematic-step="0.55" 
            data-cinematic-duration="0.1"
            data-animation="fade"
          >
            →
          </div>

          <div 
            className='step-card'
            data-cinematic-step="0.6" 
            data-cinematic-duration="0.15"
            data-animation="slide-up"
          >
            <div className='step-number'>4</div>
            <div className='step-icon'>👨‍⚕️</div>
            <h3 className='step-title'>Consult Professionals</h3>
            <p className='step-description'>
              Share your results with healthcare providers to expedite the diagnostic process and access early intervention services.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}

export default Act4HowItWorks;
