# Hands-on Rust

Demo repository for my Rust language experiments. Basically, this is a simple REST client
to call the Kubernetes API and retrieve the list of pods for a namespace.

## Building and Running

```bash
$ kubectl proxy &
$ cargo run -- kube-system
```

## Maintainer

M.-Leander Reimer (@lreimer), <mario-leander.reimer@qaware.de>

## License

This software is provided under the MIT open source license, read the `LICENSE`
file for details.
