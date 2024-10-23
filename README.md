## Features

- 🏢 Multi-user
- ✍️ Effective and distraction free notemaking
- 🖋 Plaintext or markdown
- 🔐 Secure authentication and session management
- 🌱 Extremely simple to self-host
- 🌲 Extremely light on resources
- 🍃 Extremely lightweight on the browser
- ⚒️ Simple configuration

## Getting started

1. First, install `docker-compose` by following the instructions [here](https://docs.docker.com/compose/install/)
2. Open your browser and head to http://localhost:2022

If you're having issues, consider reading the [configuration guide](./CONFIG.md).

## Updating to the latest release

To update to the latest release (don't worry, you won't lose any data), run this:

```sh
cd jotsy && sudo docker pull ohsayan/jotsy:latest && sudo docker-compose up -d
```

This will pull the latest Jotsy image, stop the existing instance, rebuild the container and start it up again.
