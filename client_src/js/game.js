'use strict';

let canvas;

//input objects
let input = {
    // mouse_down:  false,
    keys: new Array(256),
};

const VK_W = 87;
const VK_A = 65;
const VK_S = 83;
const VK_D = 68;
const VK_SPACE = 32;

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
function initEvents() {
    document.addEventListener("keydown", function (event) {
        input.keys[event.keyCode] = true;

        event.preventDefault();
    });

    document.addEventListener("keyup", function (event) {
        input.keys[event.keyCode] = false;
        event.preventDefault();
    });

    document.getElementById('connect-button').addEventListener('click', function () {
        WsController.getInstance()
            .setListener(function(event) {
                let received = document.getElementById("received");
                let br = document.createElement("BR");
                let text = document.createTextNode(event.data);
                received.appendChild(br);
                received.appendChild(text);
                console.log(WsController.getInstance().isOpen());
            })
            .connect();
    });

    document.getElementById('text-form').addEventListener('submit', function (event) {
        event.preventDefault();
        let input = document.getElementById('message');
        WsController.getInstance().send(input.value);
    });
}

function runGame(arena)
{
    // console.log('game cycle');
    arena.update();
    arena.draw();
    setTimeout(function () {runGame(arena);}, DT * 1000);
}

if (initCanvas()) {
    initEvents();

    let arena = new Arena(canvas);

    let player = new Player('Super', true);
    let player_1 = new Player('Player_1', false);
    arena.addPlayer(player);
    arena.addPlayer(player_1);
    arena.init();

    runGame(arena);
    // console.log(input);
}
