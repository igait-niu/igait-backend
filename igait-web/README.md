# iGait ASD - Frontend
The sole objective of this repository is to interact with the [iGAIT backend](https://github.com/igait-niu/igait-backend), as well as provide educational and informative resources to help people learn more about ASD.

## Prerequisites
- Bun: Ensure Bun is installed on your machine. [Download here](https://bun.sh)
- Docker Account: This project's frontend and backend are containerized using Docker. Setup [here](https://www.docker.com/products/docker-desktop/)

## Setup Guide (Local Development)
1. Clone the repository:
   ```bash
   git clone https://github.com/igait-niu/igait-frontend.git
   cd igait-frontend
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Start the development server:
   ```bash
   bun dev
   ```

The app will now be running locally, usually at [http://localhost:5173](http://localhost:5173).

---

## How to Update Frontend Container(Production)
1. Login to AWS
2. Navigate to our instance in EC2
3. `cd igait-backend`
4. `docker pull ghcr.io/igait-niu/igait-web:latest`
5. `docker compose down`
6. `docker compose up -d`

Changes will now be visible at [igaitapp.com](http://igaitapp.com)

---

### Development Team
* [John W](https://github.com/hiibolt) - Head Backend and Systems Engineer
* [Michael S](https://github.com/michaelslice) - Head Frontend Engineer
