'use strict';

//input objects
let input = {
    // mouse_down:  false,
    keys: [],
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
    return input.keys[code] === true;
}

function initEvents(arena)
{
    // Button events
    document.addEventListener(KEY_DOWN, function (event) {
        if (!isKeyPressed(event.keyCode) && document.activeElement.tagName !== 'INPUT' && GAME_KEYS[event.keyCode] !== undefined) {
            input.keys[event.keyCode] = true;

            if (arena.issetMainPlayer()) {
                global.keysQueue.push(new KeyAction(event.keyCode, KEY_DOWN));
            }

            event.preventDefault();
        }
    });

    document.addEventListener(KEY_UP, function (event) {
        if (document.activeElement.tagName !== 'INPUT') {
            input.keys[event.keyCode] = false;
            global.keysQueue.push(new KeyAction(event.keyCode, KEY_UP));
            event.preventDefault();
        }
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
            // Hide connect form
            document.getElementById('connection-form').style.display = 'none';

            // Focus on nickname input
            document.getElementById('nickname-form').style.display = 'block';
            document.getElementById('name-input').focus();

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
    document.getElementById('connect-button').addEventListener('click', function (event) {
        let serverIp = document.getElementById('server-ip').value;
        let isConnected = wsController.connect(serverIp);
        console.log(wsController.isOpen());
        if (isConnected) {
            // for (let input of document.getElementById('connection-form').children) {
            //     input.setAttribute('disabled', 'disabled');
            // }

        }

        event.preventDefault();
    });

    document.getElementById('join-button').addEventListener('click', function (event) {
        let nickname = document.getElementById('name-input').value;

        if (nickname) {
            let cmd = {
                cmd_type: 'RegisterPlayer',
                nickname: nickname
            };

            wsController.send(JSON.stringify(cmd));
        }

        event.preventDefault();
    });
}

function processFrame(arena)
{
    arena.update();
    arena.draw();

    if (arena.issetMainPlayer()) {
        let player = arena.getMainPlayer();
        player.updateCmd(global.keysQueue);

        let playerCmdList = player.getCmdList();
        for (let playerCmd of playerCmdList) {
            if (playerCmd.isReadyForSend()) {
                WsController.getInstance().send(JSON.stringify(playerCmd));
            }
            playerCmd.toDefault();
        }
    }

    if (global.stopGame) {
        global.stopGame = false;
        return;
    }

    System.updateFPS();
    setTimeout(function () {processFrame(arena);}, DT * 1000);
}

function tryAutoRun()
{
    let ipInput = document.getElementById('server-ip');

    let host = window.location.hostname;
    if (!host) {
        host = DEFAULT_IP;
    }
    ipInput.value = host;

    let wsController = WsController.getInstance();
    wsController.connect(host);
}

// Run
let canvas = document.getElementById('canvas');
let arena = new Arena(canvas);
initEvents(arena);
tryAutoRun();