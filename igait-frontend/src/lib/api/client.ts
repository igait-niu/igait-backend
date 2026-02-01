/**
 * Core API client with Result-based error handling
 * All API calls return Result<T, AppError> - never throw!
 */

import { type Result, Ok, Err, AppError, tryAsync } from '$lib/result';
import { authStore } from '$lib/stores';
import { API_ENDPOINTS, DEFAULT_TIMEOUT_MS } from './config';
import type { ContributionRequest, ProgressCallback } from './types';
import { validateVideoFile, validateRequired, validateEmail } from './validation';

/**
 * Submit a gait analysis contribution
 */
export async function submitContribution(
	request: ContributionRequest,
	onProgress?: ProgressCallback
): Promise<Result<string, AppError>> {
	// Validate all fields first
	const emailResult = validateEmail(request.email);
	if (emailResult.isErr()) {
		return Err(emailResult.error.withContext('Invalid submission'));
	}

	const nameResult = validateRequired(request.name, 'Name');
	if (nameResult.isErr()) {
		return Err(nameResult.error.withContext('Invalid submission'));
	}

	const frontVideoResult = validateVideoFile(request.frontVideo, 'front video');
	if (frontVideoResult.isErr()) {
		return Err(frontVideoResult.error.withContext('Invalid submission'));
	}

	const sideVideoResult = validateVideoFile(request.sideVideo, 'side video');
	if (sideVideoResult.isErr()) {
		return Err(sideVideoResult.error.withContext('Invalid submission'));
	}

	onProgress?.(10);

	// Build form data
	const formData = new FormData();
	formData.append('uid', request.uid);
	formData.append('email', request.email);
	formData.append('name', request.name);
	formData.append('fileuploadfront', request.frontVideo);
	formData.append('fileuploadside', request.sideVideo);

	onProgress?.(20);

	// Create abort controller for timeout
	const controller = new AbortController();
	const timeoutId = setTimeout(() => controller.abort(), DEFAULT_TIMEOUT_MS);

	const result = await tryAsync(
		async () => {
			const response = await fetch(API_ENDPOINTS.contribute, {
				method: 'POST',
				body: formData,
				signal: controller.signal
			});

			onProgress?.(80);

			if (!response.ok) {
				const errorText = await response.text().catch(() => 'Unknown error');
				throw new Error(`Server error (${response.status}): ${errorText}`);
			}

			onProgress?.(100);
			return 'Your submission has been received! You will receive an email with your results shortly.';
		},
		'Failed to submit contribution'
	);

	clearTimeout(timeoutId);

	// Handle specific error cases
	if (result.isErr()) {
		const error = result.error;
		
		if (error.rootCause.includes('aborted') || error.rootCause.includes('abort')) {
			return Err(new AppError(
				'Request timed out. Your files might be too large or your connection is slow.'
			).withContext('Submission failed'));
		}

		if (error.rootCause.includes('NetworkError') || error.rootCause.includes('fetch')) {
			return Err(new AppError(
				'Network error. Please check your internet connection.'
			).withContext('Submission failed'));
		}
	}

	return result;
}

/**
 * Generic authenticated API request helper
 */
export async function authenticatedFetch<T>(
	url: string,
	options: RequestInit = {}
): Promise<Result<T, AppError>> {
	// Get auth token
	const tokenResult = await authStore.getIdToken();
	if (tokenResult.isErr()) {
		return tokenResult as unknown as Result<T, AppError>;
	}

	const headers = new Headers(options.headers);
	headers.set('Authorization', `Bearer ${tokenResult.value}`);

	return tryAsync(
		async () => {
			const response = await fetch(url, {
				...options,
				headers
			});

			if (!response.ok) {
				const errorText = await response.text().catch(() => 'Unknown error');
				throw new Error(`API error (${response.status}): ${errorText}`);
			}

			return response.json() as Promise<T>;
		},
		'API request failed'
	);
}
