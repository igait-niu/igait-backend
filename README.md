# iGait

## Development
### Dependencies
Please use Linux or [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) to work on this repository. It's **strongly** encouraged not to use Windows.

You'll want to have the following installed on your machine:
- [Docker](https://www.docker.com/)
- [Nix](https://nixos.org/download/). Enable [Nix Flakes](https://nixos.wiki/wiki/flakes)
- [`direnv`](https://direnv.net/), for fast dev environment loading. Be sure to hook your shell!

First, download this repository:
```bash
git clone https://github.com/igait-niu/igait.git
cd igait
```

Then, download the `.envrc` and `gcp-key.json` files from "iGait Credentials/Monorepo" in Onedrive. Place `.envrc` in the project root, and `gcp-key.json` at `credentials/gcp-key.json`.

If you've correctly hooked your shell with `direnv`, you should see the following:
```bash
direnv: error /home/hiibolt/igait/.envrc is blocked. Run `direnv allow` to approve its content
```
...if not, go back and ensure you installed/hooked correctly.

If you're using WSL2, you may need to normalize `.envrc` from CRLF to LF format. This can be done by selecting the file in Visual Studio Code, clicking the "CRLF" button in the footer panel, and selecting LF instead.

Next, run `direnv allow` as it suggests, and it should automatically download all dependencies, and load all environment variables. Neat, right?

### Starting iGait
Please save any work open on your machine before starting the application - Docker will try to build the entire application in parallel by default, which even beefy machines will struggle with for a first-time build. My laptop usually bricks itself when I try.

If your machine has under 64gb of RAM, first run the following commands to build a cache - it's normal to take quite a while the first time:
```bash
docker compose build web
docker compose build backend
docker compose build stage1
docker compose build stage2
docker compose build stage3
docker compose build stage4
docker compose build stage5
docker compose build stage6
docker compose build stage7
```

Once you've built this initial cache, you can build and run iGait with the following:
```bash
docker compose up --build
```

This will bring the entire stack online - frontend, backend, and pipeline stages! You can view the frontend now at https://localhost:4173.

Other useful commands:
- `docker compose down` - Takes iGait offline
- `docker compose logs <container>` - View the logs

### Working on iGait
**Backend/Pipeline**:
Each pipeline stage and the backend can (and should!) be worked on entirely independently.

Starting a stage/the backend is as simple as navigating to its respective folder and running the following, where `<port>` is the port you'd like to start it on:
```bash
export PORT=<port> && cargo run --release
```

**Frontend**:
Working on the frontend is a bit more complicated, since it may rely on the 