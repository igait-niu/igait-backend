/**
 * API response types
 */

// Re-export generated types from ts-rs
export type { Job } from '../../types/Job';
export type { JobStatus } from '../../types/JobStatus';
export type { Sex } from '../../types/Sex';
export type { Ethnicity } from '../../types/Ethnicity';
export type { UserRole } from '../../types/UserRole';
export type { User } from '../../types/User';

// Import types for use in this file
import type { Sex } from '../../types/Sex';
import type { Ethnicity } from '../../types/Ethnicity';
import type { UserRole } from '../../types/UserRole';
import type { Job } from '../../types/Job';

/**
 * Contribution/submission request - includes all demographic fields
 * required by the backend for gait analysis.
 */
export interface ContributionRequest {
	readonly uid: string;
	readonly email: string;
	readonly age: number;
	readonly sex: Sex;
	readonly ethnicity: Ethnicity;
	readonly heightFeet: number;
	readonly heightInches: number;
	readonly weight: number;
	readonly role: UserRole;
	readonly frontVideo: File;
	readonly sideVideo: File;
}

/**
 * Assistant WebSocket message types
 */
export type AssistantMessageType = 'Error' | 'Message' | 'Waiting' | 'You' | 'Typing' | 'Info' | 'Jobs';

export interface AssistantMessage {
	readonly type: AssistantMessageType;
	readonly content: string | Job[];
}

/**
 * Progress callback type
 */
export type ProgressCallback = (progress: number) => void;
