'use strict';

let canvas;

//input objects
let input = {
    // mouse_down:  false,
    keys: new Array(256),
};

// const VK_W = 87;
// const VK_A = 65;
// const VK_S = 83;
// const VK_D = 68;
// const VK_SPACE = 32;
const GAME_KEYS = {
    // Game codes
    VK_W: 87,
    VK_A: 65,
    VK_S: 83,
    VK_D: 68,
    VK_SPACE: 32,
    // Common codes (to check is the pressed key is  the game key)
    87: true,
    65: true,
    83: true,
    68: true,
    32: true,
};

const FPS = 60;
const DT = 1/FPS;

function isKeyPressed(code) {
    return input.keys[code];
}

function initCanvas() {
    canvas = document.getElementById('canvas');
    return canvas;
}

// Set up event handling
function initEvents(arena) {
    document.addEventListener("keydown", function (event) {
        if (!isKeyPressed(event.keyCode) && document.activeElement.tagName !== 'INPUT' && GAME_KEYS[event.keyCode] !== undefined) {
            input.keys[event.keyCode] = true;

            let player = arena.getMainPlayer();
            if (player !== null) {

                player.updateCmd();
            }

            event.preventDefault();
        }
    });

    document.addEventListener("keyup", function (event) {
        input.keys[event.keyCode] = false;
        event.preventDefault();
    });

    let serverCmd = new ServerCmd(arena);
    let wsController = WsController.getInstance();
    wsController
        .setListener(function (event) {
            serverCmd.process(event.data);
        });

    wsController
        .setOnOpen(function (event) {
            runGame(arena);
        });

    wsController
        .setOnClose(function (event) {
            let notificationElement = document.getElementById('ws-notification');
            let element = document.createElement('div');
            element.textContent = 'Disconnected [' + WsController.getCloseReason(event) + ']';
            notificationElement.appendChild(element);
            setTimeout(function () {
                element.remove();
            }, 20000);
        });

    document.getElementById('connect-button').addEventListener('click', function () {
        let serverIp = document.getElementById('server-ip').value;
        wsController.connect(serverIp);
    });
}

function runGame(arena)
{
    if (arena.issetMainPlayer()) {
        arena.update();
        arena.draw();
        let player = arena.getMainPlayer();
        player.updateCmd();
        let playerCmd = player.getCmd();
        playerCmd.prepare();
        if (!playerCmd.isEmpty()) {
            console.log(JSON.stringify(playerCmd));
            WsController.getInstance().send(JSON.stringify(playerCmd));
        }
        playerCmd.toDefault();
        //TODO interrupt game on close connection
    }
    System.updateFPS();
    setTimeout(function () {runGame(arena);}, DT * 1000);
}

if (initCanvas()) {
    let arena = new Arena(canvas);
    initEvents(arena);

    // let player = new Player('Super', true);
    // let player_1 = new Player('Player_1', false);
    // arena.addPlayer(player);
    // arena.addPlayer(player_1);
    // arena.init();

    // runGame(arena);
    // console.log(input);
}
