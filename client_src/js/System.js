'use strict';

class System
{
    static updateFPS()
    {
        let curTs = Date.now();
        if (System.lastTs === undefined) {
            System.fpsElement = document.getElementById('fps');
            System.lastTs = curTs;
            System.sumDt = 0;
            System.fpsCounter = 0;
            return;
        }

        let dt = curTs - System.lastTs;
        System.lastTs = curTs;

        System.sumDt += dt;
        System.fpsCounter++;
        if (System.fpsCounter === 60) {
            System.fpsElement.textContent = parseInt(60 / System.sumDt * 1000);
            System.sumDt = 0;
            System.fpsCounter = 0;
        }
    }
}