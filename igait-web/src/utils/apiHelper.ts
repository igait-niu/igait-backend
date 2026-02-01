/**
 * Enhanced API submission handler with full CORS support
 * The backend supports CORS, so we can read response status and errors!
 */

export interface SubmissionResult {
  success: boolean;
  message: string;
  statusCode?: number;
  error?: Error;
}

export interface SubmissionOptions {
  endpoint: string;
  formData: FormData;
  timeoutMs?: number;
  onProgress?: (progress: number) => void;
}

/**
 * Submit form data with proper error handling
 * 
 * @note The API supports CORS (access-control-allow-origin: *), 
 * so we can read the full response including status codes and error messages!
 */
export async function submitWithCors({
  endpoint,
  formData,
  timeoutMs = 60000, // 60 second timeout for large video files
  onProgress
}: SubmissionOptions): Promise<SubmissionResult> {
  
  // Pre-flight validation
  const validationError = validateFormData(formData);
  if (validationError) {
    return {
      success: false,
      message: validationError
    };
  }

  if (onProgress) onProgress(10);

  try {
    // Create timeout promise
    const timeoutPromise = new Promise<never>((_, reject) => {
      setTimeout(() => {
        reject(new Error('Request timeout - the files might be too large or your connection is slow'));
      }, timeoutMs);
    });

    if (onProgress) onProgress(20);

    // Make the request WITHOUT no-cors since the API supports CORS!
    const fetchPromise = fetch(endpoint, {
      method: 'POST',
      body: formData,
    });

    if (onProgress) onProgress(50);

    // Race between fetch and timeout
    const response = await Promise.race([fetchPromise, timeoutPromise]);

    if (onProgress) onProgress(80);

    // Now we can actually read the response!
    if (response.ok) {
      if (onProgress) onProgress(100);
      
      return {
        success: true,
        message: 'Success! Your submission has been received. You will receive an email with your results shortly.',
        statusCode: response.status
      };
    } else {
      // Server returned an error status
      let errorMessage = `Server error (${response.status})`;
      
      try {
        const errorData = await response.text();
        if (errorData) {
          errorMessage = errorData;
        }
      } catch {
        // Couldn't parse error message, use default
      }

      return {
        success: false,
        message: `Upload failed: ${errorMessage}. Please try again or contact support if the problem persists.`,
        statusCode: response.status
      };
    }

  } catch (error) {
    console.error('Submission error:', error);

    // Network-level errors
    if (error instanceof TypeError) {
      return {
        success: false,
        message: 'Network error: Please check your internet connection and try again.',
        error: error as Error
      };
    }

    if (error instanceof Error && error.message.includes('timeout')) {
      return {
        success: false,
        message: 'Request timed out. Your files might be too large. Please try compressing your videos or check your connection.',
        error
      };
    }

    // Unknown error
    return {
      success: false,
      message: 'An unexpected error occurred. Please try again or contact support if the problem persists.',
      error: error as Error
    };
  }
}

/**
 * Validate form data before submission
 */
function validateFormData(formData: FormData): string | null {
  const maxVideoSize = 500 * 1024 * 1024; // 500MB max per video
  const validVideoExtensions = ['.mp4', '.mov', '.avi', '.mkv', '.webm', '.m4v', '.wmv', '.flv'];
  
  for (const [key, value] of formData.entries()) {
    // Check for empty required fields
    if (!value || (typeof value === 'string' && value.trim() === '')) {
      return `Missing required field: ${key}`;
    }

    // Check file sizes
    if (value instanceof File) {
      if (value.size === 0) {
        return `The file "${value.name}" is empty. Please select a valid video file.`;
      }
      
      if (value.size > maxVideoSize) {
        const sizeMB = (value.size / (1024 * 1024)).toFixed(1);
        return `The file "${value.name}" is too large (${sizeMB}MB). Maximum size is 500MB. Please compress the video.`;
      }

      // Check file type - use both MIME type and extension
      const hasValidMimeType = value.type.startsWith('video/');
      const fileName = value.name.toLowerCase();
      const hasValidExtension = validVideoExtensions.some(ext => fileName.endsWith(ext));
      
      if (!hasValidMimeType && !hasValidExtension) {
        return `The file "${value.name}" doesn't appear to be a video. Please upload video files only (MP4, MOV, AVI, etc.).`;
      }
    }
  }

  return null;
}

/**
 * Retry logic for failed submissions
 */
export async function submitWithRetry(
  options: SubmissionOptions,
  maxRetries: number = 2
): Promise<SubmissionResult> {
  let lastError: SubmissionResult | null = null;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    if (attempt > 0) {
      // Exponential backoff: 2s, 4s, 8s...
      const delay = Math.min(1000 * Math.pow(2, attempt), 8000);
      await new Promise(resolve => setTimeout(resolve, delay));
    }

    const result = await submitWithCors(options);
    
    if (result.success) {
      return result;
    }

    lastError = result;

    // Don't retry validation errors or client errors (4xx)
    if (result.message.includes('Missing required field') || 
        result.message.includes('too large') ||
        result.message.includes('empty') ||
        (result.statusCode && result.statusCode >= 400 && result.statusCode < 500)) {
      break;
    }
  }

  return lastError || {
    success: false,
    message: 'Failed after multiple attempts. Please try again later.'
  };
}
