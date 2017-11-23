'use strict';

class Arena
{
    constructor(canvas)
    {
        this.canvas = canvas;
        this.players = {};
        this.mainPlayer = null;
    }

    getCanvas()
    {
        return this.canvas;
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
    }

    /**
     * Установлен ли главный игрок
     *
     * @returns {boolean}
     */
    issetMainPlayer()
    {
        return this.mainPlayer !== undefined;
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
     * Очищает холст
     */
    clearCanvas()
    {
        let ctx = this.canvas.getContext('2d');
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }

    /**
     * Инициализация арены
     */
    // init()
    // {
    //     for (let player of this.players) {
    //         player.respawn();
    //     }
    // }

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
        for (let player_id in this.players) {
            this.players[player_id].draw();
        }
    }
}