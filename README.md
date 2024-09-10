# cdevent

A CLI tool for sending CDEvents

## Usage

### Service Deployed

```shell
./cdevent -e https://acme.com service deployed --id 1 --source zsh --subid xyz --envid local --artifact pkg:123 --custom team=team1,service=service-1
```

Results in the following CloudEvent body:

```json
{
  "context": {
    "id": "1",
    "source": "zsh",
    "timestamp": "2024-09-10T04:55:25.428634Z",
    "type": "dev.cdevents.service.deployed.0.1.1",
    "version": "0.3.0"
  },
  "customData": {
    "service": "service-1",
    "team": "team1"
  },
  "subject": {
    "content": {
      "artifactId": "pkg:123",
      "environment": {
        "id": "local"
      }
    },
    "id": "xyz",
    "source": "zsh",
    "type": "service"
  }
}
```

## Resources

- https://github.com/cdevents/spec/blob/v0.4.1/cloudevents-binding.md
- https://docs.rs/argfile/0.2.1/argfile/
- https://doc.rust-lang.org/cargo/guide/project-layout.html
- https://github.com/cdevents/sdk-rust/
- https://docs.rs/clap/latest/clap/index.html