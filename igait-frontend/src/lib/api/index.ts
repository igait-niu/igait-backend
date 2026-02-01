/**
 * Central API exports for the iGait application
 */

// Configuration
export { API_BASE_URL, API_ENDPOINTS, DEFAULT_TIMEOUT_MS, MAX_VIDEO_SIZE_BYTES, MAX_VIDEO_SIZE_MB, VALID_VIDEO_EXTENSIONS } from './config';

// Types
export type { Job, JobStatus, ContributionRequest, AssistantMessage, AssistantMessageType, ProgressCallback } from './types';

// Validation
export { validateEmail, validateRequired, validateVideoFile, validatePassword, validatePasswordMatch } from './validation';

// Client
export { submitContribution, authenticatedFetch } from './client';
