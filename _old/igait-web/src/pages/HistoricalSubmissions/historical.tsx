import { useState } from "react";
import { Mail, Send } from "lucide-react";
import '@/pages/HistoricalSubmissions/historical.css';

/**
 * @brief This component provides a simple way for users to request their historical submissions
 * by emailing the support team directly.
 * 
 * @note The old API endpoint was removed, so we now redirect users to email support.
 */
function HistoricalSubmissions() {
    const supportEmail = "support@igaitapp.com"; // Update with actual support email
    const [emailSent, setEmailSent] = useState(false);

    const handleEmailClick = () => {
        const subject = encodeURIComponent("Request for Historical Submissions");
        const body = encodeURIComponent(
            "Hello iGAIT Support Team,\n\n" +
            "I would like to request my historical submissions from the iGAIT system.\n\n" +
            "Please include:\n" +
            "- Number of submissions you'd like to receive\n" +
            "- Date range (start and end dates)\n" +
            "- Whether you want original videos included\n" +
            "- Whether you want skeleton videos included\n\n" +
            "Thank you!"
        );
        
        window.location.href = `mailto:${supportEmail}?subject=${subject}&body=${body}`;
        setEmailSent(true);
    };

    return (
        <div className="About">
            <div className="background">
                <div className="about-header"></div>
                <div className="about-content">
                    <div className="historical-email-container">
                        <Mail size={64} className="email-icon" />
                        <h2>Request Historical Submissions</h2>
                        <p>
                            To request a record of your previous submissions to iGAIT, 
                            please contact our support team via email. We'll send you 
                            your historical data including gait analysis results and videos.
                        </p>
                        
                        <div className="email-info-card">
                            <h3>What to Include in Your Request:</h3>
                            <ul>
                                <li>Number of submissions you'd like to receive</li>
                                <li>Date range (start and end dates)</li>
                                <li>Whether you want original videos included</li>
                                <li>Whether you want skeleton analysis videos included</li>
                            </ul>
                        </div>

                        <button 
                            onClick={handleEmailClick} 
                            className="email-button"
                        >
                            <Send size={20} />
                            Email Support Team
                        </button>

                        {emailSent && (
                            <p className="success-message">
                                ✓ Email client opened! Please send your request.
                            </p>
                        )}

                        <p className="contact-info">
                            Or email us directly at: <a href={`mailto:${supportEmail}`}>{supportEmail}</a>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default HistoricalSubmissions;