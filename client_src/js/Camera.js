'use strict';

class Camera
{
    constructor(width, height)
    {
        this.width = width;
        this.height = height;
        this.posX = 0;
        this.posY = 0;
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
     * Gets position of observed object and watches it
     *
     * @param x
     * @param y
     */
    observeObject(x, y)
    {
        this.posX = x - this.width / 2;
        this.posY = y - this.height / 2;

        if (this.posX < 0) {
            this.posX = 0;
        }
        if (this.posY < 0) {
            this.posY = 0;
        }

        if (this.posX + this.width > arena.width) {
            this.posX = arena.width - this.width;
        }
        if (this.posY + this.height > arena.height) {
            this.posY = arena.height - this.height;
        }
    }

    /**
     * Checks if at least a piece of the object is visible
     *
     * @param bound {Bound}
     * @returns {boolean}
     */
    isVisible(bound)
    {
        let objectWidth = bound.right - bound.left;
        let objectHeight = bound.down - bound.top;
        let inHorizontalBound = bound.left >= this.posX - objectWidth && bound.right < this.posX + this.width + objectWidth;
        let inVerticalBound = bound.top >= this.posY - objectHeight || bound.down < this.posY + this.height + objectHeight;
        return inHorizontalBound && inVerticalBound;
    }

    correctDrawX(x)
    {
        return Math.floor(x - this.posX);
    }

    correctDrawY(y)
    {
        return Math.floor(y - this.posY);
    }
}