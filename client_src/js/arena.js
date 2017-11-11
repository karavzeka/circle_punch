'use strict';

class Arena
{
    constructor(canvas)
    {
        this.canvas = canvas;
        this._players = [];
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
        this._players.push(player);
        player.setArena(this);
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
    init()
    {
        for (let player of this._players) {
            player.respawn();
        }
    }

    /**
     * Обновление состояния арены
     */
    update()
    {
        for (let player of this._players) {
            player.update();
        }
    }

    /**
     * Отрисовка арены
     */
    draw()
    {
        this.clearCanvas();
        for (let player of this._players) {
            player.draw();
        }
    }
}