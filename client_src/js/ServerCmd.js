'use strict';

const CMD_TYPE_PLAYERS = 'players';
const CMD_TYPE_MAP = 'map';
const CMD_TYPE_BAD_REG = 'bad_reg';

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
                this.processMapCmd(cmd);
                break;
            case CMD_TYPE_BAD_REG:
                this.processBadRegCmd(cmd);
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
                    player = new Player(playerId, playerCmd.nickname, true);
                    this.arena.setMainPlayer(player);
                }
            } else {
                if (this.arena.issetPlayer(playerId)) {
                    player = this.arena.getPlayer(playerId);
                } else {
                    player = new Player(playerId, playerCmd.nickname, false);
                    this.arena.addPlayer(player);
                }
            }

            player.setPosition(playerCmd.position.x, playerCmd.position.y);

            if (playerCmd.health_max !== undefined) {
                player.health_max = playerCmd.health_max;
            }

            if (playerCmd.health !== undefined) {
                player.setHealth(playerCmd.health);
            }
        }

        // Удаление отключенных игроков
        if (cmd.disconnected_players !== undefined && cmd.disconnected_players.length > 0) {
            for (let playerId of cmd.disconnected_players) {
                this.arena.removePlayer(playerId);
            }
        }

        // Обработка волн
        if (cmd.waves !== undefined) {
            for (let wave of cmd.waves) {
                if (this.arena.issetWave(wave.id)) {
                    if (wave.is_dead) {
                        this.arena.removeWave(wave.id);
                    } else {
                        let waveObj = this.arena.getWave(wave.id);
                        waveObj.radius = wave.r;
                    }
                } else {
                    this.arena.addWave(wave.id, wave.position.x, wave.position.y, wave.r);
                }
            }
        }

        // Обновление счета
        if (cmd.score_list !== undefined) {
            console.log(cmd.score_list);
            this.arena.score.update(cmd.score_list)
        }
    }

    processMapCmd(cmd)
    {
        this.arena.setWidth(cmd.width);
        this.arena.setHeight(cmd.height);

        if (cmd.walls === undefined) {
            return;
        }
        for (let i = 0; i < cmd.walls.length; i++) {
            let wall = cmd.walls[i];
            this.arena.addWall(wall.position.x, wall.position.y, wall.edge_size);
        }

        if (cmd.spikes !== undefined) {
            for (let i = 0; i < cmd.spikes.length; i++) {
                let spike = cmd.spikes[i];
                let drawBody = {
                    point_1: {
                        x: spike.draw_body.point_1.x,
                        y: spike.draw_body.point_1.y
                    },
                    point_2: {
                        x: spike.draw_body.point_2.x,
                        y: spike.draw_body.point_2.y
                    }
                };
                let dangerBody = {
                    point_1: {
                        x: spike.danger_body.point_1.x,
                        y: spike.danger_body.point_1.y
                    },
                    point_2: {
                        x: spike.danger_body.point_2.x,
                        y: spike.danger_body.point_2.y
                    }
                };
                let normal = {
                    x: spike.normal.x,
                    y: spike.normal.y
                };
                let vecAlongDpike = {
                    x: spike.vec_along_spike.x,
                    y: spike.vec_along_spike.y
                };
                this.arena.addSpike(
                    drawBody,
                    dangerBody,
                    normal,
                    vecAlongDpike,
                    spike.height,
                    spike.needle_half_width,
                );
            }
        }
    }

    processBadRegCmd(cmd)
    {
        alert(cmd.message);
    }
}