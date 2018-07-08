'use strict';

class ClientCmdMove extends ClientCmd
{
    constructor()
    {
        super();
        this.move_vector = {
            x: 0,
            y: 0
        };
    }

    toJSON()
    {
        return {
            cmd_type: 'Move',
            x: this.move_vector.x,
            y: this.move_vector.y
        }
    }
}