'use strict';

class ServerCmd
{
    constructor(arena) {
        this.arena = arena;
    }

    process(json) {
        let cmd = JSON.parse(json);
        console.log('Server ts: ' + cmd.time);
        // console.log(cmd);
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
}