# palworld-save-repair

[![GitHub License](https://img.shields.io/github/license/YDKK/palworld-save-repair)](https://github.com/YDKK/palworld-save-repair/blob/master/LICENSE) [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ydkk/palworld-save-repair/docker-image.yml)](https://github.com/YDKK/palworld-save-repair/actions/workflows/docker-image.yml) [![Docker Image Size (tag)](https://img.shields.io/docker/image-size/ydkk/palworld-save-repair/latest)](https://hub.docker.com/r/ydkk/palworld-save-repair/tags)


Tool to repair corrupted save data of Palworld

> [!WARNING]
> Not well tested. Use at your own risk. Be sure to back up your saved data before using.

## Feature

This tool fixes the Palworld player save data problem where the `save_game_type` changes from `/Script/Pal.PalWorldPlayerSaveGame` to `None.PalWorldPlayerSaveGame` and becomes unloadable (character creation screen is displayed).

This tool is intended for use with dedicated game servers.

## Usage

### docker compose example

```yaml
services:
  palworld:
    #...
    volumes:
      - ./palworld:/palworld/
  save-repair:
    image: ydkk/palworld-save-repair:latest
    volumes:
      - ./palworld:/palworld/
    environment:
      - PLAYERS_SAVE_PATH=/palworld/Pal/Saved/SaveGames/0/0123456789ABCDEF0123456789ABCDEF/Players
    # check and repair save data every 1 hour
    command: ["/bin/sh", "-c", "while true; do sleep 3600; /app/pal_world_save_repair; done"]
    restart: unless-stopped
```

## Reference

- [palworld-save-tools](https://github.com/cheahjs/palworld-save-tools) - Save data structure

## License

MIT
