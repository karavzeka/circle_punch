'use strict';

class System
{
    static updateFPS()
    {
        let curTs = Date.now();
        console.log('Client ts: ' + curTs);
        console.log('');
        if (System.lastTs === undefined) {
            System.fpsElement = document.getElementById('fps');
            System.lastTs = curTs;
            System.sma = 0; // Simple moving average
            return;
        }

        let dt = curTs - System.lastTs;
        System.lastTs = curTs;

        let fps = 0;
        if (dt > 0) {
            fps = (1 / dt * 1000);
        }

        System.sma = System.sma + (fps - System.sma) / 10;

        System.fpsElement.textContent = parseInt(System.sma);
    }
}