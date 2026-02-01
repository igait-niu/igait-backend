/**
 * Authentication store for Firebase Auth state management
 * Uses Svelte 5 runes for reactivity
 */

import { 
	getAuth, 
	onAuthStateChanged, 
	signInWithEmailAndPassword,
	createUserWithEmailAndPassword,
	signInWithPopup,
	GoogleAuthProvider,
	signOut as firebaseSignOut,
	type User as FirebaseUser 
} from 'firebase/auth';
import type { AuthState, User } from '$lib/types';
import { type Result, Ok, Err, AppError, tryAsync } from '$lib/result';

/**
 * Convert Firebase user to our User type
 */
function toUser(firebaseUser: FirebaseUser): User {
	return {
		uid: firebaseUser.uid,
		email: firebaseUser.email ?? '',
		displayName: firebaseUser.displayName ?? firebaseUser.email ?? 'User',
		photoURL: firebaseUser.photoURL ?? '',
		emailVerified: firebaseUser.emailVerified
	};
}

/**
 * Authentication store managing Firebase auth state
 */
class AuthStore {
	#state = $state<AuthState>({ status: 'loading' });
	#unsubscribe: (() => void) | undefined;

	/**
	 * Get the current auth state
	 */
	get state(): AuthState {
		return this.#state;
	}

	/**
	 * Initialize the auth listener - call this once on app startup
	 */
	initialize(): void {
		if (this.#unsubscribe) return; // Already initialized

		const auth = getAuth();
		this.#unsubscribe = onAuthStateChanged(auth, (firebaseUser) => {
			if (firebaseUser) {
				this.#state = {
					status: 'authenticated',
					user: toUser(firebaseUser)
				};
			} else {
				this.#state = { status: 'unauthenticated' };
			}
		});
	}

	/**
	 * Clean up the auth listener
	 */
	destroy(): void {
		this.#unsubscribe?.();
		this.#unsubscribe = undefined;
	}

	/**
	 * Sign in with email and password
	 */
	async signInWithEmail(email: string, password: string): Promise<Result<User, AppError>> {
		const auth = getAuth();
		
		const result = await tryAsync(
			async () => {
				const credential = await signInWithEmailAndPassword(auth, email, password);
				return toUser(credential.user);
			},
			'Failed to sign in'
		);

		return result;
	}

	/**
	 * Create a new account with email and password
	 */
	async signUpWithEmail(email: string, password: string): Promise<Result<User, AppError>> {
		const auth = getAuth();
		
		const result = await tryAsync(
			async () => {
				const credential = await createUserWithEmailAndPassword(auth, email, password);
				return toUser(credential.user);
			},
			'Failed to create account'
		);

		return result;
	}

	/**
	 * Sign in with Google popup
	 */
	async signInWithGoogle(): Promise<Result<User, AppError>> {
		const auth = getAuth();
		const provider = new GoogleAuthProvider();
		
		const result = await tryAsync(
			async () => {
				const credential = await signInWithPopup(auth, provider);
				return toUser(credential.user);
			},
			'Failed to sign in with Google'
		);

		return result;
	}

	/**
	 * Sign out the current user
	 */
	async signOut(): Promise<Result<void, AppError>> {
		const auth = getAuth();
		
		return tryAsync(
			() => firebaseSignOut(auth),
			'Failed to sign out'
		);
	}

	/**
	 * Get the current user's ID token for API calls
	 */
	async getIdToken(): Promise<Result<string, AppError>> {
		const auth = getAuth();
		const user = auth.currentUser;

		if (!user) {
			return Err(new AppError('No authenticated user'));
		}

		return tryAsync(
			() => user.getIdToken(),
			'Failed to get authentication token'
		);
	}
}

/**
 * Singleton auth store instance
 */
export const authStore = new AuthStore();
