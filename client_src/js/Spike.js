'use strict';

class Spike
{
    // constructor(x1, y1, x2, y2, nx, ny, vx, vy, height, needleHalfWidth)
    constructor(drawBody, dangerBody, normal, vecAlongSpike, height, needleHalfWidth)
    {
        this.drawBody = drawBody;
        this.dangerBody = dangerBody;
        this.normal = normal;
        this.vecAlongSpike = vecAlongSpike;
        this.height = height;
        this.needleHalfWidth = needleHalfWidth;
        this.bound = new Bound();
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
            let left = Math.min(this.drawBody.point_1.x, this.drawBody.point_2.x);
            if (this.normal.x < 0) {
                left += this.normal.x * this.height;
            }

            let right = Math.max(this.drawBody.point_1.x, this.drawBody.point_2.x);
            if (this.normal.x > 0) {
                right += this.normal.x * this.height;
            }

            let top = Math.min(this.drawBody.point_1.y, this.drawBody.point_2.y);
            if (this.normal.y < 0) {
                top += this.normal.y * this.height;
            }

            let down = Math.max(this.drawBody.point_1.y, this.drawBody.point_2.y);
            if (this.normal.y > 0) {
                down += this.normal.y * this.height;
            }

            this.bound.set(
                left,
                right,
                top,
                down
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

        let x1 = this.arena.camera.correctDrawX(this.drawBody.point_1.x);
        let y1 = this.arena.camera.correctDrawY(this.drawBody.point_1.y);
        let x2 = this.arena.camera.correctDrawX(this.drawBody.point_2.x);
        let y2 = this.arena.camera.correctDrawY(this.drawBody.point_2.y);

        let ctx = this.arena.getContext2D();
        //TODO make gradient to fill
        ctx.fillStyle = '#555';
        ctx.beginPath();
        ctx.moveTo(x2, y2);
        ctx.lineTo(x1, y1);

        let curX = x1;
        let curY = y1;
        let sign = 1;

        // TODO improve code
        let stop = false;
         while (true) {
             if (Math.abs(this.vecAlongSpike.x) < 0.01) {
                 // Moving along Y
                 curX += this.height * this.normal.x * sign;
                 curY += this.vecAlongSpike.y * this.needleHalfWidth;
                 if (Math.abs(curY - y1) >= Math.abs(y2 - y1)) {
                     stop = true;
                     curY = y2;
                 }
             } else {
                 // Moving along X
                 curX += this.vecAlongSpike.x * this.needleHalfWidth;
                 curY += this.height * this.normal.y * sign;
                 if (Math.abs(curX - x1) >= Math.abs(x2 - x1)) {
                     stop = true;
                     curX = x2;
                 }
             }
             ctx.lineTo(curX, curY);
             if (stop) {
                 break;
             }
             sign *= -1;
         }

        ctx.fill();

        if (DEBUG_MODE) {
            x1 = this.arena.camera.correctDrawX(this.dangerBody.point_1.x);
            y1 = this.arena.camera.correctDrawY(this.dangerBody.point_1.y);
            x2 = this.arena.camera.correctDrawX(this.dangerBody.point_2.x);
            y2 = this.arena.camera.correctDrawY(this.dangerBody.point_2.y);

            ctx.strokeStyle = '#ff00cc';
            ctx.beginPath();
            ctx.moveTo(x2, y2);
            ctx.lineTo(x1, y1);
            ctx.stroke();
        }
    }
}