'use strict';

class Bound
{
    constructor()
    {
        this.left = 0;
        this.right = 0;
        this.top = 0;
        this.down = 0;
        this.defined = false;
    }

    set(left, right, top, down)
    {
        this.left = left;
        this.right = right;
        this.top = top;
        this.down = down;
    }

    isDefined()
    {
        return this.defined;
    }

    setDefined()
    {
        this.defined = true;
    }
}