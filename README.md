# iGait ASD - Backend
This is the primary 'brain' behind everything in the iGait app. There are a variety of microservices involved with the submitssion, upload, storage, and more - this server handles this.

*Under no circumstances should this repository ever be made public. To make it public could compromise sensitive client data. All code is property of the Dr. Ziteng Wang, the iGait Project, and Northern Illinois University.*

## Tech Stack
Languages:
- **Rust** - 
  Arguably one of the best languages for writing a fault-tolerant backend is Rust. It's not error prone, and forces you to handle most edge cases in advance. The borrow checker system ensures there are very few surprises, and the speed is comparable *only* to native C.
- **Docker** - 
  We frequently moved around our server infastructure. To ensure that no matter where we did anything, stuff would work the first time, with zero config, we used Docker. There's no system-level installs, just starting the Docker container and watching the magic happen.

## Backend Request Handling and Operations
![image](https://github.com/igait-niu/igait-backend/assets/169108989/a6262923-a3a6-47e1-94d9-297513e1729d)

Let's first talk about the goal of the project. We want to take a video, convert it to pose mappings, and run an inference on them.
*Why here, and not on the customer's device?* - A valid question - but consider a Samsung from the early 2010s, and whether it'll be able to run that inference in any timely manner.

The solution is to instead upload the video to a device that *can* handle that inference in a timely manner. In this case, that's Metis - the NIU CRCD's flagship supercomputing cluster. However, Metis doesn't allow hosting web servers directly on it. How do we work around this?

### /api/v1/upload
Let's say we submit two videos, and our medical data to the backend. What happens is all of the **blue** arrows. 
Order of operations:
- User account is created as a **User** object and uploaded to Firebase Realtime DB
- User data is placed into a **Job** object, and added to the **User** object, then Firebase Realtime DB is updated
- The two videos are uploaded to AWS S3.
- The iGait backend opens an SSH session to Metis, and supplies it with the USER_ID and JOB_ID of the now-uploaded videos.
- Sends the user a 'Welcome to iGait!' email.


Then, Metis pulls the videos from S3 and runs the inference. Now, again, Metis can't really do 'web server' things like most servers can. How do we get around this?

### /api/v1/completion
The second endpoint is designed only for Metis to use, and is secured using a pre-set API key to prevent outside usage.

This allows Metis to let the central backend know that it has processed the videos and uploaded them to S3 for viewing and usage, and also provides the result from the inference.

After recieving this informaiton, the backend does the following:
- Updates the **Job** object's status for the **User** object, then updates Firebase Realtime DB
- Sends the user an email containing the results.

## Codebase Structure
The codebase is split up into multiple modules.

- **data/\***
  Directory in which the server's *State* handler looks at to check for un-processed jobs. When a job is built, a directory for it is created with various metadata here. When the server finishes uploading everything to S3, Realtime DB, and Metis, it automatically deletes the folder.
- **public/\***
  There is a (now pointless) proof-of-concept frontend. You may safely pretend this folder doesn't exist, it's for debugging purposes, however it would now require some modification to be useful, as our fields have changed. 

  Regardless, this website is served on `localhost:3000`. Our NGINX configuration intentionally doesn't let this page face the public - it shouldn't, as it allows manual, unfiltered data submission.
- **src/database/\***
  Handles everything related to Firebase Realtime DB.
- **src/email/\***
  Handles everything related to sending emails via Cloudflare Workers.
- **src/print/\***
  Printing utilities to help color server log output to distinguish whether a log message is related to S3, Firebase, Metis, etc.
- **src/request/\***
  Handles everything related to Metis SSH request sessions.
- **src/routers/\***
  Contains the actual routing server logic for the API.
- **src/state/\***
  Contains the logic for the server state handler so that everything stays properly synchronized across threads.
- **src/main.rs**
  Server entrypoint. Should contain very minimal code.

## Setting Up Your Development Environment
If you already use Nix Flakes, you can find my entire environment contained in `flake.nix`. 

If you don't, install the following:
- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/)

To run the backend, you will need to set a few environment variables:
```bash
AWS_ACCESS_KEY_ID=***REMOVED***
AWS_SECRET_ACCESS_KEY=***REMOVED***
FIREBASE_ACCESS_KEY=***REMOVED***
IGAIT_ACCESS_KEY=***REMOVED***
```

Next, clone the repository:
```bash
git clone https://github.com/igait-niu/igait-backend.git
cd igait-backend
```

Then, start the backend!
```bash
cargo run
```

To build the backend:
```bash
cargo build
```

Let's say you've done both. Before you commit it to GitHub, test to make sure it works in Docker too.
```bash
docker built -t testing .
docker run testing
```

Looks good? Kill the Docker container, comment your code, and commit it to the GitHub repository.
```bash
docker ps # find the name of the Docker container
docker kill <the name of the container>
```

Any changes to the `master` branch will set the GitHub worker to automatically build a private Docker container. Please note this process takes some time (5~ minutes), but you can view it from [the home page](https://www.github.com/igait-niu/igait-backend).
Once complete, to update the container on the AWS EC2 instance (from the `/igait-openpose` root directory): 
```bash
docker compose down
docker pull ghcr.io/igait-niu/igait-backend:latest
docker compose up -d
```
*Note: You will need to be logged into Docker with your GitHub. Since this is a private repository, you need to prove to GHCR that you have access! There are many guides online. You may need to issue yourself an authentication secret or key.*
