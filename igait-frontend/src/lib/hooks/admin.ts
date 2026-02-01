/**
 * Admin hooks for subscribing to queues and all jobs (admin only)
 */

import { getFirebaseDatabase } from '$lib/firebase';
import { ref, onValue, type Unsubscribe } from 'firebase/database';
import type { Job } from '../../types/Job';
import type { JobStatus } from '../../types/JobStatus';

/**
 * Queue item structure from Firebase RTDB
 */
export interface QueueItem {
	job_id: string;
	user_id: string;
	enqueued_at: number;
	claimed_by?: string;
	claimed_at?: number;
	input_keys: Record<string, string>;
	metadata: {
		age?: number;
		email?: string;
		ethnicity?: string;
		sex?: string;
		height?: string;
		weight?: number;
	};
}

/**
 * Finalize queue item (slightly different structure)
 */
export interface FinalizeQueueItem extends QueueItem {
	success: boolean;
	error?: string;
	error_logs?: string;
	failed_at_stage?: number;
}

/**
 * All queues data structure
 */
export interface QueuesData {
	stage_1: Record<string, QueueItem>;
	stage_2: Record<string, QueueItem>;
	stage_3: Record<string, QueueItem>;
	stage_4: Record<string, QueueItem>;
	stage_5: Record<string, QueueItem>;
	stage_6: Record<string, QueueItem>;
	finalize: Record<string, FinalizeQueueItem>;
}

/**
 * State of queues loading
 */
export type QueuesState =
	| { readonly status: 'loading' }
	| { readonly status: 'error'; readonly error: string }
	| { readonly status: 'loaded'; readonly queues: QueuesData };

/**
 * Subscribe to all queues in Firebase RTDB (admin only)
 */
export function subscribeToQueues(
	onUpdate: (state: QueuesState) => void
): Unsubscribe {
	const db = getFirebaseDatabase();
	const queuesRef = ref(db, 'queues');
	
	onUpdate({ status: 'loading' });
	
	const unsubscribe = onValue(
		queuesRef,
		(snapshot) => {
			const data = snapshot.val();
			
			const queues: QueuesData = {
				stage_1: data?.stage_1 ?? {},
				stage_2: data?.stage_2 ?? {},
				stage_3: data?.stage_3 ?? {},
				stage_4: data?.stage_4 ?? {},
				stage_5: data?.stage_5 ?? {},
				stage_6: data?.stage_6 ?? {},
				finalize: data?.finalize ?? {},
			};
			
			onUpdate({ status: 'loaded', queues });
		},
		(error) => {
			console.error('Error fetching queues:', error);
			onUpdate({ status: 'error', error: error.message });
		}
	);
	
	return unsubscribe;
}

/**
 * Job with user ID for admin view - extends Job with id field
 */
export type AdminJob = Job & { 
	id: string; // Full job ID: userId_jobIndex
};

/**
 * State of all jobs loading
 */
export interface AllJobsState {
	loading: boolean;
	jobs: AdminJob[];
	error?: string;
}

/**
 * Subscribe to all jobs across all users (admin only)
 */
export function subscribeToAllJobs(
	onUpdate: (state: AllJobsState) => void
): Unsubscribe {
	const db = getFirebaseDatabase();
	const usersRef = ref(db, 'users');
	
	onUpdate({ loading: true, jobs: [] });
	
	const unsubscribe = onValue(
		usersRef,
		(snapshot) => {
			const data = snapshot.val();
			
			if (!data) {
				onUpdate({ loading: false, jobs: [] });
				return;
			}
			
			const allJobs: AdminJob[] = [];
			
			// Iterate through all users
			for (const [userId, userData] of Object.entries(data)) {
				const user = userData as { jobs?: Job[]; administrator?: boolean };
				if (!user.jobs) continue;
				
				// Handle both array and object formats
				const jobs: Job[] = Array.isArray(user.jobs) ? user.jobs : Object.values(user.jobs);
				
				jobs.forEach((job, index) => {
					if (!job || !job.email) return;
					if (job.email === 'placeholder@placeholder.com') return;
					
					allJobs.push({
						...job,
						id: `${userId}_${index}`,
					});
				});
			}
			
			// Sort by timestamp (newest first)
			allJobs.sort((a, b) => b.timestamp - a.timestamp);
			
			onUpdate({ loading: false, jobs: allJobs });
		},
		(error) => {
			console.error('Error fetching all jobs:', error);
			onUpdate({ loading: false, jobs: [], error: error.message });
		}
	);
	
	return unsubscribe;
}

// Helper type guards for queues
export function isQueuesLoading(state: QueuesState): state is { status: 'loading' } {
	return state.status === 'loading';
}

export function isQueuesError(state: QueuesState): state is { status: 'error'; error: string } {
	return state.status === 'error';
}

export function isQueuesLoaded(state: QueuesState): state is { status: 'loaded'; queues: QueuesData } {
	return state.status === 'loaded';
}
