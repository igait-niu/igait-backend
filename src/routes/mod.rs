//! This module contains the routes for the API.
//! 
//! To learn more about a route and how it works, click on the module name at the bottom of the page.

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