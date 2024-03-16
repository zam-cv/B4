# Qrops: A game for Verqor

Qrops is a 2D video game that combines entertainment with education, allowing players to experience the challenges of farming interactively. Through this game, we seek to empower users with knowledge and perspective on agricultural operations.

## Getting Started

### Prerequisites

- [Nix](https://nixos.org/download.html)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Git](https://git-scm.com/downloads)

### Installation

```bash
git clone https://github.com/zam-cv/B4
cd B4
```

### Running the game

For Development:

```bash
nix-shell
cd backend
cargo run --release
```

For Production:

```bash
docker-compose --profile prod up
```

## Usage

Once the appropriate environment is set up and running, access the game through the provided web interface. The specific address will depend on your setup but is typically http://localhost:8080 for local development environments or a predefined domain for production setups.