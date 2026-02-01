import { Navigate, Route, Routes} from "react-router-dom"
import { getAuth, onAuthStateChanged } from "firebase/auth";
import { useState, useEffect } from 'react';
import Home from '@/pages/JobSubmission/JobSubmission';
import Navbar from "@/pages/NavigationBar/NavBar"
import Hippa from "@/pages/Hippa/hippa"
import About from "@/pages/About/About"
import Assistant from "@/pages/AssistantPage/Assistant"
import Terms from "@/pages/Terms/Terms"
import Policy from "@/pages/Policy/Policy"
import Login from "@/pages/Login/LogIn"
import Signout from "@/pages/SignOut/Signout"
import Menu from "@/pages/LandingPage/Menu"
import PrivacyPolicy from "@/pages/ConsentForms/PrivacyPolicy/privacypolicy"
import VideoConsent from '@/pages/ConsentForms/Video Consent Page/videoconsent';
import HistoricalSubmissions from '@/pages/HistoricalSubmissions/historical'
import DataSubmission from "@/pages/DataSubmission/data_submission";
import Signup from "@/pages/Signup/Signup";
import "../pages/NavigationBar/navbar.css"

function App() {
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
      {/* Render the Navbar above  */}
      <Navbar />
      
      <div className='container'>
        {/* If loggedIn true, then below are available routes */}
        {loggedIn ? (
          <Routes>
            <Route path="/" element={<Navigate to="/menu" />} />
            <Route path="/home" element={<Home />} />
            <Route path="/hippa" element={<Hippa />} />
            <Route path="/menu" element={<Menu />} />
            <Route path="/about" element={<About />} />
            <Route path="/assistant" element={<Assistant />} />
            <Route path="/terms" element={<Terms />} />
            <Route path="/login" element={<Login />} />
            <Route path="/policy" element={<Policy />} />
            <Route path="/privacypolicy" element={<PrivacyPolicy />} />
            <Route path="/videoconsent" element={<VideoConsent />} />
            <Route path="/signout" element={<Signout />} />
            <Route path="/history" element={<HistoricalSubmissions />} />
            <Route path="/data_submission" element={<DataSubmission />} />
             <Route path="/signup" element={<Navigate to="/home" />} />
          </Routes>
        ) 
          // Otherwise if loggedIn false render only these routes
          : ( 
          <Routes>    
            {/* 
            If user not loggedIn and they try to submit a job, 
            just navigate them to the login form, and then
            from there they will be navigated to the job 
            submission form, after signing up       
            */}
            <Route path="/" element={<Navigate to="/menu" />} />
            <Route path="/home" element={<Login />} />
            <Route path="/" element={<Menu />} />
            <Route path="/menu" element={<Menu />} />
            <Route path="/login" element={<Login />} />
            <Route path="/terms" element={<Terms />} />
            <Route path="/about" element={<About />} />
            <Route path="/hippa" element={<Hippa />} />
            <Route path="/policy" element={<Policy />} />
            <Route path="/signup" element={<Signup />} />
            <Route path="/privacypolicy" element={<PrivacyPolicy />} />
            <Route path="/videoconsent" element={<VideoConsent />} />
          </Routes>
        )}
      </div>
    </>
  );
}

export default App;