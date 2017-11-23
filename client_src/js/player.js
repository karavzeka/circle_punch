'use strict';

class Player
{
    constructor(id, name, isMain = false)
    {
        this.id = id;
        this.name = name;
        this.isMain = isMain;

        this._arena = null;
        this.radius = 16;
        this.posX = 0;
        this.posY = 0;
        this.color = 'rgba(0, 0, 200, 0.5)';
        // this.speed = 0;

        this.moveSpeed = 1.2;
    }

    /**
     * Дает игроку ссылку на объект арены
     *
     * @param arena
     */
    setArena(arena)
    {
        this._arena = arena;
    }

    /**
     * Случайно генерирует положение игрока для респауна
     */
    respawn()
    {
        this.posX = Math.random() * (this._arena.getCanvas().width - 4 * this.radius) + 2 * this.radius;
        this.posY = Math.random() * (this._arena.getCanvas().height - 4 * this.radius) + 2 * this.radius;
    }

    /**
     * Выставляет положение пользователя
     *
     * @param posX
     * @param posY
     */
    setPosition(posX, posY)
    {
        this.posX = posX;
        this.posY = posY;
    }

    /**
     * Обновление состояния игрока
     */
    update()
    {
        if (this.isMain) {
            if (this.isMoveRight()) {
                this.posX += this.moveSpeed;
            }
            if (this.isMoveLeft()) {
                this.posX -= this.moveSpeed;
            }
            if (this.isMoveUp()) {
                this.posY -= this.moveSpeed;
            }
            if (this.isMoveDown()) {
                this.posY += this.moveSpeed;
            }
            // console.log(this.posX);
        }
    }

    /**
     * Отрисовка игрока
     */
    draw()
    {
        let ctx = this._arena.getCanvas().getContext('2d');
        ctx.beginPath();
        ctx.arc(Math.floor(this.posX), Math.floor(this.posY), this.radius, 0, Math.PI * 2, true);
        ctx.closePath();
        ctx.fillStyle = this.color;
        ctx.fill();
    }

    isMoveRight() {
        return isKeyPressed(VK_D);
    }

    isMoveLeft() {
        return isKeyPressed(VK_A);
    }

    isMoveUp() {
        return isKeyPressed(VK_W);
    }

    isMoveDown() {
        return isKeyPressed(VK_S);
    }
}