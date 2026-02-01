import '@/pages/About/about.css';

/**
 * @brief This component contains information about the overall goal of the research
 * as well as information for about everyone involved
 *  
 * @note This is just a static web page, for users to read
 */
function About() {
    return (
        <div className="About">
            <div className="background">
                <div className='about-header'>
                </div>
                
                <div className='about-content'>
                    <h2>Student Research Team</h2>

                    <p><strong>Northern Illinois University (NIU):</strong></p>
                    <p><em>Graduate Students:</em></p>
                    <ul>
                        <li>Gabriela Ibarra</li>
                        <li>Mahesh Raju</li>
                        <li>Ricardo Torres</li>
                        <li>Alicia LaRouech</li>
                        <li>Viviana Cortes</li>
                        <li>Noelle Veome</li>
                    </ul>
                    <p><em>Undergraduate Students:</em></p>
                    <ul>
                        <li>John White</li>
                        <li>Michael Sensenbrenner</li>
                        <li>Luke Ali</li>
                        <li>Angelica Sanyal</li>
                        <li>Zoey Proper</li>
                        <li>Rachel Conolly</li>
                    </ul>

                    <p><strong>Southern Illinois University Edwardsville (SIUE):</strong></p>
                    <p><em>Graduate Students:</em></p>
                    <ul>
                        <li>Amit Shrestha</li>
                        <li>Prashanna Pandit</li>
                    </ul>
                    <p><em>Undergraduate Students:</em></p>
                    <ul>
                        <li>Cody Schaefer</li>
                        <li>Vinhhy Pham</li>
                    </ul>

                    <h2>Sponsors</h2>
                    <p>We are grateful to the following sponsors for their funding support:</p>
                    <ul>
                        <li>Illinois Innovation Network</li>
                        <li>NIH AIM-AHEAD</li>
                        <li>NIU Office of Student Engagement and Experiential Learning</li>
                        <li>NIU College of Health and Human Sciences</li>
                        <li>SIUE Graduate School</li>
                        <li>SIUE Undergraduate Research and Creative Activities</li>
                    </ul>

                    <h2 id="sponsors">Special Thanks</h2>
                    <p>We appreciate the continued support by the following partners:</p>
                    <ul>
                        <li>NIU Center for Research Computing and Data</li>
                        <li>NIU Center for the Interdisciplinary Study of Language and Literacy</li>
                        <li>NIU Autism Caregiver Group</li>
                        <li>NIU Speech-Language-Hearing Clinic</li>
                        <li>NIU Division of Research and Innovation Partnerships</li>
                        <li>NIU Foundation</li>
                    </ul>
                </div>
            </div>
        </div>
    );
}

export default About;