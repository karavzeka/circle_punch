'use strict';

class System
{
    static updateFPS()
    {
        let curTs = Date.now();
        console.log(curTs);
        if (System.lastTs === undefined) {
            System.fpsElement = document.getElementById('fps');
            System.lastTs = curTs;
            return;
        }

        let dt = curTs - System.lastTs;
        System.lastTs = curTs;

        System.fpsElement.textContent = parseInt(1 / dt * 1000);
    }
}