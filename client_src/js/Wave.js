'use strict';

class Wave {
    constructor(posX, posY, radius)
    {
        this.posX = posX;
        this.posY = posY;
        this.radius = radius;
        this.bound = new Bound();
        this.arena = null;
    }

    /**
     * Sets link to arena
     *
     * @param arena
     */
    setArena(arena)
    {
        this.arena = arena;
    }

    /**
     * Return bounds of the wall
     *
     * @returns {Bound}
     */
    getBound()
    {
        if (!this.bound.isDefined()) {
            this.bound.set(
                this.posX - this.radius,
                this.posX + this.radius,
                this.posY - this.radius,
                this.posY + this.radius
            );
            this.bound.setDefined();
        }
        return this.bound;
    }

    draw() {
        if (!this.arena.camera.isVisible(this.getBound())) {
            return;
        }

        let ctx = this.arena.getContext2D();
        let x = this.arena.camera.correctDrawX(this.posX);
        let y = this.arena.camera.correctDrawY(this.posY);
        ctx.beginPath();
        ctx.arc(x, y, this.radius, 0, Math.PI * 2, true);
        ctx.closePath();
        ctx.strokeStyle = 'rgba(150, 200, 150, 0.5)';
        ctx.lineWidth = 3;
        ctx.stroke();
    }
}