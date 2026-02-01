import { useState} from 'react';
import { initializeApp } from "firebase/app";
import { signInWithPopup, getAuth, createUserWithEmailAndPassword, signInWithEmailAndPassword, sendPasswordResetEmail, GoogleAuthProvider } from "firebase/auth";
import { useNavigate} from 'react-router-dom';
import './Signup.css'

export const firebaseConfig = {
    apiKey: "AIzaSyDN-cTPmRowdB1sERiTDVqRzA3-sM4_T2g",
    authDomain: "network-technology-project.firebaseapp.com",
    databaseURL: "https://network-technology-project-default-rtdb.firebaseio.com",
    projectId: "network-technology-project",
    storageBucket: "network-technology-project.appspot.com",
    messagingSenderId: "607332254529",
    appId: "1:607332254529:web:e38cef20befe8ad61957c0",
    measurementId: "G-LQWGV17JN5"
};

const app = initializeApp(firebaseConfig);
export const auth = getAuth(app);
const provider = new GoogleAuthProvider();


export default function Signup(){
        // Bunch state management variables used to update various variables
        // that are getting updated in input frames
        const navigate = useNavigate();
    
        const [error, setError] = useState("");
        const [invalidEmail, setInvalidEmail] = useState(false);
        const [loginMenuStatus, setloginMenuStatus] = useState(false);
        const setLoginStatusDisplayTrue = () => setloginMenuStatus(true);    
        const setLoginStatusDisplayFalse = () => setloginMenuStatus(false);
        const [resetPasswordStatus, setresetPasswordStatus] = useState(false);
        const setresetPasswordStatusTrue = () => setresetPasswordStatus(true);
        const setresetPasswordStatusFalse = () => setresetPasswordStatus(false);
        const [userCredentials, setUserCredentials] = useState({email: '', password: '', passwordConfirm: ''});
        function handleCredentials(event: any) {
        const { name, value } = event.target;
    
        /**
         * @brief Updates the user credentials state.
         * 
         * This function updates the user credentials state by spreading the previous state and
         * modifying the specific field indicated by the `name` parameter with the new `value`.
         * 
         * @param {string} name - The name of the field to update in the user credentials.
         * @param {any} value - The new value to set for the specified field.
         * @return {void}
         */
        setUserCredentials(prevState => ({
            ...prevState,
            [name]: value
            }));
        }
    
        /**
         * @brief Handles user signup process.
         * 
         * This function is triggered when the user submits the signup form. It prevents the default
         * form submission behavior and validates the user's email and password. If the email is invalid
         * or the passwords do not match, appropriate error states are set. Upon successful signup,
         * a new user account is created using Firebase Authentication, and the user is navigated to the 
         * home page. The email field in the user credentials is reset after a successful signup.
         *
         * @param {Event} e - The event object representing the form submission.
         * @return {void}
         */
        function handleSignup(e: any) {
            e.preventDefault();
    
            if(userCredentials.password !== userCredentials.passwordConfirm) {
                setError("Error: Passwords do not match.");
                return;
            }
                createUserWithEmailAndPassword(auth, userCredentials.email, userCredentials.password)
                .then((userCredential) => {
                const user = userCredential.user;
                console.log(user)
                navigate('/home');
                setUserCredentials(prevState => ({
                    ...prevState,
                    email: ''
                }));
            })
            .catch((error) => {
                console.log(error);
                setError(error.message);
            });
        }
    
        /**
         * @brief Handles user login via Google authentication.
         * 
         * This function is triggered when the user initiates a Google login. It prevents the default
         * form submission behavior and attempts to sign in the user using a Google authentication popup. 
         * Upon successful login, it navigates the user to the home page. If the login attempt fails, 
         * an error message is set to inform the user of the failure.
         *
         * @param {Event} e - The event object representing the form submission.
         * @return {void}
         */
        function googleLogin(e: any) {
            e.preventDefault();
            
            signInWithPopup(auth, provider)
            .then((result) => {
                const user = result.user;
                console.log(user);
                navigate('/home');
            }).catch((error) => {
                console.log(error); 
                setError(error.message);
            });
        }
    
        /**
         * @brief Handles the user login process.
         * 
         * This function is triggered when the user submits the login form. It prevents the default
         * form submission behavior, attempts to sign in the user with the provided email and password
         * using Firebase Authentication, and navigates to the home page upon successful login. 
         * If the login attempt fails, it sets an error message to inform the user.
         *
         * @param {Event} e - The event object representing the form submission.
         * @returns {void} 
         *
         */
        function handleLogin(e: any) {
            e.preventDefault();
            signInWithEmailAndPassword(auth, userCredentials.email, userCredentials.password)
            .then((userCredential) => {
              const user = userCredential.user;
              console.log(user)
             navigate('/home');
            })
            .catch((error) => {
                console.log(error);
                setError(error.message);
            });
        }
    
        // Manage state for user email's 
        const [emailReset, setEmailReset] = useState('');
    
        /**
         * @brief Resets a user's password based on their login credentials and email.
         * 
         * This function initiates a password reset process for the user by sending a password
         * reset email to the specified email address. If the operation is successful, the user
         * will receive an email with instructions to reset their password. If the operation fails,
         * an error message from the Firebase API will be printed to the console.
         *
         * @return {void}
        */
        function handlePasswordReset() {
            sendPasswordResetEmail(auth, emailReset)
            .then(() => {
                setInvalidEmail(true);
            })
            .catch((error) => {
                console.log(error);
                setError(error.message);
            })
        }
        
    return (
        <div className="Login">
            {!resetPasswordStatus && <div className="background1">
    
            <h2 className='header-title'>CREATE YOUR ACCOUNT</h2>
    
            <div className='google-login'>
                <button className='login-google-button' title='login' onClick={googleLogin}>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="google-logo" viewBox="0 0 16 16">
                    <path d="M15.545 6.558a9.4 9.4 0 0 1 .139 1.626c0 2.434-.87 4.492-2.384 5.885h.002C11.978 15.292 10.158 16 8 16A8 8 0 1 1 8 0a7.7 7.7 0 0 1 5.352 2.082l-2.284 2.284A4.35 4.35 0 0 0 8 3.166c-2.087 0-3.86 1.408-4.492 3.304a4.8 4.8 0 0 0 0 3.063h.003c.635 1.893 2.405 3.301 4.492 3.301 1.078 0 2.004-.276 2.722-.764h-.003a3.7 3.7 0 0 0 1.599-2.431H8v-3.08z"/>
                </svg>        
                    <p>{loginMenuStatus ? 'LOGIN WITH GOOGLE' : 'SIGN UP WITH GOOGLE'}</p>        
                </button>
            </div>
            
            <div className='or-display'>
                <p>OR</p>
            </div>
    
            <form className="email-container" onSubmit={loginMenuStatus ? handleLogin : handleSignup}>
                <div className="email-box" >
                    <input title="Please fill out this field." type="email" name="email" onChange={handleCredentials} value={userCredentials.email} required></input> 
                    <span>EMAIL *</span >
                </div>
            
                <div className="password-container">
                    <div className="password-box">
                        <input type="password" title="Please fill out this field." name="password" onChange={handleCredentials} value={userCredentials.password} required/> 
                        <span>PASSWORD *</span>
                    </div>
            
                    {!loginMenuStatus && <div className="password-confirm-box" >
                        <input type="password" title="Please fill out this field." name="passwordConfirm" onChange={handleCredentials} value={userCredentials.passwordConfirm} required/> 
                        <span>CONFIRM PASSWORD *</span>
                    </div>}
                    
                    {error && <div className="error-message">{error}</div>}
                    
                    <h2 className='header-title'>TERMS OF USE</h2>
                    
                    <div className="checkbox-div">
                        <div>
                            <input id="consent-agreement" name="consent-agreement" type="checkbox" required />
                        </div>
                        <div>
                            <label>
                                I have read and understood the <a className="link-text" href="/privacypolicy">Notice of Privacy Practice</a>.  
                            </label>
                        </div>
                    </div>
                    <div className="checkbox-div">
                        <div>
                            <input id="consent-agreement" name="consent-agreement" type="checkbox" required />
                        </div>
                        <div>
                            <label>
                                I have read and agree to the <a className="link-text" href="/videoconsent">Video Consent Agreement</a>.  
                            </label>
                        </div>
                    </div>

                    <button type="submit" className='login-account-button' title='login'>
                        {loginMenuStatus ? 'LOGIN' : 'SIGN UP'}
                    </button>
                </div>
            </form>

        </div>}  
</div>
    )
}