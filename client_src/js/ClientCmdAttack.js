'use strict';

class ClientCmdAttack extends ClientCmd
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
            cmd_type: 'Attack'
        }
    }
}