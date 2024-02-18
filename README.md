# palworld-save-repair
Tool to repair corrupted save data of Palworld

> [!WARNING]
> Not well tested. Use at your own risk. Be sure to back up your saved data before using.

## Feature

Fixes an issue where the `save_game_type` of player save data changes from `/Script/Pal.PalWorldPlayerSaveGame` to `None.PalWorldPlayerSaveGame` and becomes unloadable (character creation screen is displayed).

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
