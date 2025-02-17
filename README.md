# EdgeZone 🚀

Blazingly fast, lightweight, and extensible edge deployment solution 🔥

## What's This All About? 🤔
Ever wondered what to do with those Raspberry Pis collecting dust? Here's my story!

I've got several low-powered Raspberry Pis running various apps like [astropie](https://github.com/Smartlinuxcoder/astropie/). While modern deployment platforms like Vercel or self-hosted Coolify offer amazing features, they're just too heavy for these little devices. SSH-ing, git pulling, making backups - it all becomes super sluggish on a Pi Zero.

Here's where EdgeZone comes in! 💡 Instead of running resource-hungry deployment solutions, EdgeZone offloads the fancy UI and management features to a main server, while a tiny but mighty API (built in Rust 🦀) handles all deployments across your nodes.

The result? A mere 660KB RAM footprint on your Pi! 🎯

## Installation 🛠️
EdgeZone consists of two components:
- Master Console (manages all deployments)
- Nodes (your Raspberry Pis or other edge devices)

### Master Console Setup 🎛️
The frontend comes pre-packaged in a Docker container:
```bash
docker pull ghcr.io/smartlinuxcoder/edgenode-master:latest
```

Configuration is simple - just add this to your `.env` file:
```env
ORIGIN=https://your.domain.com:port
```

### Node Setup 📡
Quick one-line installation:
```bash
curl -sSL https://raw.githubusercontent.com/Smartlinuxcoder/edgezone/main/install.sh | bash
```

Once installed, head to your Master Console's UI to configure your node. The interface is intuitive and user-friendly! 🚀

# Wanna collab?
Issues and discussions welcome!

Licensed under GNU GPL v3.0 - see [LICENSE](LICENSE).
