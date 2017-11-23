'use strict';

class ServerCmd
{
    constructor(arena) {
        this.arena = arena;
    }

    process(json) {
        let cmd = JSON.parse(json);
        console.log(cmd);
        for (let playerCmd of cmd.players) {
            let playerId = playerCmd.player_id;
            let player = null;
            if (cmd.it_is_you) {
                if (this.arena.issetMainPlayer) {
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

            player.setPosition(playerCmd.move_vector.x, playerCmd.move_vector.y);

            // console.log(player);
        }
    }
}