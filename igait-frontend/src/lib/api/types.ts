/**
 * API response types
 */

import type { Option } from '$lib/result';

/**
 * Job status from the backend
 */
export interface JobStatus {
	readonly code: string;
	readonly value: string;
}

/**
 * Historical job data
 */
export interface Job {
	readonly timestamp: { secs_since_epoch: number };
	readonly status: JobStatus;
	readonly age: number;
	readonly height: number;
	readonly weight: number;
	readonly sex: string;
}

/**
 * Contribution/submission request
 */
export interface ContributionRequest {
	readonly uid: string;
	readonly email: string;
	readonly name: string;
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
