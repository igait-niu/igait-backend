import { useState } from 'react';
import './Act2Problem.css';

function Act2Problem() {
  const [expandedCards, setExpandedCards] = useState<Set<string>>(new Set());

  const toggleCard = (cardId: string, e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    const newSet = new Set(expandedCards);
    if (newSet.has(cardId)) {
      newSet.delete(cardId);
    } else {
      newSet.add(cardId);
    }
    setExpandedCards(newSet);
  };

  return (
    <section 
      className='viewport-section problem-section'
      data-cinematic-section
    >
      <div className='split-content'>
        <div className='stat-column'>
          <div 
            className='stat-card' 
            data-cinematic-step="0" 
            data-cinematic-duration="0.2"
            data-animation="slide-right"
          >
            <div className='stat-number'>1 in 31</div>
            <div className='stat-label'>Children are autistic</div>
            <div className='stat-description'>
              That's 3.2% of 8-year-olds in the United States, according to CDC's 2025 data.
            </div>
            <div 
              className={`expandable-card ${expandedCards.has('stat1') ? 'expanded' : ''}`}
              onClick={(e) => toggleCard('stat1', e)}
            >
              <div className='expandable-header'>
                <span>Learn more about the data</span>
                <span>{expandedCards.has('stat1') ? '−' : '+'}</span>
              </div>
              <div className='expandable-content'>
                Data from CDC's Autism and Developmental Disabilities Monitoring (ADDM) Network shows rising prevalence, highlighting the critical need for early detection tools like iGAIT.
              </div>
            </div>
          </div>

          <div 
            className='stat-card' 
            data-cinematic-step="0.2" 
            data-cinematic-duration="0.2"
            data-animation="slide-right"
          >
            <div className='stat-number'>27+</div>
            <div className='stat-label'>Months of waiting</div>
            <div className='stat-description'>
              Average time between initial screening and formal autism diagnosis.
            </div>
          </div>

          <div 
            className='stat-card' 
            data-cinematic-step="0.4" 
            data-cinematic-duration="0.2"
            data-animation="slide-right"
          >
            <div className='stat-number'>4-5</div>
            <div className='stat-label'>Years old at diagnosis</div>
            <div className='stat-description'>
              By the time most children receive a diagnosis, critical early intervention windows have narrowed.
            </div>
          </div>
        </div>

        <div 
          className='timeline-column' 
          data-cinematic-step="0" 
          data-cinematic-duration="0.6"
          data-animation="slide-left"
        >
          <div className='timeline-wrapper'>
            <div className='timeline-line'></div>
            
            <div className='timeline-item'>
              <div className='timeline-dot'></div>
              <div className='timeline-title'>The Wait Begins</div>
              <div className='timeline-text'>
                Parents notice early signs, but accessing diagnostic services proves challenging due to long waitlists and limited specialists.
              </div>
            </div>

            <div className='timeline-item'>
              <div className='timeline-dot'></div>
              <div className='timeline-title'>Rural & Underserved Families</div>
              <div className='timeline-text'>
                Children from rural areas and under-resourced families wait even longer—often closer to 8 years for a diagnosis.
              </div>
            </div>

            <div className='timeline-item'>
              <div className='timeline-dot'></div>
              <div className='timeline-title'>The Gender Gap</div>
              <div className='timeline-text'>
                Autistic girls face an average 10-year delay between first reported traits and formal diagnosis.
              </div>
            </div>

            <div className='timeline-item'>
              <div className='timeline-dot'></div>
              <div className='timeline-title'>Lost Opportunities</div>
              <div className='timeline-text'>
                Delayed diagnosis means delayed intervention—missing the crucial developmental window when early support is most effective.
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

export default Act2Problem;
