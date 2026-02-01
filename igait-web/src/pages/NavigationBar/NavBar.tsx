import { Link, useLocation } from "react-router-dom";
import { getAuth, onAuthStateChanged } from "firebase/auth";
import { useState, useEffect, useRef } from 'react';
import './navbar.css';

/**
 * @brief NavBar represents the navigation menu that is rendered above 
 * every component in this website
 * 
 * @note Depending on if the user is loggedIn in, or if they are on a 
 * mobile device, there is some logic for updating the UI to 
 * correctly display the correct layout 
 * 
 */
function NavBar() {
  // State variables for managing if the user is authenticated via Firebase
  const [loggedIn, setLoggedIn] = useState(false);
  const auth = getAuth();

  // State variables for managing if the user is on mobile
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const dropdownMenuRef = useRef<HTMLDivElement | null>(null);
  const dropdownButtonRef = useRef<HTMLButtonElement | null>(null);

  // State variable for managing the current location
  const location = useLocation();

  // Check if we're on the landing page
  const isLandingPage = location.pathname === '/menu' || location.pathname === '/';

  /**
   * @brief Monitors changes to the authentication state.
   * 
   * This effect runs whenever the `auth` variable changes. It sets up a listener 
   * for authentication state changes using `onAuthStateChanged`. When the authentication 
   * state changes, it updates the logged-in status by setting `setLoggedIn` to 
   * `true` if a user is authenticated and `false` if not. 
   * 
   * @return {void}
   */ 
  useEffect(() => {
    const unsubscribe = onAuthStateChanged(auth, (user) => {
      setLoggedIn(!!user);
    });
    return () => unsubscribe();
  }, [auth]);

  /**
   * @brief Handles clicks outside of the dropdown menu.
   * 
   * This effect adds an event listener for mouse clicks to close the dropdown menu 
   * if a click occurs outside of the dropdown menu and the dropdown button. 
   * When a click is detected, it checks whether the click target is outside the 
   * dropdown and button elements. If so, it sets the dropdown state to closed.
   * 
   * @return {void}
   */
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as Node;

      if(
        dropdownMenuRef.current &&
        !dropdownMenuRef.current.contains(target) &&
        dropdownButtonRef.current &&
        !dropdownButtonRef.current.contains(target)
      ) {
        setDropdownOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  /**
   * @brief Toggles the dropdown menu open/close state.
   * 
   * This function updates the state of the dropdown menu by flipping the 
   * previous state value. If the dropdown is currently open, it will be closed, 
   * and vice versa.
   * 
   * @return {void}
   */
  const toggleDropdown = () => {
    setDropdownOpen(prevState => !prevState);
  };

  return (
    <div className="top-menu">
      <div className="igait-text">
        <Link to={"/menu"} className="name">iGAIT</Link>
      </div>

      {/* Dropdown Button for Mobile */}
      <button
        className={`dropdown-button ${dropdownOpen ? 'hidden' : ''}`}
        onClick={toggleDropdown}
        ref={dropdownButtonRef}
      >
        ☰
      </button>

      {/* Dropdown Menu for Mobile */}
      <div
        className={`dropdown-menu ${dropdownOpen ? 'open' : 'hidden'}`}
        ref={dropdownMenuRef}
      >
        {!isLandingPage && (
          <Link to={"/menu"} onClick={toggleDropdown} className="menu-links">Home</Link>
        )}
        <Link to={"/about"} onClick={toggleDropdown} className="menu-links">About</Link>
        <Link to={"mailto:GaitStudy@niu.edu"} onClick={toggleDropdown} className="menu-links">Contact</Link>
        {loggedIn && (
          <>
            <Link to={"/assistant"} onClick={toggleDropdown} className="menu-links">Assistant</Link>
            <Link to={"/data_submission"} onClick={toggleDropdown} className="menu-links">Data Submission</Link>
            <Link to={"/history"} onClick={toggleDropdown} className="menu-links">History</Link>
            <Link to={"/home"} onClick={toggleDropdown} className="menu-links">Try iGAIT</Link>
            <Link to={"/signout"} onClick={toggleDropdown} className="menu-links">Sign Out</Link>
          </>
        )}
        {!loggedIn && (
          <>
            <Link to={"/login"} onClick={toggleDropdown} className="menu-links">Log In</Link>
            <Link to={"/signup"} onClick={toggleDropdown} className="menu-links">Sign Up</Link>
          </>
        )}
      </div>

      {/* Regular Menu for Larger Screens */}
      <div className={`menu-links-container ${dropdownOpen ? 'hidden' : ''}`}>
        {!isLandingPage && (
          <Link to={"/menu"} className="menu-links">Home</Link>
        )}
        <Link to={"/about"} className="menu-links">About</Link>
        <Link to={"mailto:GaitStudy@niu.edu"} className="menu-links">Contact</Link>
        {loggedIn && (
          <>
            <Link to={"/assistant"} className="menu-links">Assistant</Link>
            <Link to={"/data_submission"} className="menu-links">Data Submission</Link>
            <Link to={"/history"} className="menu-links">History</Link>
          </>
        )}

        {/* Action buttons on the right */}
        {loggedIn ? (
          <>
            <Link to={"/home"} className="btn">Try iGAIT</Link>
            <div className="login-button">
              <Link to={"/signout"} className="text">SIGN OUT</Link>
            </div>
          </>
        ) : (
          <>
            <div className="signup-button">
              <Link to={"/login"} className="text">LOG IN</Link>
            </div>
            <div className="login-button">
              <Link to={"/signup"} className="text">SIGN UP</Link>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default NavBar;