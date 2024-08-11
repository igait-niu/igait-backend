# iGait ASD - Backend
This is the primary brain behind everything in the iGait app. There are a variety of microservices involved with the submitssion, upload, storage, and more - this server handles all of this to bring our product to the convenience of a low-power mobile device.

# Other Repositories and Documents
Helpful links and documentation for other sources is also in this documentation.
### Email
- [Email Documentation](helper::email)
- [`igait-email` GitHub Repository](https://github.com/igait-niu/igait-email)
- [Cloudflare Worker](https://dash.cloudflare.com/4e56fe676d50c0b04e4593542eb465f2/workers/services/view/email-service/production?versionFilter=all)
### Metis
- [How to Login](https://www.niu.edu/crcd/current-users/getting-started/login-to-metis.shtml)
- [(NIU's) Documentation on Metis](https://www.niu.edu/crcd/current-users/getting-started/queue-commands-job-management.shtml)
- [(My) Documentation on Metis](helper::metis)

# Table of Contents
- [1](#1---api) - API
  - [1.1](#11---layout-and-explanation) - Layout and Explanation
  - [1.2](#12---notes) - Notes
- [2](#2---codebase-structure) - Codebase Structure
- [3](#3---setting-up-your-development-environment) - Setting Up Your Development Environment
  - [3.1](#31---installation) - Installation
    - [3.1.1](#311---nix-recommended) - Nix (recommended)
    - [3.1.2](#312---individual-installation-alternative-to-nix) - Individual Installation (alternative to Nix)
  - [3.2](#32---secrets) - Secrets
  - [3.3](#33---download-the-igait-backend-repository) - Download the `igait-backend` Repository
  - [3.4](#34---development-and-deployment-command-list) - Development and Deployment Command List
    - [3.4.1](#341---running-the-backend-locally) - Running the Backend Locally
    - [3.4.2](#342---building-and-running-release-binary-locally) - Building and Running Release Binary Locally
    - [3.4.3](#343---docker-build-and-run) - Docker Build and Run
    - [3.4.4](#344---nix-build-and-run) - Nix Build and Run
    - [3.4.5](#345---aws-pull-and-deploy-from-ghcr) - AWS Pull and Deploy From GHCR
    - [3.4.6](#346---aws-startup-from-stopped) - AWS Startup from Stopped
    - [3.4.7](#347---rust-docs-build) - Rust Docs Build
- [4](#4---ground-up-explanation-of-backend-service-selection) - Ground-up Explanation of Backend Service Selection
  - [4.0](#40---about) - About
  - [4.1](#41---how-to-extract-gait-parameters) - How to Extract Gait Parameters
  - [4.2](#42---consumer-devices-are-often-low-in-computation-power) - Consumer Devices are Often Low in Computation Power
  - [4.3](#43---openpose-uses-the-entire-gpu) - OpenPose Uses the Entire GPU
  - [4.4](#44---how-to-store-client-data-for-later-retrieval) - How to Store Client Data for Later Retrieval
  - [4.5](#45---growing-number-of-programs-can-be-easy-to-break) - Growing Number of Programs can be Easy to Break
  - [4.6](#46---how-to-securely-allow-access-of-historical-results) - How to Securely Allow Access of Historical Results
  - [4.7](#47---aws-ec2--gpu-is-extremely-costly) - AWS EC2 + GPU is Extremely Costly 
- [5](#5---implementation) - Implementation
  - [5.1](#51---email) - Email
  - [5.2](#52---job-statuses) - Job Statuses
  - [5.3](#53---database) - Database
    - [5.3.1](#531---database-structure) - Database Structure
    - [5.3.2](#532---database-wrapper-functions) - Database Wrapper Functions
  - [5.4](#54---server-state) - Server State
  - [5.5](#55---metis) - Metis
    - [5.5.1](#551---automating-ssh-job-creation) - Automating SSH Job Creation
    - [5.5.2](#552---pbs-script) - PBS Script

# 1 - API
### 1.1 - Layout and Explanation
The API has three routes:
* [`api/v1/completion`](routes::completion)
* [`api/v1/historical_submissions`](routes::historical)
* [`api/v1/upload`](routes::upload)

In the lifecycle of a job, first, the patient information and files are uploaded to the server via the `upload` route.
Then, the job is processed by the server, and eventually shipped to **Metis**. 
Finally, the status of the job is updated by **Metis** via the `completion` route. When the status is finalized by the backend server, emails are sent to the owner of the job via Cloudflare Workers by the backend.
After the job is completed, the user can view the historical submissions via the `historical_submissions` route.

To learn more about why the API is designed how it is, or more about how it works, skip to the [API section](#).

To see more information about a specific route, see [the routes module](routes).
### 1.2 - Notes
* The API is currently versioned at `v1`, meaning every route is actually at `/api/v1/<route>`.
* The `completion` endpoint is only for use by **Metis**.
* The `upload` and `historical_submissions` endpoints are for use by the iGait frontend.

# 2 - Codebase Structure
<img src="https://github.com/user-attachments/assets/3eaebabc-ac73-4041-a866-c7221923f94a" width=750></img>

The codebase is split up into multiple modules:

- [`daemons`]:
  - [`daemons/filesystem.rs`](daemons::filesystem): Daemon which fires Metis inference requests when a new file is detected
- [`helper`]:
  - [`helper/database.rs`](helper::database): Handles the interfacing with Google Firebase
  - [`helper/email.rs`](helper::email): Handles the interfacing with Cloudflare Workers to send email
  - [`helper/lib.rs`](helper::lib): Defines all custom datatypes the iGait backend uses 
  - [`helper/metis.rs`](helper::metis): Handles the interfacing with the Metis supercomputer
  - [`helper/print.rs`](helper::print): Helper printing macros to create easily readable print messages
- [`routes`]: 
  - [`routes/completion.rs`](routes::completion): This route is for use by **Metis** to update the status of a job
  - [`routes/historical.rs`](routes::historical): This route is for use by the **iGait frontend** to get the historical submissions of a user.
  - [`routes/upload.rs`](routes::upload): This route is for use by the **iGait frontend** to upload a job to the server.

# 3 - Setting Up Your Development Environment
There are two ways to ensure that you have the proper development environment.

You can either individually install each dependancy, or you can use the [Nix package manager](https://nixos.org/) to instead only run a single command to download [my](https://github.com/hiibolt) exact environment.
### 3.1 - Installation
#### 3.1.1 - Nix (recommended)
- Install [Nix](https://nixos.org/)
- Enable Nix Flakes (the process will likely have changed/integrated into standard by the time someone reads this, so you will need to read the [Nix documentation](https://nixos.wiki/wiki/Flakes))
- Run `nix develop`
#### 3.1.2 - Individual Installation (alternative to Nix)
Do the following:
- Install [Docker](https://www.docker.com/)
- Install [Rust](https://www.rust-lang.org/)
- Install [GCC](https://gcc.gnu.org/)
- Install the following packages via your package manager of choice:
  - `pkg-config`
  - `openssl`
  - `openssh`
  - `curl`

Recommended (but optional!) Extensions and Packages:
- `rustfmt`
- `clippy`
- `rust-analyzer`
### 3.2 - Secrets
To run the backend, you will need to set a few environment variables:
- `AWS_ACCESS_KEY_ID`: Found via the AWS Console
- `AWS_SECRET_ACCESS_KEY`: Found via the AWS Console
- `FIREBASE_ACCESS_KEY`: Found in the Google Firebase API settings
- `IGAIT_ACCESS_KEY`: This is an arbitrary value, what is important is that it is set to the same value for both the **Metis** scripts and the backend. This is because this API key is what secures the [`completion`](routes::completion) endpoint.
### 3.3 - Download the `igait-backend` Repository
Next, clone the repository:
```bash
$ git clone https://github.com/igait-niu/igait-backend.git
$ cd igait-backend
```
### 3.4 - Development and Deployment Command List
A typical development process should be in the following order:
- 1.) Implement changes
- 2.) Test a basic run with the [run command](#3.4.1)
- 3.) Test a Docker build with the [associated commands](#3.4.3)

A typical deployment process should be in the following order:
- 1.) Commit changes to the `master` branch of the [GitHub repository](https://github.com/igait-niu/igait-backend)
- 2.) Wait for GitHub Actions to build and publish the image automatically
- 3.) Pull and launch the new Docker Image with the [associated commands](#3.4.5)

To test production speed locally to gauge performance, use the [release build commands](#3.4.2).

To diagnose an error where it builds locally but not on GitHub Actions, use the [Nix release build command](#3.4.4).
#### <a name="3.4.1">3.4.1</a> - Running the Backend Locally
```bash
$ cargo run
```
#### <a name="3.4.2">3.4.2</a> - Building and Running Release Binary Locally
```bash
$ cargo build --release
$ ./target/release/igait-backend
```
#### <a name="3.4.3">3.4.3</a> - Docker Build and Run
```bash
$ docker build -t testing .
$ docker run testing
$ docker ps # find the name of the Docker container
$ docker kill <the name of the container>
```
#### <a name="3.4.4">3.4.4</a> - Nix Build and Run
```bash
$ nix build .#igait-backend
```
#### <a name="3.4.5">3.4.5</a> - AWS Pull and Deploy From GHCR
```bash
$ cd igait-backend
$ docker compose down
$ docker pull ghcr.io/igait-niu/igait-backend:latest
$ docker compose up -d
```
#### <a name="3.4.6">3.4.6</a> - AWS Startup from Stopped
```bash
$ cd igait-backend
$ docker compose up -d
```
#### <a name="3.4.7">3.4.7</a> - Rust Docs Build
```bash
$ cargo doc --no-deps
$ rm -rf ./docs
$ echo "<meta http-equiv=\"refresh\" content=\"0; url=igait_backend/index.html\">" > target/doc/index.html
$ cp -r target/doc ./docs
```
# 4 - Ground-up Explanation of Backend Service Selection
### 4.0 - About
This section is quite extensive, and will generally follow a problem-solution style explanation. It's important to note that exact function signatures, datatypes, or other parts may be altered in the future.

However, it is strongly recommended for any incoming maintainers or developers to read this in order to understand the decisions we made, and why.
### 4.1 - How to Extract Gait Parameters
Firstly, a [pose estimation](https://en.wikipedia.org/wiki/Pose_(computer_vision)) must be created. This allows us to serialize a person’s current bodily position. Accordingly, by analyzing the rate of change between each position for each joint, we can train a model to scan for abnormalities in [walking gait](https://www.kenhub.com/en/library/anatomy/gait-cycle).  

To accomplish this, we employed Carnegie Mellon’s [OpenPose](https://github.com/CMU-Perceptual-Computing-Lab/openpose).  

![image](https://github.com/CMU-Perceptual-Computing-Lab/openpose/raw/master/.github/media/pose_face_hands.gif)
### 4.2 - Consumer Devices are Often Low in Computation Power
Since mobile devices do not always have the computational power to run a dense machine learning model, we must create a machine that can run these jobs *for* the client. 

By creating a backend that could accept job submissions, perform computations, and return a score, we can effectively handle these intensive computations on otherwise weak devices.  

![image](https://github.com/hiibolt/hiibolt/assets/91273156/11647ab4-2339-49f4-95d7-452af2f5cbf2)  
### 4.3 - OpenPose Uses the Entire GPU
OpenPose is computationally intensive and makes usage of the *entire GPU* while running. This creates parallelization issues almost immediately. To understand why, let us consider the following scenario. We have two people who submit a job request within a brief period of time to our theoretical backend.

**Person 1:** Submits a job, which the server accepts and gets to work on, putting the GPU at 100% usage.  

**Person 2:** Submits a job, which the server accepts, but immediately errors! OpenPose could not access the GPU as it is currently in use!  

![image](https://github.com/hiibolt/hiibolt/assets/91273156/5197cd12-137c-4ca6-b489-cad61869e76d)  

This means that only one job can run at a time. To combat this, we must create our own **queue system**. This means the server should have a list of jobs, running them one at a time. Instead of trying to immediately run incoming submissions, as most API requests are handled, we assume an extended period. Instead of receiving a result on submission, we receive **200 OK**, signifying that while the backend has received your job submission, a result is not ready – check back later.  

![image](https://github.com/hiibolt/hiibolt/assets/91273156/d5196814-377d-4f80-b2aa-f6e32c1358dd)
### 4.4 - How to Store Client Data for Later Retrieval
This diagram is a great first step to a time-concerned backend system. However, there are additional constraints that were next introduced. Firstly, all entry results and associated data had to be *stored for later retrieval*. This meant we needed a database. 

For this, we selected [Firebase Realtime DB](https://firebase.google.com/products/realtime-database/). 

Secondly, we needed to store our user’s videos. This is more complicated than a standard database – these videos can be multiple hundred megabytes. 

Accordingly, we chose [AWS S3](https://aws.amazon.com/s3/) – since we planned to use AWS EC2 instances to host our backend, it made sense to also use S3.  

![image](https://github.com/hiibolt/hiibolt/assets/91273156/c6f27a3e-6a9e-44d4-a956-a4db4cc80b23)  
### 4.5 - Growing Number of Programs can be Easy to Break
At this point, there are three programs running on AWS. 
- Frontend (React)
- Backend API (Rust)
- OpenPose

The issue with this is that there are three programs with seperate and potentially conflicting dependancies. It can become extremely difficult to keep track of setup, deployment steps, and more with this many technologies.

[Docker](https://www.docker.com/) became our immediate solution. Docker allows you to keep messy programs defined in a zero-config file which builds an image which always runs, every time. This also means we can more directly isolate each section of our application from eachother - note that each container below is instead only able to communicate via API or port number.

![image](https://github.com/hiibolt/hiibolt/assets/91273156/4817a850-1d4f-4b8b-bdd1-71227e7083aa)  
### 4.6 - How to Securely Allow Access of Historical Results
The above diagram is already very secure. However, there is one flaw – Realtime DB must be pseudo-public to be able to re-access results!

To combat this, we deliver results by email instead. There are many email services, but we opted to use Cloudflare Workers, as it allows us to easily send emails from our Cloudflare-protected domain. We still use Firebase DB behind the scenes, but it can now be made private, as seen below.

![image](https://github.com/hiibolt/hiibolt/assets/91273156/4be2c6ac-35b7-4621-8e11-18cd24e38ea2)  
### 4.7 - AWS EC2 + GPU is Extremely Costly
AWS EC2 with GPU acceleration is **not** cheap. Hundreds, and even thousands, of dollars per month.

Thankfully, NIU’s CRCD (Center for Research Computing and Data) created [Metis](https://www.niu.edu/crcd/index.shtml) - an absolute powerhouse of computation.

Accordingly, we now let AWS EC2 handle job submission requests, and let Metis handle the heavy lifting. To learn more about how this communication occurs, see the [`metis module`](helper::metis).

This resulting final model is incredibly performant, secure, and HIPAA compliant. 

![image](https://github.com/hiibolt/hiibolt/assets/91273156/cc1884fa-e1dd-4c93-b77c-8666ef8b8c7c)  
# 5 - Implementation
### 5.1 - Email
We needed a way to send email. To do so, we selected [Cloudflare Workers](https://workers.cloudflare.com/). If you want to see an example of the code deployed to our Worker, you can see my article on setting it up [here](https://hiibolt.com/nodejs/npm/cloudflare/2024/04/16/emails-cloudflare.html).

Implementation is extremely simple - fire a POST request to the Cloudflare Worker, which does the rest of the work behind the scenes. Since we are also using Cloudflare DNS and Origin Server Certificates for HTTPS, we can easily implement emails to come from our domain. This ensures that customers understand it is us by an easily recognizable and legitimate email.

```rust
...
pub fn send_email (
    to:      &str,
    subject: &str,
    body:    &str,
    task_number: JobTaskID
) -> Result<()> {
    ...
}
```

If an email bounces, the Cloudflare Worker is down, or the email fails to send, the server can react accordingly thanks to the returned [`Result`] type.
### 5.2 - Job Statuses
We must be able to track what stage of completion each job is currently. To do so, we shall explictly define every possible status in advance.

Some statuses should also be able to contain additional information, such as error and completion types - which would hold error reasons or completion scores respectfully.
```rust
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum JobStatusCode {
    Submitting,
    SubmissionErr,
    Queue,
    Processing,
    InferenceErr,
    Complete
}
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct JobStatus {
    pub code: JobStatusCode,
    pub value: String,
}
```
### 5.3 - Database
#### 5.3.1 - Database Structure
We use Google Firebase Realtime DB to handle our data.

The Rust crate for handling database calls expects strictly typed datatypes that are consistent across all users.

Accordingly, we must define what a [`User`](helper::lib::User) is to look like, and what their [`Job`](helper::lib::Job)s must look like.

Note that each [`Job`](helper::lib::Job) also has its [`JobStatus`](helper::lib::JobStatus) that we previously defined.
```rust
#[derive( Serialize, Deserialize, Debug )]
pub struct User {
    pub uid: String,
    pub jobs: Vec<Job>
}
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct Job {
    pub age: i16,
    pub ethnicity: String,
    pub sex: char,
    pub height: String,
    pub status: Status,
    pub timestamp: SystemTime,
    pub weight: i16,
    pub email: String
}
```
#### 5.3.2 - Database Wrapper Functions
The Firebase crate is somewhat complex, and has much utility we can abstract away specifically for our usecase.

To do so, we will create a struct that contains only the Firebase data to be operated on, and make that field private.

This means the only way to modify that data is through our helper functions, which we define in order to manipulate the database.
```rust
#[derive( Debug )]
pub struct Database {
    _state: Firebase
}
impl Database {
    pub async fn init () -> Self {
        ...
    }
    pub async fn count_jobs ( &self, uid: String ) -> usize {
        ...
    }
    pub async fn new_job ( &self, uid: String, job: Job) {
        ...
    }
    pub async fn update_status ( &self, uid: String, job_id: usize, status: Status) {
        ...
    }
    pub async fn get_status ( &self, uid: String, job_id: usize) -> Option<Status> {
        ...
    }
    pub async fn get_job ( &self, uid: String, job_id: usize) -> Option<Job> {
        ...
    }
}
```
### 5.4 - Server State
We now have two handles - one to Firebase Realtime DB, `Database`; and another (not previously mentioned) to S3, `Bucket`.

When a job is submitted, the submitted files are downloaded to our AWS EC2 and placed in a folder. The existance of this folder indicates to the server that it has not yet been handled.

These two handles and the folder containing data comprise the entire state of our app. To combine the three, we create an `AppState` object which holds both handles, and a `work_queue` function. The [`filesystem daemon`](daemons::filesystem) repeatedly checks the folder for new job submissions, and updates the app state accordingly.

```rust
#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub bucket: Bucket
}
```

Well, it is important to consider that each incoming request is handled asynchronously. In order to share data across multiple threads, you *need* to protect it. Otherwise, if **Thread 1** and **Thread 2** try to write data to the state at the same time, a [data race](https://en.wikipedia.org/wiki/Race_condition) can occur, which can cause a multitude of images.

To combat this, [`Mutex`] is used. This will force other threads to wait until the currently accessing thread is done working.

Now, Rust cares a lot about [lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html), especially for pointers. However, if multiple things [own](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) a pointer, Rust refuses to compile.

This is because by default, in Rust, shared references cannot be mutable, for the same data race concern. To solve this, similarly, we create an [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html) which allows multiple pointers to the same thing, atomically (thread-safe).

So to recap, `Arc<Mutex<AppState>>` is a thread-safe pointer that can be copied to as many places as we need which allows access to the two main microservices, **S3** and **Firebase**.
### 5.5 - Metis
#### 5.5.1 - Automating SSH Job Creation
Because Metis is an NIU-only server, access is closed, and no web servers may be hosted on it. 

This makes sense, as it is computationally optimized, and NIU offers web server solutions seperately. 

Accordingly, to submit work to Metis, we must use SSH and run a [PBS Professional](https://altair.com/pbs-professional/) script. However, it is possible to automate this normally manual process! 
```rust
...
pub async fn query_metis (
    uid:         &str,
    job_id:      usize,
    task_number: JobTaskID
) -> Result<()> {
    ...
}
```
#### 5.5.2 - PBS Script
Metis can't recieve files by hosting a file drop endpoint. Instead we provide the job and user ID by launch arguments to the job script.

By doing so, within the PBS script, we can download the files to then perform work on them. 

It is worth noting that since we can have any number of different file extensions, a JSON file keeps track of what Metis needs to download from S3.

```bash
# Import files
/.../.venv/bin/python /.../download_files.py "$USER_ID" "$JOB_ID" ... ... ...
ls ./queue

# Get the video dir
VIDEO_DIR="$TMPDIR/queue"

# Find the front and side videos and extract their extensions
FRONT_VIDEO=$(find "$VIDEO_DIR" -type f -iname '*front*' | head -n 1)
SIDE_VIDEO=$(find "$VIDEO_DIR" -type f -iname '*side*' | head -n 1)

# Extract the extensions
FRONT_EXT="${FRONT_VIDEO##*.}"
SIDE_EXT="${SIDE_VIDEO##*.}"

printf "EXTENSIONS: $FRONT_EXT and $SIDE_EXT"
```

Because OpenPose uses so many libraries and programs to run, rather than bug the sysadmin to install them, we used Docker. 

There are some caveats, mainly that OpenPose needs access to the GPU. To make this happen, and to be able to run Docker without `sudo`, we employed [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) and [Podman](https://podman.io/).

Since the files are on our host operating system and not yet in our Docker container, we must also copy them there.

```bash
# Start Openpose
printf "[ :3 - Starting OpenPose GPU container... - :3 ]\n"
/bin/podman run --name openpose -t -d --device nvidia.com/gpu=all --security-opt=label=disable ghcr.io/hiibolt/igait-openpose
printf "[ :3 - Started OpenPose GPU container! - :3 ]\n\n"

# Build file structure
printf "[ :3 - Building file structure in OpenPose container... - :3 ]\n"
/bin/podman exec openpose mkdir /inputs
/bin/podman exec openpose mkdir /outputs
/bin/podman exec openpose mkdir /outputs/videos
/bin/podman exec openpose mkdir /outputs/json
printf "[ :3 - Build file structure in OpenPose container! - :3 ]\n\n"

# Import video files
printf "[ :3 - Importing video file inputs to OpenPose container... - :3 ]\n"
/bin/podman cp $VIDEO_DIR/front.$FRONT_EXT openpose:/inputs/front.$FRONT_EXT
/bin/podman cp $VIDEO_DIR/side.$FRONT_EXT openpose:/inputs/side.$FRONT_EXT
/bin/podman exec openpose ls /inputs
printf "[ :3 - Imported video file inputs to OpenPose container! - :3 ]\n\n"
```

Finally, we run the pose estimation. 

After the video overlays and JSON serializations of the pose estimation are completed, we pull them back out of the Docker container, and upload them to S3. 

```bash
# Run OpenPose on video files
printf "[ :3 - Starting OpenPose pose estimation... - :3 ]\n"
/bin/podman exec openpose ./build/examples/openpose/openpose.bin --video /inputs/front.$FRONT_EXT --display 0 --write_video /outputs/videos/front.$FRONT_EXT --write_json /outputs/json/front
/bin/podman exec openpose ./build/examples/openpose/openpose.bin --video /inputs/side.$SIDE_EXT --display 0 --write_video /outputs/videos/side.$SIDE_EXT --write_json /outputs/json/side
printf "[ :3 - Finished OpenPose pose estimations! - :3 ]\n\n"

# Move output to host filesystem
printf "[ :3 - Copying outputs... - :3 ]\n"
/bin/podman cp openpose:/outputs /.../
printf "[ :3 - Finished copying outputs! - :3 ]\n\n"
```

Since we do not need those videos again on Metis, we safely delete them. 

Next, we take those JSON serialized pose mappings and run our inference on them. 

With our confidence score, we send a request to our AWS EC2 server, letting them know what the new status is.

```bash
# Kill  OpenPose
printf "[ :3 - Killing OpenPose... - :3 ]\n"
/bin/podman kill openpose
/bin/podman rm openpose
printf "[ :3 - Finished killing OpenPose! - :3 ]\n\n"

# Clean up files and submit confidence score
/.../.venv/bin/python /.../post_and_cleanup.py "$USER_ID" "$JOB_ID" ... ... ... "$FRONT_EXT" "$SIDE_EXT"
printf "[[ :3 - Ending job - :3 ]]"
```
# More
For additional, more in-depth documentation, it is suggested to first read the documentation in the associated modules below.

If your questions are still unanswered, the raw Rust source code is thoroughly documented.

If you still have questions, please reach out to the current project lead.