import { useNavigate } from 'react-router-dom';
import { getAuth, signOut } from "firebase/auth";
import { useState } from 'react';
import './signout.css'

/**
 * @brief Handle signout functionality by using Firebase functions
 */
function Signout() {
    const navigate = useNavigate();
    const [isLoading, setIsLoading] = useState(false);
    
    /**
     * @brief Signs the user out of the application.
     * 
     * This function calls the `signOut` method from the Firebase authentication 
     * service to log the user out. Upon successful logout, it navigates the user 
     * to the '/menu' route. If an error occurs during the sign-out process, 
     * the error message is logged to the console.
     * 
     * @return {void}
     */
    function handleSignOut(){    
        setIsLoading(true);
        const auth = getAuth();
        signOut(auth).then(() => {    
            localStorage.setItem("privacyPolicy", "false");
            localStorage.setItem("videoConsent", "false");
            navigate('/menu');     
        }).catch((error) => {
            console.log(error);
            setIsLoading(false);
        });
    }

    function handleCancel() {
        navigate(-1); // Go back to previous page
    }
    
    return(
        <div className='signout-page'>  
            <div className='signout-overlay' onClick={handleCancel}></div>
            <div className='signout-modal'>
                <div className='signout-icon'>👋</div>
                <h1 className='signout-title'>Sign Out?</h1>
                <p className='signout-description'>
                    Are you sure you want to sign out? You'll need to log back in to access your submissions and use iGAIT.
                </p>
                <div className='signout-buttons'>
                    <button 
                        className='button-cancel' 
                        onClick={handleCancel}
                        disabled={isLoading}
                    >
                        Cancel
                    </button>
                    <button 
                        className='button-signout' 
                        onClick={handleSignOut}
                        disabled={isLoading}
                    >
                        {isLoading ? 'Signing out...' : 'Sign Out'}
                    </button>
                </div>
            </div>
        </div>
    )
}

export default Signout;