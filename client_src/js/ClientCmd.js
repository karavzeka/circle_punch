'use strict';

class ClientCmd
{
    constructor()
    {
        this.toDefault();
    }

    /**
     * Reset command to default state
     */
    toDefault()
    {
        this.move_vector = {
            x: 0,
            y: 0
        };
        this.attack = false;
    }

    /**
     * Prepare command to send
     */
    prepare()
    {
        if (this.move_vector.x === 0 && this.move_vector.y === 0) {
            delete this.move_vector;
        } else {
            if (this.move_vector.x === 0) {
                delete this.move_vector.x;
            } else if (this.move_vector.x > 1) {
                this.move_vector.x = 1;
            } else if (this.move_vector.x < -1) {
                this.move_vector.x = -1;
            }

            if (this.move_vector.y === 0) {
                delete this.move_vector.y;
            } else if (this.move_vector.y > 1) {
                this.move_vector.y = 1;
            } else if (this.move_vector.y < -1) {
                this.move_vector.y = -1;
            }
        }

        if (this.attack === false) {
            delete this.attack;
        }
    }

    /**
     * Is command empty. If yes, there is no need to send it.
     * It should be call after prepare.
     *
     * @returns {boolean}
     */
    isEmpty()
    {
        return this.move_vector === undefined && this.attack === false;
    }
}