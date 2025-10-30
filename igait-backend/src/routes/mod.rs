//! This module contains the routes for the API.
//! 
//! To learn more about a route and how it works, click on the module name at the bottom of the page.

/// This module contains the pipeline submission endpoint for the API.
/// 
/// This endpoint receives results from the iGait pipeline running on the compute cluster.
/// It is secured with a shared secret to ensure only authorized pipeline instances can submit.
/// 
/// # Arguments
/// * `X-Pipeline-Secret` header: The shared secret for authentication
/// * `output`: JSON-serialized Output struct (multipart field)
/// * `archive`: Optional results.zip file (multipart field)
/// 
/// # Example cURL Request
/// ```sh
/// curl -v -X POST \
///     -H "X-Pipeline-Secret: MEOWMEOWMEOW" \
///     -F output=@output.json \
///     -F archive=@results.zip \
///     http://api.igaitapp.com/api/v1/pipeline/submit
/// ```
/// 
/// # Testing
/// For local testing instructions, see the [testing guide](../docs/testing.md).
/// 
/// # Potential Reasons for Failure
/// * `401 Unauthorized` - Invalid or missing secret
/// * `400 Bad Request` - Malformed data
/// * `500 Internal Server Error` - Processing failed
pub mod pipeline;

/// This module contains the historical submissions endpoint for the API.
/// 
/// # Arguments
/// * `uid`: The Google Firebase user ID. Use the Firebase JS SDK to obtain before submitting.
/// * `entries`: The number of entries to return. This is an unsigned integer.
/// * `start_timestamp`: The UNIX timestamp (seconds since the epoch date) to start the search from.
/// * `end_timestamp`: The UNIX timestamp (seconds since the epoch date) to end the search from.
/// * `result_type`: The type of result to return. This is a string - either `ASD` or `NO ASD`.`
/// * `include_original`: Whether to include the original video files in the response.
/// * `include_skeleton`: Whether to include the skeleton files in the response.
/// 
/// # Example cURL Request
/// ```sh
/// curl -v -F user_id=hiibolt -F entries=2 -F start_timestamp=173
/// 6628724 -F end_timestamp=3736628726 -F result_type='NO ASD' -F inc
/// lude_skeleton=true -F include_original=true http://api.igaitapp.com/
/// api/v1/historical_submissions
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

/// This module contains the assistant endpoint for the API.
/// 
/// Currently, this works by proxying requests to the iGait Assistant API.
/// 
/// This is done so due to the limitations of Firebase Auth expecting authentication via the `Bearer` header.
/// Unfortunately, the WSS standard does not support this, so we instead extract it from an initial request and re-proxy to a now-authenticaed secure websocket.
/// This approach is certainly not ideal, but it is indeed highly secure. 
/// In fact, it is significantly more secure than the WSS standard itself, which does not support authentication at all.
/// 
/// For more information on usage, see the iGAIT frontend codebase.
pub mod assistant;

/// This module contains the contribute endpoint for the API,
/// which is used to submit a video for our research purposes.
/// 
/// # Arguments
/// * `uid`: The Google Firebase user ID. Use the Firebase JS SDK to obtain before submitting.
/// * `email`: The email address of the user of which to send emails to.
/// * `fileuploadfront`: The front video file. Needs to be a file compatible with OpenPose.
/// * `fileuploadside`: The side video file. Needs to be a file compatible with OpenPose.
/// * `name`: The name of the user submitting the video.
/// 
/// # Example cURL Request
/// ```sh
/// curl -v -F fileuploadfront=@test.mp4 -F fileuploadside=test.mp4 /
///     -F uid=curlplaceholder -F email=me@hiibolt.com -F name="John Doe" /
///     http://api.igaitapp.com/api/v1/contribute
/// ```
/// 
/// # Potential Reasons for Failure
/// See each individual function for potential reasons for failure.
pub mod contribute;