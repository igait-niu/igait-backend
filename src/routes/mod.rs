//! This module contains the routes for the API.
//! 
//! To learn more about a route and how it works, click on the module name at the bottom of the page.
//! 
//! # Layout and Explanation
//! The API has three routes:
//! * `completion`: This route is for use by **Metis** to update the status of a job.
//! * `historical_submissions`: This route is for use by the iGait frontend to get the historical submissions of a user.
//! * `upload`: This route is for use by the iGait frontend to upload a job to the server.
//! 
//! In the lifecycle of a job, first, the patient information and files are uploaded to the server via the `upload` route.
//! Then, the job is processed by the server, and eventually shipped to **Metis**. 
//! Finally, the status of the job is updated by **Metis** via the `completion` route.
//! After the job is completed, the user can view the historical submissions via the `historical_submissions` route.
//! 
//! # Notes
//! * The API is currently versioned at `v1`, meaning every route is actually at `/api/v1/<route>`.
//! * The `completion` endpoint is only for use by **Metis**.
//! * The `upload` and `historical_submissions` endpoints are for use by the iGait frontend.

/// This module contains the completion endpoint for the API.
/// 
/// # Arguments
/// * `uid`: The Google Firebase user ID. Use the Firebase JS SDK to obtain before submitting.
/// * `job_id`: The ID of the job to update.
/// * `status_code`: The status code of the job. Either `OK` or `ERROR`.
/// * `status_content`: The content of the status. This is a human-readable string.
/// * `igait_access_key`: The access key to the iGait API.
/// 
/// # Example cURL Request
/// ```sh
/// curl -v -F user_id=curlplaceholder -F job_id=0 -F status_code=OK /
///     -F status_content="Job completed successfully!" -F igait_access_key /
///    http://api.igaitapp.com/api/v1/completion
/// ```
/// 
/// # Potential Reasons for Failure
/// See each individual function for potential reasons for failure.
/// 
/// # Notes
/// * Only for use by **Metis**. Do not use this endpoint for the iGait API.
/// * To find the API key, check the `.env` file on the AWS deployment.
pub mod completion;

/// This module contains the historical submissions endpoint for the API.
/// 
/// # Arguments
/// * `uid`: The Google Firebase user ID. Use the Firebase JS SDK to obtain before submitting.
/// 
/// # Example cURL Request
/// ```sh
/// curl -v -F user_id=curlplaceholder http://api.igaitapp.com/api/v1/historical_submissions
/// ```
/// 
/// # Potential Reasons for Failure
/// See each individual function for potential reasons for failure.
pub mod historical;

/// This module contains the job upload endpoint for the API.
/// 
/// # Arguments
/// * `uid`: The Google Firebase user ID. Use the Firebase JS SDK to obtain before submitting.
/// * `age`: The age of the user in `MM/DD/YYYY` format.
/// * `ethnicity`: The ethnicity of the user.
/// * `sex`: The sex assigned to the patient at birth.
/// * `height`: The height of the user in `ft'in"` format.
/// * `weight`: The weight of the user in pounds.
/// * `email`: The email address of the user of which to send emails to.
/// * `front_file`: The front video file. Needs to be a file compatible with OpenPose.
/// * `side_file`: The side video file. Needs to be a file compatible with OpenPose.
/// 
/// # Example cURL Request
/// ```sh
/// curl -v -F fileuploadfront=@test.mp4 -F fileuploadside=@test.mp4 /
///     -F uid=curlplaceholder -F age=18 -F ethnicity=Caucasian /
///     -F email=me@hiibolt.com -F sex=M -F height="5'10" -F weight=120 / 
///     http://api.igaitapp.com/api/v1/upload
/// ```
/// 
/// # Potential Reasons for Failure
/// See each individual function for potential reasons for failure.
/// 
/// # Notes
/// <div class="warning">This route takes a long time to process. Make users aware of this on the frontend.</div>
pub mod upload;