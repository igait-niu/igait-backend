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
 * Validate all submission fields
 */
function validateSubmission(request: ContributionRequest): Result<void, AppError> {
	// Validate email
	const emailResult = validateEmail(request.email);
	if (emailResult.isErr()) {
		return Err(emailResult.error.withContext('Invalid submission'));
	}

	// Validate age
	if (request.age < 1 || request.age > 115) {
		return Err(new AppError('Age must be between 1 and 115').withContext('Invalid submission'));
	}

	// Validate weight
	if (request.weight < 1 || request.weight > 500) {
		return Err(new AppError('Weight must be between 1 and 500 lbs').withContext('Invalid submission'));
	}

	// Validate height
	if (request.heightFeet < 1 || request.heightFeet > 8) {
		return Err(new AppError('Height (feet) must be between 1 and 8').withContext('Invalid submission'));
	}
	if (request.heightInches < 0 || request.heightInches > 11) {
		return Err(new AppError('Height (inches) must be between 0 and 11').withContext('Invalid submission'));
	}

	// Validate videos
	const frontVideoResult = validateVideoFile(request.frontVideo, 'front video');
	if (frontVideoResult.isErr()) {
		return Err(frontVideoResult.error.withContext('Invalid submission'));
	}

	const sideVideoResult = validateVideoFile(request.sideVideo, 'side video');
	if (sideVideoResult.isErr()) {
		return Err(sideVideoResult.error.withContext('Invalid submission'));
	}

	return Ok(undefined);
}

/**
 * Submit a gait analysis contribution
 */
export async function submitContribution(
	request: ContributionRequest,
	onProgress?: ProgressCallback
): Promise<Result<string, AppError>> {
	// Validate all fields first
	const validationResult = validateSubmission(request);
	if (validationResult.isErr()) {
		return validationResult as unknown as Result<string, AppError>;
	}

	onProgress?.(5);

	// Build form data matching backend UploadRequestArguments
	const formData = new FormData();
	formData.append('uid', request.uid);
	formData.append('age', request.age.toString());
	formData.append('ethnicity', request.ethnicity);
	formData.append('sex', request.sex);
	formData.append('height', `${request.heightFeet}'${request.heightInches}`);
	formData.append('weight', request.weight.toString());
	formData.append('email', request.email);
	formData.append('fileuploadfront', request.frontVideo);
	formData.append('fileuploadside', request.sideVideo);

	onProgress?.(15);

	// Create abort controller for timeout
	const controller = new AbortController();
	const timeoutId = setTimeout(() => controller.abort(), DEFAULT_TIMEOUT_MS);

	const result = await tryAsync(
		async () => {
			const response = await fetch(API_ENDPOINTS.upload, {
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
