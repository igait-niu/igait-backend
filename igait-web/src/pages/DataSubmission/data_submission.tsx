import { getAuth } from "firebase/auth";
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useToast } from "@/components/Toast";
import { Loader2 } from "lucide-react";
import { submitWithRetry } from "@/utils/apiHelper";
import '@/pages/DataSubmission/data_submission.css';

/**
 * @brief This component allows a user to submit their walking video data 
 *  
 * @note A POST request is sent to the /contribute API 
 */
function DataSubmission() {
    const [displayPDF, setDisplayPDF] = useState(false);
    const navigate = useNavigate();
    const firebaseAuth = getAuth();
    const current_user_id: string | null | undefined = firebaseAuth.currentUser?.uid;
    const current_user_email: string | null | undefined = firebaseAuth.currentUser?.email;
    const { showToast } = useToast();
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [uploadProgress, setUploadProgress] = useState(0);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        const { name, value, type } = e.target;
        if (type === "file") {
          setFormData({ ...formData, [name]: (e.target as HTMLInputElement).files![0] });
        } else if (type === "checkbox") {
          const checked = (e.target as HTMLInputElement).checked;
          setFormData({ ...formData, [name]: checked });
        } else {
          setFormData({ ...formData, [name]: value });
        }
    };

    const [formData, setFormData] = useState({
        email: current_user_email,
        uid: current_user_id,
        name: "",
        fileuploadfront: null,
        fileuploadside: null
    });
    
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (isSubmitting) return;

        if (
            !formData.email ||
            !formData.fileuploadfront ||
            !formData.fileuploadside ||
            !formData.name
        ) {
          showToast("Please complete all the required fields!", "warning");
          return;
        }

        setIsSubmitting(true);
        setUploadProgress(0);

        try {
          const formBody = new FormData();
          formBody.append('uid', formData.uid as string);
          formBody.append('name', formData.name as string);
          formBody.append('fileuploadfront', formData.fileuploadfront as Blob);
          formBody.append('email', formData.email);
          formBody.append('fileuploadside', formData.fileuploadside as Blob);
    
          // Use the enhanced API helper with retry logic
          const result = await submitWithRetry({
            endpoint: "https://api.igaitapp.com/api/v1/contribute",
            formData: formBody,
            timeoutMs: 60000, // 60 seconds for large videos
            onProgress: setUploadProgress
          });

          if (result.success) {
            showToast(result.message, "success");

            // Wait a moment before navigating so user sees the toast
            setTimeout(() => {
              navigate("/menu");
            }, 1500);
          } else {
            showToast(result.message, "error");
          }

        } catch (error) {
          console.error("Unexpected submission error:", error);
          showToast(
            "An unexpected error occurred. Please try again.",
            "error"
          );
        } finally {
          setIsSubmitting(false);
          setUploadProgress(0);
        }
    };
    
    return ( 
        <div className="About">
        <div className="background">
            <div className='about-header'>
            </div>
            <div className='about-content'>
            <div className='hippa-form-inner'>
            <h2>Data Submission</h2>
            <p>If you talked to one of our research members about data collection, please use this page.</p>
            
            {/* Tutorial videos upload  */}
            <div className="tutorial-videos-container">
              <div className="tutorial-card">
                <h3 className="tutorial-card-title">How to record your child's walking gait</h3>
                <p className="tutorial-card-desc">
                  This video shows you how to record the two videos you will need to upload. Please make sure your videos follow these simple guidelines.
                </p>
                <div className="responsive-video">
                <iframe
                  width="560"
                  height="315"
                  src="https://mediaspace.niu.edu/embed/secure/iframe/entryId/1_z7bejbcu/uiConfId/45549541/st/0"
                  title="New IGAIT Video Tutorial - Made with Clipchamp"
                  allowFullScreen
                  frameBorder="0"
                ></iframe>
                </div>
              </div>
              
              <div className="tutorial-card">
                <h3 className="tutorial-card-title">How to Submit your Data into iGAIT</h3>
                <p className="tutorial-card-desc">
                  This video shows you how to fill out the form, agree to the required terms, and upload your child's videos.
                </p>
                <div className="responsive-video">
                <iframe
                  width="560"
                  height="315"
                  src="https://mediaspace.niu.edu/embed/secure/iframe/entryId/1_u2q0gcej/uiConfId/45549541/st/0"
                  title="iGAIT_VIDEO_DEMO"
                  allowFullScreen
                  frameBorder="0"
                ></iframe>
                </div>
              </div>
            </div> 
       
          {/* Tutorial videos upload  */}
                
            <form className="data-submission" onSubmit={handleSubmit}>
                <div className="checkbox-div">
                  <div>
                    <input type="checkbox" name="name" onChange={handleChange} disabled={isSubmitting}></input>
                  </div>
                  <div>
                    <label>Yes, I was directed by the iGAIT research group to submit videos to this website.</label>
                  </div>
                </div>
                <button 
                  type="button" 
                  onClick={() => setDisplayPDF(!displayPDF)}
                  disabled={isSubmitting}
                >
                  {displayPDF ? "View The Parent Permission Form in English" : "View The Parent Permission Form in Spanish"}
                </button>
                {displayPDF ? 
                  <>
                    <a 
                      href="/Documents/Consent-Form-Spanish.pdf" 
                      target="_blank" 
                      rel="noopener noreferrer" 
                      style={{ display: "inline" }}
                    >
                      Click to View PDF in New Window
                    </a>
                    <iframe
                      src="/Documents/Consent-Form-Spanish.pdf"
                      title="Consent Form Spanish"
                      width="100%"
                      style={{ minHeight: "100vh", border: "none" }}
                    ></iframe>
                  </>
                  : 
                  <>
                  <a 
                    href="/Documents/Consent-Form-English.pdf"                  
                    target="_blank" 
                    rel="noopener noreferrer" 
                    style={{ display: "inline" }}
                  >
                    Click to View PDF in New Window
                  </a>
                  <iframe
                    src="/Documents/Consent-Form-English.pdf"                  
                    title="Consent Form English"
                    width="100%"
                    style={{ minHeight: "100vh", border: "none" }}
                  ></iframe>
                  </>
                }
                
                {/* Name Agreement */}
                <label className="data-submission-label">
                  By typing my name below, I confirm that I give consent for my child to participate in this study as a research volunteer.
                  <input 
                    type="text" 
                    name="name" 
                    placeholder="Please Type Your Name" 
                    onChange={handleChange} 
                    required
                    disabled={isSubmitting}
                  />
                </label>
                <label className="data-submission-label">
                  By typing my name below, I confirm that I give my consent for my child to be video recorded while walking in the room/or on the treadmill, and for these recordings to be used in this research study.
                  <input 
                    type="text" 
                    name="name" 
                    placeholder="Please Type Your Name" 
                    onChange={handleChange} 
                    required
                    disabled={isSubmitting}
                  />
                </label>
                {/* Checkbox Agreements */}                
                <strong><p>I consent, by clicking the checkbox next to any/each statement below, that: <br></br>
                  Please note that your child is still eligible to participate if you do not grant permission to some or all of the below statements
                </p></strong>
                <div className="checkbox-div">
                  <div>
                    <input type="checkbox" onChange={handleChange} disabled={isSubmitting}></input>
                  </div>
                  <div>
                    <label>The recordings or still pictures made from recordings can be used in scientific publications.</label>
                  </div>
                </div>
                
                <div className="checkbox-div">
                  <div>
                    <input type="checkbox" onChange={handleChange} disabled={isSubmitting}></input>
                  </div>
                  <div>
                    <label>The recordings can be used in classrooms for teaching purposes.</label>
                  </div>
                </div>
                <div className="checkbox-div">
                  <div>
                    <input type="checkbox" onChange={handleChange} disabled={isSubmitting}></input>
                  </div>
                  <div>
                    <label>The recordings can be used in presentations at professional meetings/conferences.</label>
                  </div>
                </div>
  
                <div className="checkbox-div">
                  <div>
                    <input type="checkbox" onChange={handleChange} disabled={isSubmitting}></input>
                  </div>
                  <div>
                    <label>The recordings can be used in presentations about child development to non-scientific groups.</label>
                  </div>
                </div>
                <label className="data-submission-label">
                    Front Facing Walking Video:
                    <div className="input-flex-div"> 
                    <input 
                      type="file" 
                      name="fileuploadfront" 
                      onChange={handleChange} 
                      required
                      disabled={isSubmitting}
                      accept="video/*"
                    />
                    </div>
                </label>
                <label className="data-submission-label">
                    Side Facing Walking Video:
                    <div className="input-flex-div">
                    <input 
                      type="file" 
                      name="fileuploadside" 
                      onChange={handleChange} 
                      required
                      disabled={isSubmitting}
                      accept="video/*"
                    />
                    </div>
                </label>
                
                {isSubmitting && uploadProgress > 0 && (
                  <div className="progress-container">
                    <div className="progress-bar" style={{ width: `${uploadProgress}%` }}></div>
                    <span className="progress-text">{uploadProgress}%</span>
                  </div>
                )}
                
                <button className="data-submission_button" type="submit" disabled={isSubmitting}>
                  {isSubmitting ? (
                    <>
                      <Loader2 className="spinner" size={20} />
                      Submitting...
                    </>
                  ) : (
                    "Submit"
                  )}
                </button>
            </form>                 
        </div>
        </div>
    </div>
  </div>
  );
}

export default DataSubmission;