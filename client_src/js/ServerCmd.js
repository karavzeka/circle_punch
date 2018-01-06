'use strict';

const CMD_TYPE_PLAYERS = 'players';
const CMD_TYPE_MAP = 'map';

class ServerCmd
{
    constructor(arena) {
        this.arena = arena;
    }

    process(json) {
        let cmd = JSON.parse(json);
        // console.log(cmd);
        if (cmd.cmd_type === undefined) {
            return;
        }

        switch (cmd.cmd_type) {
            case CMD_TYPE_PLAYERS:
                this.processPlayersCmd(cmd);
                break;
            case CMD_TYPE_MAP:
                console.log(cmd);
                this.processMapCmd(cmd);
                break;
        }
    }

    processPlayersCmd(cmd) {
        for (let playerCmd of cmd.players) {
            let playerId = playerCmd.player_id;
            let player = null;
            if (playerCmd.it_is_you) {
                if (this.arena.issetMainPlayer()) {
                    player = this.arena.getMainPlayer();
                } else {
                    player = new Player(playerId, 'Super', true);
                    this.arena.setMainPlayer(player);
                }
            } else {
                if (this.arena.issetPlayer(playerId)) {
                    player = this.arena.getPlayer(playerId);
                } else {
                    player = new Player(playerId, 'Some', false);
                    this.arena.addPlayer(player);
                }
            }

            player.setPosition(playerCmd.position.x, playerCmd.position.y);

            // console.log(player);
        }

        // Удаление отключенных игроков
        if (cmd.disconnected_players !== undefined && cmd.disconnected_players.length > 0) {
            for (let playerId of cmd.disconnected_players) {
                this.arena.removePlayer(playerId);
            }
        }
    }

    processMapCmd(cmd)
    {
        if (cmd.walls === undefined) {
            return;
        }
        for (let i = 0; i < cmd.walls.length; i++) {
            let wall = cmd.walls[i];
            this.arena.addWall(wall.position.x, wall.position.y, wall.edge_size);
        }
    }
}