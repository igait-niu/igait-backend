import { useNavigate } from 'react-router-dom';
import { getAuth, signOut } from "firebase/auth";
import './signout.css'

/**
 * @brief Handle signout functionality by using Firebase functions
 */
function Signout() {
    const navigate = useNavigate();
    
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
        const auth = getAuth();
        signOut(auth).then(() => {    
            localStorage.setItem("privacyPolicy", "false");
            localStorage.setItem("videoConsent", "false");
            navigate('/menu');     
        }).catch((error) => {
            console.log(error);
        });
    }    
    return(
        <div className='Signout'>  
        <div className='background'></div>
            <div className='logout-container'>
                <h1 className='signout-text'>Signout</h1>
                <p className='signout-text-small'>Are you sure you want to sign out?</p>
                <button className='logout-button' onClick={handleSignOut}>Logout</button>
            </div>
        </div>
    )
}

export default Signout;