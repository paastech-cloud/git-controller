# Deploiment


## 1. Git authentication layer (git-auth-layer)




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