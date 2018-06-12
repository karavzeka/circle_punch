'use strict';

//input objects
let input = {
    // mouse_down:  false,
    keys: new Array(256),
};

const GAME_KEYS = {
    // Game codes
    VK_W: 87,
    VK_A: 65,
    VK_S: 83,
    VK_D: 68,
    VK_LEFT: 37,
    VK_UP: 38,
    VK_RIGHT: 39,
    VK_DOWN: 40,
    VK_SPACE: 32,
    // Common codes (to check is the pressed key is  the game key)
    87: true,
    65: true,
    83: true,
    68: true,
    37: true,
    38: true,
    39: true,
    40: true,
    32: true,
};

const KEY_DOWN = 'keydown';
const KEY_UP = 'keyup';

let global = {
    stopGame: false,
    keysQueue: []
};

const FPS = 60;
const DT = 1/FPS;

let url = new URL(location.href);
const DEBUG_MODE = url.searchParams.has('debug') && url.searchParams.get('debug');

function isKeyPressed(code)
{
    return input.keys[code];
}

function initEvents(arena)
{
    // Button events
    document.addEventListener(KEY_DOWN, function (event) {
        if (!isKeyPressed(event.keyCode) && document.activeElement.tagName !== 'INPUT' && GAME_KEYS[event.keyCode] !== undefined) {
            input.keys[event.keyCode] = true;

            global.keysQueue.push(new KeyAction(event.keyCode, KEY_DOWN));

            // let player = arena.getMainPlayer();
            // if (player !== null) {

                // player.updateCmd();
            // }

            event.preventDefault();
        }
    });

    document.addEventListener(KEY_UP, function (event) {
        input.keys[event.keyCode] = false;
        global.keysQueue.push(new KeyAction(event.keyCode, KEY_UP));
        event.preventDefault();
    });

    // WebSocket events
    let serverCmd = new ServerCmd(arena);
    let wsController = WsController.getInstance();
    wsController
        .setListener(function (event) {
            serverCmd.process(event.data);
        });

    wsController
        .setOnOpen(function (event) {
            processFrame(arena);
        });

    wsController
        .setOnClose(function (event) {
            global.stopGame = true;
            arena.removeAllPlayers();

            let notificationElement = document.getElementById('ws-notification');
            let element = document.createElement('div');
            element.textContent = 'Disconnected [' + WsController.getCloseReason(event) + ']';
            notificationElement.appendChild(element);
            setTimeout(function () {
                element.remove();
            }, 20000);
        });

    // Other events
    document.getElementById('connect-button').addEventListener('click', function () {
        let serverIp = document.getElementById('server-ip').value;
        wsController.connect(serverIp);
    });
}

function processFrame(arena)
{
    if (arena.issetMainPlayer()) {
        arena.update();
        arena.draw();

        let player = arena.getMainPlayer();
        player.updateCmd(global.keysQueue);

        let playerCmd = player.getCmd();
        if (playerCmd.isReadyForSend()) {
            // console.log(JSON.stringify(playerCmd));
            WsController.getInstance().send(JSON.stringify(playerCmd));
        }
        playerCmd.toDefault();
    }

    if (global.stopGame) {
        global.stopGame = false;
        return;
    }

    System.updateFPS();
    setTimeout(function () {processFrame(arena);}, DT * 1000);
}

// Run
let canvas = document.getElementById('canvas');
let arena = new Arena(canvas);
initEvents(arena);