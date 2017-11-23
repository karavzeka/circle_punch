'use strict';

const IP = '127.0.0.1';
const PORT = '9002';

let wsGuardCounter = 0;

class WsController
{
    constructor() {
        if (wsGuardCounter > 0) {
            return;
        }

        wsGuardCounter++;
        WsController.wsController = new WsController();
        this.onMessageCallback = null;
    }

    /**
     * Возвращает единственный инстанс класса WsController.
     * Реализует singleton
     *
     * @returns {WsController}
     */
    static getInstance()
    {
        if (WsController.wsController === undefined || !(WsController.wsController instanceof WsController)) {
            WsController.wsController = new WsController();
        }

        return WsController.wsController;
    }

    /**
     * Выполняет соединение с сервером
     */
    connect()
    {
        if (this.onMessageCallback === null) {
            throw new Error('Socket listener is undefined. You should set callback function.');
        }

        if (!(this.connection instanceof WebSocket)) {
            this.connection = new WebSocket('ws://' + IP + ':' + PORT, "rust-websocket");
            this.connection.addEventListener('message', this.onMessageCallback);
        }
    }

    /**
     * Устанавливает коллбэк, вызываемый при приеме сообщений
     *
     * @param callback
     * @returns {WsController}
     */
    setListener(callback)
    {
        if (this.connection instanceof WebSocket) {
            // Если сокет уже слушается, то заменяем ему обработчик события
            this.connection.removeEventListener('message');
            this.connection.addEventListener('message', callback);
        } else {
            // Иначе записываем обработчик в переменную для дальнейшего использования
            this.onMessageCallback = callback;
        }
        return this;
    }

    /**
     * Отправляет сообщение
     *
     * @param message
     */
    send(message)
    {
        this.connection.send(message);
    }

    /**
     * Открыто ли соединение
     *
     * @returns {boolean}
     */
    isOpen()
    {
        return (this.connection instanceof WebSocket && this.connection.readyState === WebSocket.OPEN);
    }
}