'use strict';

class Arena
{
    constructor(canvas)
    {
        this.players = {};
        this.mainPlayer = null;
        // Width and height of arena (not canvas)
        this.width = 0;
        this.height = 0;
        // Canvas
        this.canvas = canvas;
        this.ctx = null;
        this.walls = [];
        this.spikes = [];

        this.camera = new Camera(canvas.width, canvas.height);
        this.camera.setArena(this);
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
     * Sets width of arena
     *
     * @param width
     */
    setWidth(width)
    {
        this.width = width;
    }

    /**
     * Sets height of arena
     *
     * @param height
     */
    setHeight(height)
    {
        this.height = height;
    }

    /**
     * Adds a wall to arena
     *
     * @param x
     * @param y
     * @param size
     */
    addWall(x, y, size)
    {
        let wall = new Wall(x, y, size);
        wall.setArena(this);
        this.walls.push(wall);
    }

    /**
     * Adds spike to arena
     *
     * @param drawBody
     * @param dangerBody
     * @param normal
     * @param vecAlongSpike
     * @param height
     * @param needleHalfWidth
     */
    addSpike(drawBody, dangerBody, normal, vecAlongSpike, height, needleHalfWidth)
    {
        let spike = new Spike(drawBody, dangerBody, normal, vecAlongSpike, height, needleHalfWidth);
        spike.setArena(this);
        this.spikes.push(spike);
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
        this.camera.observeObject(this.mainPlayer.posX, this.mainPlayer.posY);
        this.mainPlayer.update();
        for (let player_id in this.players) {
            this.players[player_id].update();
        }
    }

    /**
     * Draw arena and its objects
     */
    draw()
    {
        this.clearCanvas();
        this.drawWalls();
        this.drawPlayers();
        this.drawSpikes();
    }

    drawPlayers()
    {
        this.mainPlayer.draw();
        for (let player_id in this.players) {
            this.players[player_id].draw();
        }
    }

    drawWalls()
    {
        for (let i = 0, len = this.walls.length; i < len; i++) {
            this.walls[i].draw();
        }
    }

    drawSpikes()
    {
        for (let i = 0, len = this.spikes.length; i < len; i++) {
            this.spikes[i].draw();
        }
    }
}