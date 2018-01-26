'use strict';

class Player
{
    constructor(id, name, isMain = false)
    {
        this.id = id;
        this.name = name;
        this.isMain = isMain;
        this.cmd = null;
        if (isMain) {
            this.cmd = new ClientCmd();
        }

        this.arena = null;
        this.radius = 16;
        this.posX = 0;
        this.posY = 0;
        this.color = 'rgba(0, 0, 200, 0.7)';

        this.moveSpeed = 1.2;

        this.bound = new Bound();
    }

    /**
     * Дает игроку ссылку на объект арены
     *
     * @param arena
     */
    setArena(arena)
    {
        this.arena = arena;
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
     * Return bounds of the player
     *
     * @returns {Bound}
     */
    getBound()
    {
        this.bound.set(
            this.posX - this.radius,
            this.posX + this.radius,
            this.posY - this.radius,
            this.posY + this.radius
        );
        return this.bound;
    }

    /**
     * Обновление состояния игрока
     */
    update()
    {
        // if (this.isMain) {
        //     if (this.isMoveRight()) {
        //         this.posX += this.moveSpeed;
        //     }
        //     if (this.isMoveLeft()) {
        //         this.posX -= this.moveSpeed;
        //     }
        //     if (this.isMoveUp()) {
        //         this.posY -= this.moveSpeed;
        //     }
        //     if (this.isMoveDown()) {
        //         this.posY += this.moveSpeed;
        //     }
        //     // console.log(this.posX);
        // }
    }

    /**
     * Отрисовка игрока
     */
    draw()
    {
        if (!this.arena.camera.isVisible(this.getBound())) {
            return;
        }

        let ctx = this.arena.getContext2D();
        ctx.beginPath();
        ctx.arc(this.arena.camera.correctDrawX(this.posX), this.arena.camera.correctDrawY(this.posY), this.radius, 0, Math.PI * 2, true);
        ctx.closePath();
        ctx.fillStyle = this.color;
        ctx.fill();
    }

    updateCmd()
    {
        if (isKeyPressed(GAME_KEYS.VK_D) || isKeyPressed(GAME_KEYS.VK_RIGHT)) {
            this.cmd.move_vector.x++;
        }
        if (isKeyPressed(GAME_KEYS.VK_A) || isKeyPressed(GAME_KEYS.VK_LEFT)) {
            this.cmd.move_vector.x--;
        }
        if (isKeyPressed(GAME_KEYS.VK_W) || isKeyPressed(GAME_KEYS.VK_UP)) {
            this.cmd.move_vector.y--;
        }
        if (isKeyPressed(GAME_KEYS.VK_S) || isKeyPressed(GAME_KEYS.VK_DOWN)) {
            this.cmd.move_vector.y++;
        }
        if (isKeyPressed(GAME_KEYS.VK_SPACE)) {
            this.cmd.shot = true;
        }
    }

    getCmd()
    {
        return this.cmd;
    }
}