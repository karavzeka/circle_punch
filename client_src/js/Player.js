'use strict';

class Player
{
    constructor(id, nickname, isMain = false)
    {
        this.id = id;
        this.nickname = nickname;
        this.isMain = isMain;
        this.moveCmd = null;
        this.attackCmd = null;
        if (isMain) {
            this.moveCmd = new ClientCmdMove();
            this.attackCmd = new ClientCmdAttack();
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
        this.healthColor = 'rgba(0, 0, 0, 1)';

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

        let nicknameColorR = healthColorR - 20;
        nicknameColorR = nicknameColorR < 0 ? 0 : nicknameColorR;
        let nicknameColorG = healthColorG - 80;
        nicknameColorG = nicknameColorG < 0 ? 0 : nicknameColorG;
        this.nicknameColor = 'rgba(' + nicknameColorR + ', ' + nicknameColorG + ', 0, 1)';
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

        // Drawing nickname
        ctx.beginPath();
        ctx.textAlign = 'center';
        ctx.font = '11px sans-serif';
        ctx.fillStyle = this.nicknameColor;
        ctx.fillText(this.nickname, x, y - this.radius - 4);
    }

    updateCmd(keysQueue)
    {
        let  len = keysQueue.length;
        for (let i = 0; i < len; i++) {
            let keyAction = keysQueue.pop();
            if (keyAction.code === GAME_KEYS.VK_D || keyAction.code === GAME_KEYS.VK_RIGHT) {
                if (keyAction.action === KEY_DOWN) {
                    this.moveCmd.move_vector.x++;
                } else {
                    this.moveCmd.move_vector.x--;
                }
            }
            if (keyAction.code === GAME_KEYS.VK_A || keyAction.code === GAME_KEYS.VK_LEFT) {
                if (keyAction.action === KEY_DOWN) {
                    this.moveCmd.move_vector.x--;
                } else {
                    this.moveCmd.move_vector.x++;
                }
            }
            if (keyAction.code === GAME_KEYS.VK_W || keyAction.code === GAME_KEYS.VK_UP) {
                if (keyAction.action === KEY_DOWN) {
                    this.moveCmd.move_vector.y--;
                } else {
                    this.moveCmd.move_vector.y++;
                }
            }
            if (keyAction.code === GAME_KEYS.VK_S || keyAction.code === GAME_KEYS.VK_DOWN) {
                if (keyAction.action === KEY_DOWN) {
                    this.moveCmd.move_vector.y++;
                } else {
                    this.moveCmd.move_vector.y--;
                }
            }
            this.moveCmd.readyForSend = true;
        }
        if (isKeyPressed(GAME_KEYS.VK_SPACE)) {
            this.attackCmd.readyForSend = true;
        }
    }

    getCmdList()
    {
        return [this.moveCmd, this.attackCmd];
    }
}