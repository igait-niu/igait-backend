# iGait ASD - Monorepo
![image](https://github.com/hiibolt/hiibolt/assets/91273156/6c3abf53-cc67-451c-a605-e76d5e726356)

This monorepo contains all the core components of the iGait ASD system:
- **igait-backend** - Backend server handling submission, upload, storage, and processing
- **igait-lib** - Shared library code used across the project
- **igait-pipeline** - Gait analysis pipeline for processing video data

## Documentation
It is highly recommended to read the [documentation](https://igait-niu.github.io/igait-backend/) as onboarding before contributing to this project.

## Installation & Usage

### Installation (One-Time Setup)

To install the iGait pipeline on your account, run:

```bash
/lstr/sahara/zwlab/jw/igait-pipeline/install.sh
```

The installation script will:
1. Verify the pipeline is built and ready
2. Set up the module system for your account
3. Update your `~/.bashrc` automatically

After installation, either:
- Run `source ~/.bashrc` to activate immediately, OR
- Start a new terminal session

### Using the Pipeline

Once installed, you can run the pipeline from anywhere:

```bash
# Load the module (only needed once per session)
module load igait

# Run the pipeline
igait-pipeline --input-path-front front.mp4 --input-path-side side.mp4 --output-dir-path ./output
```

**Example:**
```bash
module load igait
igait-pipeline \
  --input-path-front /path/to/front_video.mp4 \
  --input-path-side /path/to/side_video.mp4 \
  --output-dir-path /path/to/output
```

The pipeline will process the videos through 7 stages:
1. Media Conversion
2. Validity Check
3. Reframing
4. Pose Estimation (OpenPose)
5. Cycle Detection
6. Prediction (ML Model)
7. Archive

### Additional Options

```bash
# Skip to a specific stage (useful for debugging)
igait-pipeline --skip-to-stage 5 --input-path-front ... --input-path-side ... --output-dir-path ...

# Show help
igait-pipeline --help
```

---

## For Developers

### Building from Source

```bash
# Build all workspace members
cargo build --release

# Build specific component
cargo build --release -p igait-pipeline
cargo build --release -p igait-backend

# Run tests
cargo test --workspace
```

### Using Nix Flakes

If you have Nix with flakes enabled:

```bash
# Build the backend
nix build .#igait-backend

# Build the Docker image
nix build .#docker
docker load < result

# Enter development shell
nix develop
```

## Technologies Used
- ⚡ - Rust and Cargo
- 🔭 - Nix Flakes
- 🌱 - Docker
- 🌟 - Cloudflare
- 🕖 - Nginx
- ⚡ - Google Firebase Authentication
- 🎹 - Google Realtime DB
- 🔭 - Amazon S3 and EC2 (m6in.large)

## Service Map
<img src="https://github.com/user-attachments/assets/3eaebabc-ac73-4041-a866-c7221923f94a" width=750></img>

### Development Team
- [John W](https://github.com/hiibolt) - Head Backend and Systems Engineer
- [Michael S](https://github.com/michaelslice) - Head Frontend Engineer