'use strict';

class Arena
{
    constructor(canvas)
    {
        this.players = {};
        this.mainPlayer = null;
        // Canvas
        this.canvas = canvas;
        this.ctx = null;
        this.walls2D = new Path2D();
    }

    getContext2D()
    {
        if (this.ctx === null) {
            this.ctx = this.canvas.getContext('2d');
        }
        return this.ctx;
    }

    /**
     * Добавляет пользователя на арену
     *
     * @param player
     */
    addPlayer(player)
    {
        this.players[player.id] = player;
        player.setArena(this);
    }

    /**
     * Существует ли указанный игрок в стеке игроков
     *
     * @param playerId
     * @returns {boolean}
     */
    issetPlayer(playerId) {
        return this.players[playerId] !== undefined;
    }

    /**
     * Удаляет игрока из стека
     *
     * @param playerId
     */
    removePlayer(playerId) {
        delete this.players[playerId];
    }

    /**
     * Возвращает указанного игрока
     *
     * @param playerId
     * @returns {*}
     */
    getPlayer(playerId)
    {
        return this.players[playerId];
    }

    /**
     * Устанавливает главного игрока (которым рулит пользователь)
     *
     * @param player
     */
    setMainPlayer(player)
    {
        this.mainPlayer = player;
        this.mainPlayer.setArena(this);
    }

    /**
     * Установлен ли главный игрок
     *
     * @returns {boolean}
     */
    issetMainPlayer()
    {
        return this.mainPlayer !== null;
    }

    /**
     * Возвращает главного играка
     *
     * @returns {*|null}
     */
    getMainPlayer()
    {
        return this.mainPlayer;
    }

    /**
     * Удаляет всех игроков
     */
    removeAllPlayers()
    {
        this.players = {};
        this.mainPlayer = null;
    }

    /**
     * Добавляет стену к canvas'у.
     * Добавляется без возможности удаления.
     *
     * @param x
     * @param y
     * @param size
     */
    addWall(x, y, size)
    {
        this.walls2D.rect(x, y, size, size);
    }

    /**
     * Очищает холст
     */
    clearCanvas()
    {
        let ctx = this.canvas.getContext('2d');
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }

    /**
     * Обновление состояния арены
     */
    update()
    {
        for (let player_id in this.players) {
            this.players[player_id].update();
        }
    }

    /**
     * Отрисовка арены
     */
    draw()
    {
        this.clearCanvas();
        this.drawWalls();
        this.mainPlayer.draw();
        for (let player_id in this.players) {
            this.players[player_id].draw();
        }
    }

    drawWalls()
    {
        let ctx = this.getContext2D();
        ctx.fillStyle = '#A00';
        ctx.fill(this.walls2D);
    }
}