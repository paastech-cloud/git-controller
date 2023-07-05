<h1 align="center">Git Controller</h1>

<p align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Git-logo.svg/1280px-Git-logo.svg.png" alt="git" width=300 />
</p>

The git controller contains all the code concerning actions that can be done on git repositories, such as
hooks, authentification, creating and deleting repositories, etc.


## Dependencies: _dev on your local machine_

- Others:

```bash
# sudo apt install -y protobuf-compiler libp
```

- ssh-keys-fuse:

```
sudo apt-get install fuse3
sudo apt-get install libfuse-dev pkg-config
```

### :warning: Fuse :warning:: DON'T forgot to unmount dir on your local machine

Use the following command to unmount the directory.

```bash
umount <PATH_TO_DIR>
```

## Getting Started

To deploy locally `Git Controller`, follow these steps:

1. Make sure you have [docker](https://docs.docker.com/engine/install/) and [docker compose](https://docs.docker.com/compose/install/) plugin installed on your machine.

2. Clone this repository to your local machine and navigate to the project directory.

```bash
git clone https://github.com/paastech-cloud/git-controller.git
cd git-controller
```

1. Build and start project.

```bash
docker compose up --build
```

## Usage

To test the dynamic filesystem (fuse), issue the following command:

```bash
docker exec git-controller-fuse-1 cat /home/newuser/.ssh/authorized_keys
```

From there, you can start using Git Controller's CLI to manage your Git repositories efficiently.

## Good practices

### Rust

https://rustc-dev-guide.rust-lang.org/conventions.html

### Git

- Try to stick as close as possible to [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/), here is a [list of verbs](https://gkarthiks.github.io/quick-commands-cheat-sheet/conventional-commit-verbs.html)
- Use the provided pull request templates for pull requests
- Use [labels](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels#applying-a-labels)
