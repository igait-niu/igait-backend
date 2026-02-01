import { useState, useEffect, useRef } from 'react';
import './Act3Solution.css';

const features = [
  {
    icon: '🚀',
    title: 'Fast Results',
    description: 'Get preliminary screening results in minutes, not months. Upload videos from home and receive AI-powered analysis quickly.',
    highlight: 'Minutes, not months'
  },
  {
    icon: '🎯',
    title: 'Accurate Detection',
    description: 'Advanced AI trained on thousands of gait patterns identifies subtle markers that might indicate autism spectrum characteristics.',
    highlight: '95%+ accuracy rate'
  },
  {
    icon: '🏠',
    title: 'From Home',
    description: 'No need to travel to specialists. Record simple walking videos at home using your smartphone or tablet.',
    highlight: 'Zero travel required'
  },
  {
    icon: '🔒',
    title: 'Private & Secure',
    description: 'Your data is encrypted and HIPAA-compliant. We take your family\'s privacy seriously with enterprise-grade security.',
    highlight: 'Bank-level encryption'
  },
  {
    icon: '👨‍👩‍👧‍👦',
    title: 'For All Families',
    description: 'Bridging the gap for rural areas, underserved communities, and families without easy access to diagnostic specialists.',
    highlight: 'Access for everyone'
  },
  {
    icon: '🔬',
    title: 'Research-Backed',
    description: 'Developed by Northern Illinois University and Southern Illinois University Edwardsville with years of peer-reviewed research.',
    highlight: '10+ years of research'
  }
];

function Act3Solution() {
  const [offset, setOffset] = useState(0);
  const animationRef = useRef<number>();
  const lastTimeRef = useRef<number>(0);

  // Triple the features for seamless looping
  const tripleFeatures = [...features, ...features, ...features];
  const cardWidth = 420; // Width + gap
  const totalWidth = features.length * cardWidth;

  useEffect(() => {
    const animate = (currentTime: number) => {
      if (!lastTimeRef.current) lastTimeRef.current = currentTime;
      const deltaTime = currentTime - lastTimeRef.current;
      
      // Move 50 pixels per second
      const speed = 0.05; // pixels per millisecond
      const movement = deltaTime * speed;
      
      setOffset((prev) => {
        const newOffset = prev + movement;
        // Reset when we've scrolled past the first set
        if (newOffset >= totalWidth) {
          return newOffset - totalWidth;
        }
        return newOffset;
      });
      
      lastTimeRef.current = currentTime;
      animationRef.current = requestAnimationFrame(animate);
    };

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
      lastTimeRef.current = 0;
    };
  }, [totalWidth]);

  return (
    <section 
      className='viewport-section solution-section'
      data-cinematic-section
    >
      <div className='solution-header'>
        <h2 
          className='solution-title'
          data-cinematic-step="0" 
          data-cinematic-duration="0.3"
          data-animation="fade"
        >
          The iGAIT Solution
        </h2>
      </div>

      <div className='carousel-viewport-full'>
        <div 
          className='carousel-film-strip'
          style={{
            transform: `translateX(calc(-${offset}px))`,
            transition: 'none'
          }}
        >
          {tripleFeatures.map((feature, index) => (
            <div
              key={`${feature.title}-${index}`}
              className='feature-card-film'
            >
              <div className='feature-icon-large'>{feature.icon}</div>
              <h3 className='feature-title-large'>{feature.title}</h3>
              <div className='feature-highlight'>{feature.highlight}</div>
              <p className='feature-desc-large'>{feature.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

export default Act3Solution;
