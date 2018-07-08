'use strict';

class ClientCmd
{
    constructor()
    {
        this.readyForSend = false;
    }

    /**
     * Reset command to default state
     */
    toDefault()
    {
        this.readyForSend = false;
    }

    /**
     * Is command empty. If yes, there is no need to send it.
     * It should be call after prepare.
     *
     * @returns {boolean}
     */
    isReadyForSend()
    {
        return this.readyForSend;
    }
}