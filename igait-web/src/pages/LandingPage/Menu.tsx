import { Link } from 'react-router-dom'
import './menu.css'
import { useState } from 'react';
import { faqs } from '@/pages/LandingPage/questions';

function Menu() {
  // State management for opening the frequently asked questions section
  const [openIndex, setOpenIndex] = useState(null);

  // Triggers when a user clicks on the frequently asked questions section
  // and renders its answer based on the index
  const toggleAnswer = (index: any) => {
    setOpenIndex(openIndex === index ? null : index);
  };

  
  // Default land user at top of page
  window.scrollTo(0, 0);
  
  return (
    <>
      <div className='home'>
        <section className='section-one'>
        <div className='large-space'>
        </div>
          <div className='intro-text'>
            <h1 className='large-text'>iGAIT</h1>
            <h1 className='large-text'>Transforming Early Autism Detection by AI-powered Gait Analysis</h1>
            <h2 className='smaller-text'>Introducing iGAIT, an innovative, objective, equitable, widely accessible, easy-to-use, free web-based tool for early autism screening</h2>
        </div>
        <div className='demo-buttons'>
          <Link to={"/signup"}>
            <a className='launch-terminal'>
              Sign Up Today!
            </a>
          </Link>
          </div>
          <div className='funding-section'>
            <b>  
              <p>
                Created by a research partnership between <a className="link-text" href="https://www.niu.edu/index.shtml">Northern Illinois University </a> 
                and <a className="link-text" href="https://www.siue.edu/">Southern Illinois University Edwardsville</a>.
              </p>
            </b>  
          </div>
          <div className='funding-section'>
            <b>  
              <p>
                Funded by <a className="link-text" href="https://iin.uillinois.edu/">Illinois Innovation Network</a>, 
                <a className="link-text" href="https://www.aim-ahead.net/"> NIH-sponsored AIM-AHEAD program</a>, 
                <a className="link-text" href="https://www.niu.edu/index.shtml"> NIU</a> and 
                <a className="link-text" href="https://www.siue.edu/"> SIUE</a> (see the 
                <a className="link-text" href="/about/#sponsors"> full list</a> of our sponsors) 
              </p>
            </b>  
          </div>
          <div className="uui-heroheader16_image-wrapper">
          </div>
          <section className="uui-section_layout49">
              <div className="uui-page-padding-3">
                  <div className="uui-container-large"></div>
              </div>
          </section>
          </section>

          <section className='section-two'>
            <div className='outer-container'>
              <div className='text-side'>
                <div className='header-div'>
                  <h1 className='section-text'>Innovation</h1>
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="header-icon" viewBox="0 0 16 16">
                    <path d="M2.5.5A.5.5 0 0 1 3 0h10a.5.5 0 0 1 .5.5q0 .807-.034 1.536a3 3 0 1 1-1.133 5.89c-.79 1.865-1.878 2.777-2.833 3.011v2.173l1.425.356c.194.048.377.135.537.255L13.3 15.1a.5.5 0 0 1-.3.9H3a.5.5 0 0 1-.3-.9l1.838-1.379c.16-.12.343-.207.537-.255L6.5 13.11v-2.173c-.955-.234-2.043-1.146-2.833-3.012a3 3 0 1 1-1.132-5.89A33 33 0 0 1 2.5.5m.099 2.54a2 2 0 0 0 .72 3.935c-.333-1.05-.588-2.346-.72-3.935m10.083 3.935a2 2 0 0 0 .72-3.935c-.133 1.59-.388 2.885-.72 3.935"/>
                  </svg>
                </div>

                <div className="section-info">
                  <p><strong>iGAIT is a web-based tool that:</strong></p>
                  <ul>
                    <li>screens children for autism by assessing their walking gait</li>
                    <li>takes home-recorded videos by parents on their phone, tablet, or camera</li>
                    <li>leverages cutting-edge video analysis and artificial intelligence technology</li>
                    <li>ensures privacy and data security at every step</li>
                    <li>is completely free to the user</li>
                  </ul>

                  <p><strong>iGAIT transforms early autism detection by being:</strong></p>
                  <ul>
                    <li><strong>innovative:</strong> it fuses advanced technologies including machine learning, computer vision, pose estimation, gait analysis, high-performance computing, and latest autism research</li>
                    <li><strong>easy-to-use:</strong> user simply uploads two 1-min videos</li>
                    <li><strong>accessible:</strong> it only requires Internet connection and a video recording device, like a smartphone</li>
                    <li><strong>equitable:</strong> it reduces gender, racial, linguistic, cultural biases by including diverse demographic backgrounds in research and development</li>
                    <li><strong>inclusive:</strong> it assumes no verbal developmental milestones, language spoken, nor cultural identity by the child</li>
                    <li><strong>fast:</strong> it runs on NIU’s state-of-the-art computing infrastructure to deliver results within minutes</li>
                    <li><strong>non-invasive:</strong> the child wears no sensors or gadgets</li>
                  </ul>
                </div>

                <h2 className="highlight-title">From Videos to Insights</h2>
                <div className="video-row">
                  <div className='video-row-wrapper'>
                  <img className='video-embed' src='/Videos/110.gif' alt='Raw Video' />
                  <img className='video-embed' src='/Videos/9S.gif' alt='Processed Video' />
                  <img className='video-embed' src='/Videos/114.gif' alt='Skeleton View' />
                  </div>
                </div>

                <h2 className="highlight-title">Watch Our Research Story</h2>
                <div className="youtube-container">
                  <iframe
                    width="100%"
                    height="450"
                    src="https://www.youtube.com/embed/Y22kgic6YZE"
                    title="Trek Talk Video"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowFullScreen
                  ></iframe>
                </div>
                <div className='small-space'></div>
              </div>
            </div>
          </section>

          <section className='section-one'>
            <div className='outer-container'>
              <div className='text-side'>
                <div className='header-div'>
                  <h1 className='section-text'>Significance</h1>
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="header-icon" viewBox="0 0 16 16">
                    <path fillRule="evenodd" d="M6 2a.5.5 0 0 1 .47.33L10 12.036l1.53-4.208A.5.5 0 0 1 12 7.5h3.5a.5.5 0 0 1 0 1h-3.15l-1.88 5.17a.5.5 0 0 1-.94 0L6 3.964 4.47 8.171A.5.5 0 0 1 4 8.5H.5a.5.5 0 0 1 0-1h3.15l1.88-5.17A.5.5 0 0 1 6 2"/>
                  </svg>
                </div>

                <p className='section-info'>
                  About 1 in 31 (3.2%) children aged 8 years are autistic (estimates from <a href="https://www.cdc.gov/mmwr/volumes/74/ss/ss7402a1.htm" style={{color: '#1976D2'}}>CDC’s ADDM Network 2025</a>).
                </p>

                <p className='section-info'>
                  The US is currently experiencing a public health crisis regarding access to timely autism diagnosis.
                </p>

                <p className='section-info'>
                  Although autistic children could show clinical signs before two years of age, they typically wait over two years (26.9 months) after initial screening to receive a formal diagnosis, resulting in an average age of diagnosis between 4 to 5 years. Moreover, children from under-resourced families and rural backgrounds are diagnosed at an even later age, closer to 8 years on average. For autistic girls, the delay is even longer, with an average of 10 years between the time of first reported autistic traits and a formal diagnosis. Consequently, delayed diagnosis prevents children from receiving early intervention at the crucial time of their optimal developmental impact, leading to reduced quality of life and increased costs associated with autism at individual, family, and societal levels.
                </p>

                <p className='section-info'>
                  A longstanding concern for decades, the delay remains challenging because of several major systemic and technological barriers including (1) there is a shortage of qualified professionals, such as developmental-behavioral pediatricians and clinical psychologists, particularly in rural and underserved areas; (2) autism diagnostic services can be unaffordable for some families due to lack of insurance coverage and high costs; (3) current diagnostic practices solely rely on parent reports and clinician observations, thus can be subjective and time-consuming; and (4) existing diagnostic tools have limitations such as insufficient coverage of children from all demographic backgrounds and non-verbal children in developing the assessment criteria, unavailability in other popular languages than English, and omitting motor impairment among other important autistic traits.
                </p>

                <p className='section-info'>
                  Therefore, there is an urgent need for technologically innovative, widely accessible, low-cost, objective approaches to autism detection.
                </p>
              </div>
            </div>
          </section>

          
          {/* <section className='section-one'>
            <div className='outer-container'>
                <div className='text-side'>
                <div className='header-div'>
                  <h1 className='section-text'>Project Description</h1>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="header-icon" viewBox="0 0 16 16">
                    <path fillRule="evenodd" d="M10.854 7.146a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708 0l-1.5-1.5a.5.5 0 1 1 .708-.708L7.5 9.793l2.646-2.647a.5.5 0 0 1 .708 0"/>
                    <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1z"/>
                    <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0z"/>
                  </svg>
                  </div>
                  <p className='section-info'><strong>Gait Analysis for Detection:</strong> The application leverages gait analysis—a method that examines walking patterns—to predict the likelihood of ASD. This method has shown promise in differentiating children with ASD from neurotypical peers.</p>
                  <p className='section-info'><strong>Technical Approach:</strong> Using open-source human pose estimation algorithms, the application will analyze gait features captured through video recordings. Data collected will also include caregiver questionnaires assessing symptom severity and motor skills.</p>
                </div>
            </div>
          </section>  */}

          <section className='section-two'>
            <div className='outer-container'>
              <div className='text-side'>
                <div className='header-div'>
                <h1 className='section-text'>Try iGAIT in 3 Steps</h1>
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="header-icon" viewBox="0 0 16 16">
                  <path d="M11 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM5 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z"/>
                  <path d="M8 14a1 1 0 1 0 0-2 1 1 0 0 0 0 2"/>
                  </svg>
                </div>
                <li className='section-info'>Step 1: Sign up at <a style={{color: '#1976D2'}} href="https://igaitapp.com/signup">igaitapp.com/signup</a> using your email address or Google account.</li>
                <li className='section-info'>Step 2: Go to “Try iGAIT” tab and upload a front-facing and a side-facing video of your child walking, along with basic demographic information. </li>
                <li className='section-info'>Step 3: Click “Submit”, and check your email for result. </li>
              </div>
            </div>
          </section> 

          {/* <section className='section-one'>
            <div className='outer-container'>
            <div className='text-side'>
            <div className='header-div'>
                <h1 className='section-text'>How iGAIT Works</h1>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className='header-icon' viewBox="0 0 16 16">
                  <path d="M6 12.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 0 1h-3a.5.5 0 0 1-.5-.5M3 8.062C3 6.76 4.235 5.765 5.53 5.886a26.6 26.6 0 0 0 4.94 0C11.765 5.765 13 6.76 13 8.062v1.157a.93.93 0 0 1-.765.935c-.845.147-2.34.346-4.235.346s-3.39-.2-4.235-.346A.93.93 0 0 1 3 9.219zm4.542-.827a.25.25 0 0 0-.217.068l-.92.9a25 25 0 0 1-1.871-.183.25.25 0 0 0-.068.495c.55.076 1.232.149 2.02.193a.25.25 0 0 0 .189-.071l.754-.736.847 1.71a.25.25 0 0 0 .404.062l.932-.97a25 25 0 0 0 1.922-.188.25.25 0 0 0-.068-.495c-.538.074-1.207.145-1.98.189a.25.25 0 0 0-.166.076l-.754.785-.842-1.7a.25.25 0 0 0-.182-.135"/>
                    <path d="M8.5 1.866a1 1 0 1 0-1 0V3h-2A4.5 4.5 0 0 0 1 7.5V8a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1v1a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-1a1 1 0 0 0 1-1V9a1 1 0 0 0-1-1v-.5A4.5 4.5 0 0 0 10.5 3h-2zM14 7.5V13a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V7.5A3.5 3.5 0 0 1 5.5 4h5A3.5 3.5 0 0 1 14 7.5"/>
                </svg>
            </div>
                <p className='section-info'>iGAIT is a data-driven application designed to predict Autism Spectrum Disorder (ASD) indices for children in underserved and low-income communities. Developed by researchers from Southern Illinois University and Northern Illinois University, the app leverages smartphone technology to provide accessible early screening for ASD.</p>
                <p className='section-info'>The project addresses significant delays in ASD diagnosis—children in low-income areas often receive diagnoses later than their wealthier counterparts. By using gait analysis, which involves assessing walking patterns through video recordings, iGAIT aims to identify gait abnormalities associated with ASD.</p>
                <h2 className='section-text'>Key Components:</h2>
                <ul className='section-info'>
                    <li ><strong>Data Collection:</strong>Gait data is captured using cameras, along with caregiver questionnaires assessing autism symptoms and motor skills.</li>
                    <li><strong>Machine Learning:</strong> A deep learning algorithm analyzes extracted gait features to predict the ASD index.</li>
                    <li><strong>Accessibility:</strong> The application is designed for smartphone use, allowing families to upload videos and receive immediate feedback.</li>
                </ul>
                <p className='section-info'>The project aligns with the goals of the Illinois Autism Task Force by promoting early diagnosis and intervention, ultimately aiming to improve outcomes for children at risk for ASD.</p>
            </div>
            </div>
          </section> */}

          <section className='section-one'>
            <div className='outer-container'>
              <div className='text-side'>
                <div className='header-div'>
                  <h1 className='section-text'>Meet the Team</h1>
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className='header-icon' viewBox="0 0 16 16">
                    <path d="M6 6.207v9.043a.75.75 0 0 0 1.5 0V10.5a.5.5 0 0 1 1 0v4.75a.75.75 0 0 0 1.5 0v-8.5a.25.25 0 1 1 .5 0v2.5a.75.75 0 0 0 1.5 0V6.5a3 3 0 0 0-3-3H6.236a1 1 0 0 1-.447-.106l-.33-.165A.83.83 0 0 1 5 2.488V.75a.75.75 0 0 0-1.5 0v2.083c0 .715.404 1.37 1.044 1.689L5.5 5c.32.32.5.754.5 1.207"/>
                    <path d="M8 3a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3"/>
                  </svg>
                </div>

                <div className='team-grid'>
                  <div className='team-member'>
                    <img src='/Images/Team Photos/sonal.jpg' className='member-image' />
                    <div className='flex-div-wrapper'>
                      <h3 className='member-name'>
                        <a href='http://www.sinanonal.com/' target='_blank' rel='noopener noreferrer'>Sinan Onal, PhD</a>
                      </h3>
                    </div>
                  </div>

                  <div className='team-member'>
                    <img src='/Images/Team Photos/wang.jpg' className='member-image' />
                    <div className='flex-div-wrapper'>
                      <h3 className='member-name'>
                        <a href='https://www.wang-zt.com/' target='_blank' rel='noopener noreferrer'>Ziteng Wang, PhD</a>
                      </h3>
                    </div>
                  </div>

                  <div className='team-member'>
                    <img src='/Images/Team Photos/gladfelter.jpg' className='member-image' />
                    <div className='flex-div-wrapper'>
                      <h3 className='member-name'>
                        <a href='https://www.chhs.niu.edu/about/staff/gladfelter.shtml' target='_blank' rel='noopener noreferrer'>Allison Gladfelter, PhD, CCC-SLP</a>
                      </h3>
                    </div>
                  </div>

                  <div className='team-member'>
                    <img src='/Images/Team Photos/buac.jpg' className='member-image' />
                    <div className='flex-div-wrapper'>
                      <h3 className='member-name'>
                        <a href='https://www.chhs.niu.edu/about/staff/buac.shtml' target='_blank' rel='noopener noreferrer'>Milijana Buac, PhD, CCC-SLP</a>
                      </h3>
                    </div>
                  </div>
                </div>

                <p className='thank-you-text'>
                  This research would be impossible without the talent and endeavors of the <a href='/about#student-team' className='link-text'>student team</a>. We thank their contributions and hard work!
                </p>
              </div>
            </div>
          </section>

          {/* <section className='section-one'>
            <div className='outer-container'>
          <div className='header-div'>
            <h1 className='section-text'>Frequently Asked Questions</h1>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className='header-icon' viewBox="0 0 16 16">
              <path fillRule="evenodd" d="M4.475 5.458c-.284 0-.514-.237-.47-.517C4.28 3.24 5.576 2 7.825 2c2.25 0 3.767 1.36 3.767 3.215 0 1.344-.665 2.288-1.79 2.973-1.1.659-1.414 1.118-1.414 2.01v.03a.5.5 0 0 1-.5.5h-.77a.5.5 0 0 1-.5-.495l-.003-.2c-.043-1.221.477-2.001 1.645-2.712 1.03-.632 1.397-1.135 1.397-2.028 0-.979-.758-1.698-1.926-1.698-1.009 0-1.71.529-1.938 1.402-.066.254-.278.461-.54.461h-.777ZM7.496 14c.622 0 1.095-.474 1.095-1.09 0-.618-.473-1.092-1.095-1.092-.606 0-1.087.474-1.087 1.091S6.89 14 7.496 14"/>
            </svg>
            </div>
            </div>
            <div className='outer-padding'>
            <div className='faq-container'>
                {faqs.map((faq, index) => (
                  <div key={index} className='faq-item'>
                    <div className='question-div'>
                    <button className='faq-question' onClick={() => toggleAnswer(index)}>
                      {faq.question} 
                    </button>

                {openIndex === index ? (
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="logi-sign" viewBox="0 0 16 16">
                    <path fillRule="evenodd" d="M2 8a.5.5 0 0 1 .5-.5h11a.5.5 0 0 1 0 1h-11A.5.5 0 0 1 2 8"/>
                  </svg>
                ) : (
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="logi-sign" viewBox="0 0 16 16">
                    <path fillRule="evenodd" d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2"/>
                  </svg>
                )}     
                </div>
                  {openIndex === index && <div className='faq-answer'>{faq.answer}</div>}
                  </div>
                ))}
              </div>
            </div>
          </section> */}

          <section className='section-two'>
          <section className="uui-section_layout10">
              <div className="uui-page-padding-3">
                  <div className="uui-container-large">
                      <div className="uui-padding-vertical-xhuge">
                          <section className="uui-section_cta07">
                              <div className="uui-page-padding-3">
                                  <div className="uui-container-large">
                                      <div className="uui-padding-vertical-xhuge-2">
                                          <div className="uui-cta07_component">
                                            <div className="uui-signup-form_wrapper">
                                                <a className="uui-button-5-copy is-button-large w-inline-block">
                                                  <Link to={"/home"}>
                                                      <div className='launch-text'>Try iGAIT for Free</div>
                                                  </Link>
                                                </a>
                                            </div>
                                          </div>
                                      </div>
                                  </div>
                              </div>
                          </section>
                          
                          <footer className="uui-footer07_component">
                              <div className="uui-page-padding-3">
                                  <div className="uui-container-large">
                                      <div className="uui-padding-vertical-xlarge">
                                          <div className="uui-footer07_top-wrapper">
                                              <a href="#" id="w-node-_03b22535-d2b0-7908-8893-0080a10a8e14-72ae7e2c" className="uui-footer07_logo-link w-nav-brand">
                                                  <div className="uui-logo_component">
                                                      <div className="text-block-18">Disclaimer</div>
                                                  </div>
                                              </a>
                                          </div>
                                          <p className="paragraph">
                                             iGAIT is an AI-based tool designed to assist in detecting autism. 
                                             It is important to note that the result provided by iGAIT must not 
                                             be interpreted as a diagnosis. Nor should the result be considered a 
                                             substitute for professional medical advice, diagnosis, or intervention. 
                                             Please always consult your physician or other qualified healthcare professionals 
                                             for a comprehensive evaluation and guidance regarding autism. The creators of 
                                             iGAIT do not assume any responsibility for the consequences of using the tool 
                                             or for any decisions made based on its results.
                                          <strong>
                                              <br/>
                                          </strong>
                                          </p>
                                        <div className="uui-footer07_bottom-wrapper">
                                      <div className="uui-text-size-small text-color-gray500">Copyright by Northern Illinois University and Southern Illinois University Edwardsville</div>
                                    <div className="w-layout-grid uui-footer07_legal-list">
                                <Link to={"/terms"}>
                                    <a target="_blank" className="uui-footer07_legal-link">Terms</a>
                                </Link>
                                <Link to={"/policy"}>
                                    <a className="uui-footer07_legal-link">Privacy</a>
                                </Link>
                                <Link to="/about" className="uui-footer07_legal-link">About Us</Link>
                              </div>
                            <div className="uui-footer07_contact">
                            <Link to={"mailto:GaitStudy@niu.edu"}>
                              <div className="uui-text-size-small text-color-gray500">Contact us: GaitStudy@niu.edu</div>
                            </Link>
                            </div>
                      </div>
                      </div>
                      </div>
                      </div>
                      </footer>
                    </div>
                </div>
            </div>
        </section>
        </section>
      </div>
    </>
  )
}

export default Menu