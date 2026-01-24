# Media D-Bus Interface

Control MPRIS media players.

- **Service:** `com.wayle.Media1`
- **Path:** `/com/wayle/Media`

## Player Targeting

Methods accepting `player_id`:

- **Empty string `""`** - Targets the currently active player
- **Specific ID** - Targets a specific player (get IDs from `ListPlayers`)

## Methods

### Playback Control

| Method      | Arguments                    | Returns | Description                      |
| ----------- | ---------------------------- | ------- | -------------------------------- |
| `PlayPause` | `s player_id`                | -       | Toggle play/pause                |
| `Next`      | `s player_id`                | -       | Skip to next track               |
| `Previous`  | `s player_id`                | -       | Go to previous track             |
| `Seek`      | `s player_id, x position_us` | -       | Seek to position in microseconds |

### Mode Control

| Method          | Arguments              | Returns | Description                              |
| --------------- | ---------------------- | ------- | ---------------------------------------- |
| `SetShuffle`    | `s player_id, s state` | -       | Set shuffle: "on", "off", or "toggle"    |
| `SetLoopStatus` | `s player_id, s mode`  | -       | Set loop: "none", "track", or "playlist" |

### Player Management

| Method            | Arguments     | Returns  | Description                         |
| ----------------- | ------------- | -------- | ----------------------------------- |
| `ListPlayers`     | -             | `a(sss)` | List players: (id, identity, state) |
| `GetActivePlayer` | -             | `s`      | Get active player ID                |
| `SetActivePlayer` | `s player_id` | -        | Set active player (empty to clear)  |
| `GetPlayerInfo`   | `s player_id` | `a{ss}`  | Get detailed player info            |

## Properties

| Property       | Type | Access | Description                 |
| -------------- | ---- | ------ | --------------------------- |
| `ActivePlayer` | `s`  | read   | Currently active player ID  |
| `PlayerCount`  | `u`  | read   | Number of available players |
