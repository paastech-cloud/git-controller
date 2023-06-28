# Deploiment


## 1. Git authentication layer (git-auth-layer)

In order for this workflow to work, you will need to install a github runner on your machine : (Add a runner)[https://docs.github.com/en/enterprise-cloud@latest/actions/hosting-your-own-runners/managing-self-hosted-runners/adding-self-hosted-runners]

All files for this folder will be copied over to the server on which this program will be run.

This will be done through a github workflow that executes every time there will be a **push on main**.

This action uses the appleboy/scp-action package to copy the github repository files to a specified server.

To be able to use the repository as a source, we need to call the actions/checkout.

In order to work, you need to define 3 secrets : 

- **USERNAME**: the username of the server you want to connect to
- **HOST**: address of the server you want to connect to
- **SSH_KEY**: the ssh key to connect to the server

You can choose to replace PASSWORD by KEY and put an ssh key in its place.


Change the ```target``` key to whatever folder you want your authentification repository to be saved in.


## 2. Git repository manager (git-repo-manager)

To start the git repository manager, please place yourself at the root of the repository and launch the docker compose :

```sh
docker compose up
```

Should you have any issues with an invalid address, please think of changing the line 

```rust
let addr = format!("[::1]:{}", std::env::var(port_key).unwrap()).parse()?;
```

To 
```rust
let addr = format!("0.0.0.0:{}", std::env::var(port_key).unwrap()).parse()?;
```
in the file *git-repo-manager/src/main.rs* to allow the program to communicate outside the container.