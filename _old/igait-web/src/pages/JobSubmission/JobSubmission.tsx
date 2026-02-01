import { useEffect, useState } from "react";
import { useNavigate } from 'react-router-dom';
import "./job_submission.css"
import "/src/pages/ConsentForms/Video Consent Page/videoconsent.css"
import { getAuth, onAuthStateChanged } from "firebase/auth";
import { useToast } from "@/components/Toast";
import { Loader2 } from "lucide-react";
import { submitWithRetry } from "@/utils/apiHelper";
import "../ConsentForms/Video Consent Page/videoconsent.css"
import "../ConsentForms/PrivacyPolicy/privacypolicy.css"

function JobSubmission() {
  const navigate = useNavigate();
  const firebaseAuth = getAuth();
  const current_user_id: string | null | undefined = firebaseAuth.currentUser?.uid;
  const { showToast } = useToast();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);

  const [formData, setFormData] = useState({
    age: "",
    weight: "",
    heightFeet: "",
    heightInches: "",
    sex: "",
    ethnicity: "",
    userRole: "",
    email: "",
    fileuploadfront: null,
    fileuploadside: null,
    uid: current_user_id
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    if (type === "file") {
      setFormData({ ...formData, [name]: (e.target as HTMLInputElement).files![0] });
    } else {
      setFormData({ ...formData, [name]: value });
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (isSubmitting) return;

    if (
      !formData.age ||
      !formData.weight ||
      !formData.heightFeet ||
      !formData.heightInches ||
      !formData.sex ||
      !formData.ethnicity ||
      !formData.userRole ||
      !formData.email ||
      !formData.fileuploadfront ||
      !formData.fileuploadside
    ) {
      showToast("Please complete all the required fields!", "warning");
      return;
    }

    setIsSubmitting(true);
    setUploadProgress(0);

    try {
      const formBody = new FormData();
      const height = `${formData.heightFeet}'${formData.heightInches}`;

      formBody.append('uid', formData.uid as string);
      formBody.append('age', formData.age);
      formBody.append('ethnicity', formData.ethnicity);
      formBody.append('sex', formData.sex);
      formBody.append('height', height);
      formBody.append('weight', formData.weight);
      formBody.append('email', formData.email);
      formBody.append('fileuploadfront', formData.fileuploadfront as Blob);
      formBody.append('fileuploadside', formData.fileuploadside as Blob);

      // Use the enhanced API helper with retry logic
      const result = await submitWithRetry({
        endpoint: "https://api.igaitapp.com/api/v1/upload",
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

  const [loggedIn, setLoggedIn] = useState(false);

  useEffect(() => {
    const auth = getAuth();

    onAuthStateChanged(auth, (user) => {
      if (user) {
        setLoggedIn(true);
      } else {
        setLoggedIn(false);
      }
    });
  }, []);

  return (
    <>
      {loggedIn &&
        <div className="Job-Form">
          <form className="form-background" onSubmit={handleSubmit}>
            <div className="form-header-div">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="medical-icon" viewBox="0 0 16 16">
                <path d="M7.5 5.5a.5.5 0 0 0-1 0v.634l-.549-.317a.5.5 0 1 0-.5.866L6 7l-.549.317a.5.5 0 1 0 .5.866l.549-.317V8.5a.5.5 0 1 0 1 0v-.634l.549.317a.5.5 0 1 0 .5-.866L8 7l.549-.317a.5.5 0 1 0-.5-.866l-.549.317zm-2 4.5a.5.5 0 0 0 0 1h5a.5.5 0 0 0 0-1zm0 2a.5.5 0 0 0 0 1h5a.5.5 0 0 0 0-1z" />
                <path d="M14 14V4.5L9.5 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2M9.5 3A1.5 1.5 0 0 0 11 4.5h2V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h5.5z" />
              </svg>
              <h2>Demographical Information</h2>
            </div>
            <label>
              Age:
              <input type="number" name="age" value={formData.age} onChange={handleChange} min={1} max={115} required disabled={isSubmitting} />
            </label>
            <label>
              Weight (lbs):
              <input type="number" name="weight" value={formData.weight} onChange={handleChange} min={1} max={500} required disabled={isSubmitting} />
            </label>
            <label>
              Height (feet):
              <input type="number" name="heightFeet" value={formData.heightFeet} onChange={handleChange} min={1} max={12} required disabled={isSubmitting} />
            </label>
            <label>
              Height (inches):
              <input type="number" name="heightInches" value={formData.heightInches} onChange={handleChange} min={1} max={11} required disabled={isSubmitting} />
            </label>
            <label>
              Sex:
              <select name="sex" value={formData.sex} onChange={handleChange} required disabled={isSubmitting}>
                <option value="">Select</option>
                <option value="male">Male</option>
                <option value="female">Female</option>
                <option value="other">Other</option>
              </select>
            </label>
            <label>
              Ethnicity:
              <select name="ethnicity" value={formData.ethnicity} onChange={handleChange} required disabled={isSubmitting}>
                <option value="">Select</option>
                <option value="africanAmerican">African American/Black</option>
                <option value="nativeAmerican">Native American/American Indian</option>
                <option value="asian">Asian</option>
                <option value="hispanic">Hispanic/Latino</option>
                <option value="caucasian">Caucasian/White</option>
                <option value="pacificIslander">Pacific Islander</option>
              </select>
            </label>
            <label>
              Who is Completing This Form?
              <select name="userRole" value={formData.userRole} onChange={handleChange} required disabled={isSubmitting}>
                <option value="">Select</option>
                <option value="parent">Parent</option>
                <option value="doctor">Doctor</option>
                <option value="schoolOfficial">School Official</option>
                <option value="sibling">Sibling</option>
                <option value="grandparent">Grandparent</option>
              </select>
            </label>
            <label>
              What Email Would you Like to Use?:
              <input type="email" name="email" value={formData.email} onChange={handleChange} required disabled={isSubmitting} />
            </label>
            <label>
              Front Facing Walking Video:
              <input type="file" name="fileuploadfront" onChange={handleChange} required disabled={isSubmitting} accept="video/*" />
            </label>
            <label>
              Side Facing Walking Video:
              <input type="file" name="fileuploadside" onChange={handleChange} required disabled={isSubmitting} accept="video/*" />
            </label>

            {isSubmitting && uploadProgress > 0 && (
              <div className="progress-container">
                <div className="progress-bar" style={{ width: `${uploadProgress}%` }}></div>
                <span className="progress-text">{uploadProgress}%</span>
              </div>
            )}

            <button type="submit" disabled={isSubmitting}>
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
      }
    </>
  );
}

export default JobSubmission;