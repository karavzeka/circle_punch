'use strict';

class Wall
{
    constructor(posX, posY, edgeSize)
    {
        this.posX = posX;
        this.posY = posY;
        this.edgeSize = edgeSize;
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
                this.posX,
                this.posX + this.edgeSize,
                this.posY,
                this.posY + this.edgeSize
            );
            this.bound.setDefined();
        }
        return this.bound;
    }

    draw()
    {
        if (!this.arena.camera.isVisible(this.getBound())) {
            return;
        }

        let x = this.arena.camera.correctDrawX(this.posX);
        let y = this.arena.camera.correctDrawY(this.posY);

        let ctx = this.arena.getContext2D();
        ctx.beginPath();
        ctx.rect(x, y, this.edgeSize, this.edgeSize);
        ctx.fillStyle = '#A00';
        ctx.fill();
    }
}
