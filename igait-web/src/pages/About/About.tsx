import '@/pages/About/about.css';

function About() {
    return (
        <div className="about-page">
            <div className="about-hero">
                <h1 className="about-hero-title">About iGAIT</h1>
                <p className="about-hero-subtitle">
                    Advancing early autism detection through innovative technology and collaborative research
                </p>
            </div>

            <div className="about-container">
                <section className="about-section" id="student-team">
                    <div className="section-icon">👥</div>
                    <h2>Student Research Team</h2>
                    
                    <div className="university-group">
                        <h3>Northern Illinois University (NIU)</h3>
                        
                        <div className="student-category">
                            <h4>Graduate Students</h4>
                            <ul>
                                <li>Gabriela Ibarra</li>
                                <li>Mahesh Raju</li>
                                <li>Ricardo Torres</li>
                                <li>Alicia LaRouech</li>
                                <li>Viviana Cortes</li>
                                <li>Noelle Veome</li>
                            </ul>
                        </div>

                        <div className="student-category">
                            <h4>Undergraduate Students</h4>
                            <ul>
                                <li>John White</li>
                                <li>Michael Sensenbrenner</li>
                                <li>Luke Ali</li>
                                <li>Angelica Sanyal</li>
                                <li>Zoey Proper</li>
                                <li>Rachel Conolly</li>
                            </ul>
                        </div>
                    </div>

                    <div className="university-group">
                        <h3>Southern Illinois University Edwardsville (SIUE)</h3>
                        
                        <div className="student-category">
                            <h4>Graduate Students</h4>
                            <ul>
                                <li>Amit Shrestha</li>
                                <li>Prashanna Pandit</li>
                            </ul>
                        </div>

                        <div className="student-category">
                            <h4>Undergraduate Students</h4>
                            <ul>
                                <li>Cody Schaefer</li>
                                <li>Vinhhy Pham</li>
                            </ul>
                        </div>
                    </div>
                </section>

                <section className="about-section">
                    <div className="section-icon">💰</div>
                    <h2>Sponsors</h2>
                    <p className="section-description">
                        We are grateful to the following sponsors for their funding support:
                    </p>
                    <div className="sponsor-grid">
                        <div className="sponsor-card">Illinois Innovation Network</div>
                        <div className="sponsor-card">NIH AIM-AHEAD</div>
                        <div className="sponsor-card">NIU Office of Student Engagement and Experiential Learning</div>
                        <div className="sponsor-card">NIU College of Health and Human Sciences</div>
                        <div className="sponsor-card">SIUE Graduate School</div>
                        <div className="sponsor-card">SIUE Undergraduate Research and Creative Activities</div>
                    </div>
                </section>

                <section className="about-section">
                    <div className="section-icon">🤝</div>
                    <h2 id="sponsors">Special Thanks</h2>
                    <p className="section-description">
                        We appreciate the continued support by the following partners:
                    </p>
                    <div className="partner-list">
                        <div className="partner-item">NIU Center for Research Computing and Data</div>
                        <div className="partner-item">NIU Center for the Interdisciplinary Study of Language and Literacy</div>
                        <div className="partner-item">NIU Autism Caregiver Group</div>
                        <div className="partner-item">NIU Speech-Language-Hearing Clinic</div>
                        <div className="partner-item">NIU Division of Research and Innovation Partnerships</div>
                        <div className="partner-item">NIU Foundation</div>
                    </div>
                </section>
            </div>
        </div>
    );
}

export default About;