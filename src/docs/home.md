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

# 1 - API
### 1.1 - Layout and Explanation
The API has three routes:
* [`api/v1/historical_submissions`](routes::historical)
* [`api/v1/upload`](routes::upload)

In the lifecycle of a job, first, the patient information and files are uploaded to the server via the `upload` route.
Then, the job is processed by the server, and eventually shipped to **Metis**. 
Next, the server waits until Metis has completed processing (by scanning the `inputs` folder)
Finally, the server pulls the outputs from Metis into the `outputs` folder and processes/records them.

To learn more about why the API is designed how it is, or more about how it works, skip to the [API section](#).

To see more information about a specific route, see [the routes module](routes).
### 1.2 - Notes
* The API is currently versioned at `v1`, meaning every route is actually at `/api/v1/<route>`.
* The `upload` and `historical_submissions` endpoints are public 

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
- `AWS_REGION`: The region the AWS SDK should expect - in this case, `us-east-2`
- `FIREBASE_ACCESS_KEY`: Found in the Google Firebase API settings

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

So the natural question is - what machine will we use for these intensive computations?

Well, we have opted to use [Metis](https://www.niu.edu/crcd/prospective-user/index.shtml), which is an absolute powerhouse of computation. However, there are many limitations of the machine.

For one, the machine does not allow any port-forwarding. This means that the backend must be hosted elsewhere (which will be discussed shortly). On top of this, the only way to achieve maximum performance is through a job scheduling system - [PBS Professional](https://altair.com/pbs-professional).

Before worrying more about the hardware and 'server-izing', let's think about how we will actually perform these computations for a singular run with just one front and side video.

### 4.3 - Planning for The Inference Process
In order to use Metis, we must be able to fit our entire process into one Bash script that is run by the PBS job scheduler, called a batchfile.

Because we need to fit a ton of stuff into one file, it's important to have a clear goal of what the inputs, outputs, and dependancies of the script are.

**Inputs**
* A front-facing walking video
* A side-facing walking video


**Outputs**
* A skeletonized front and side-facing video
* A confidence score representing the possibility of ASD


**Dependencies**
* TONS of Python dependancies
* OpenPose (the worst example of dependency hell i've ever laid eyes on)


Looking at this, there's some things we can decide to do. Firstly, we'll create a folder structure that makes sense:
```text
/lstr/sahara/zwlab/data
                     \- inputs 
                     \- outputs
                     \- scripts
```

We'll place our inputs and outputs in the `inputs` and `outputs` folder respectively, and put our machine learning model, Python dependency list and anything else in the `scripts` folder.

Our next problem is that OpenPose is a mess. Compiling it ourselves on Metis is almost out of the question just because of the sheer number of dependencies, often with conflicting or outdated versions. So, how can we get around this?

Well, the cluster has [Apptainer (AKA Singularity)](https://apptainer.org/) - and you can build these container images from Docker images - for which OpenPose has documentation! So, we build that image, and also store it in the `scripts` folder.

Now, the next problem is Python. Python, oh, Python. We have many dependencies, and installing globally with Python is a universally terrible idea, especially with complex and massive machine learning packages like TensorFlow or Keras. The most common solution is to use a `.venv` folder for caching an isolated environment - but there were many problems with that. The solution we opted to use was a user installation, and enabling login-linger for the user that we installed the packages on. That way, we don't have to re-install every time, which takes notable time for some packages.

And just like that, our programs are ready to use, and we know where to direct inputs and outputs. Piping between programs is as simple as moving between the `inputs` and `outputs` folder!

### The Backend
So the next and natural question arises: How do we actually develop a backend for this?

It's not possible to host a backend on Metis, so the workaround to this is to host another server that can. In this case, we set up a small instance on AWS EC2. It doesn't need crazy specs, it's just going to handle files - Metis will do all the heavy lifting. However, because it could in production be taking in massive files, high network thouroughput was very important!

So how do we actually submit our files? Well, firstly, there must be an API to upload these files. 

#### The Upload Endpoint
This endpoint must be able to accept patient data and the front/side videos, so that's what it does first!

Once recieving the patient data, the next step taken is to upload it to Google Firebase Realtime DB for logistics purposes. The data is assigned into a "job", and given a status - `Queue`.

Next, we need to somehow get this data over to Metis for processing. There's no way to set up a server for it on Metis, but we do have the ability to use SSH!

The backend uses the `scp` command to move the two files into the aforementioned `inputs` directory on Metis.

Next, it signals to PBS to start the job, and moves the files into another `inputs` directory *on the AWS server* - it will be looked at by the `inputs` daemon!

#### The Inputs Daemon
This is a thread that repeatedly scans the `inputs` directory on the AWS EC2 instance. It's looking at each entry, and checking for when it completes on Metis.

Once the job actually completes on Metis, it then copies the outputs from the `outputs` folder on Metis into the `outputs` folder on the AWS EC2 instance.

It then updates the status of the job on the Google Firebase!

#### The Outputs Daemon
This is another thread that repeatedly scans the `outputs` directory on the AWS EC2 instance.

This daemon first uploads the outputs to S3 for permanent storage. Then, it inspects the outputs to see if there was a confidence score produced.

If there *was* a confidence score, it emails the user the score and updates the entry in the DB.
If there *wasn't* a confidence score, it reports the error to the user by email and updates the entry in the DB.

Finally, it deletes the `outputs` folder from both the AWS EC2 instance and Metis!

#### The Historical Route
It's very possible that users could lose their results, or want to see an aggregate collection of all of their submissions. To allow this, we provide a `historical_submissions` route that can email a (potentially filtered collection of the results the user wishes to see.

There are a variety of ways to filter this - if you wish to learn more about it, please see the documentation for the route!

# More
For additional, more in-depth documentation, it is suggested to first read the documentation in the associated modules below.

If your questions are still unanswered, the raw Rust source code is thoroughly documented.

If you still have questions, please reach out to the current project lead.
