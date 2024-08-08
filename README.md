# iGait ASD - Backend
This is the primary brain behind everything in the iGait app. There are a variety of microservices involved with the submitssion, upload, storage, and more - this server handles all of this to bring our product to the convenience of a low-power mobile device.

*Under no circumstances should this repository ever be made public. To make it public could compromise sensitive client data. All code is property of the Dr. Ziteng Wang, the iGait Project, and Northern Illinois University.*

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

To see more information about a specific route, see [the routes module](routes).
### 1.2 - Notes
* The API is currently versioned at `v1`, meaning every route is actually at `/api/v1/<route>`.
* The `completion` endpoint is only for use by **Metis**.
* The `upload` and `historical_submissions` endpoints are for use by the iGait frontend.

# 2 - Codebase
### 2.1 - Structure
<img src="https://github-production-user-asset-6210df.s3.amazonaws.com/169108989/328351527-a6262923-a3a6-47e1-94d9-297513e1729d.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240808%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240808T013947Z&X-Amz-Expires=300&X-Amz-Signature=7d400b74fe999346368ca913dec1549c85d99455853fd58d149cbcaf1453bbbb&X-Amz-SignedHeaders=host&actor_id=91273156&key_id=0&repo_id=739494899" width=750></img>

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
git clone https://github.com/igait-niu/igait-backend.git
cd igait-backend
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