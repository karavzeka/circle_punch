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

        this.health = 0;
        this.health_max = 0;
        this.healthRelative = 0;
        this.is_health_changed = false;
        this.get_damage = false;
        this.healthRadius = 12;
        this.healthColor = 'rgba(0, 255, 0, 1)';

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
     * Sets current health
     *
     * @param health
     */
    setHealth(health)
    {
        if (health < this.health) {
            this.get_damage = true;
        }
        this.health = health;
    }

    /**
     * Обновление состояния игрока
     */
    update()
    {
        if (this.get_damage) {
            this.color = 'rgba(200, 0, 0, 0.7)';
            this.get_damage = false;
        } else {
            this.color = 'rgba(0, 0, 200, 0.7)';
        }

        this.healthRelative = this.health / this.health_max;
        let healthColorR, healthColorG;
        if (this.healthRelative > 0.5) {
            healthColorR = Math.ceil(255 - (255 * this.healthRelative * 2 - 255));
            healthColorG = 255;
        } else {
            healthColorR = 255;
            healthColorG = Math.ceil(255 * this.healthRelative * 2);
        }
        this.healthColor = 'rgba(' + healthColorR + ', ' + healthColorG + ', 0, 1)';
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
        let x = this.arena.camera.correctDrawX(this.posX);
        let y = this.arena.camera.correctDrawY(this.posY);
        ctx.beginPath();
        ctx.arc(x, y, this.radius, 0, Math.PI * 2, true);
        ctx.closePath();
        ctx.fillStyle = this.color;
        ctx.fill();

        // Drawing health
        ctx.beginPath();
        let startAngle = Math.PI * 1.5;
        let endAngle = startAngle - 2 * Math.PI * this.healthRelative;
        ctx.arc(x, y, this.healthRadius, startAngle, endAngle, true);
        ctx.strokeStyle = this.healthColor;
        ctx.lineWidth = 3;
        ctx.stroke();
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