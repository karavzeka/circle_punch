'use strict';

const DEFAULT_IP = '127.0.0.1';
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
        this.connection = null;
        this.onMessageCallback = null;
        this.onErrorCallback = null;
        this.onCloseCallback = null;
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
    connect(serverIp = '')
    {
        if (this.onMessageCallback === null) {
            throw new Error('Socket listener is undefined. You should set callback function.');
        }

        if (!(this.connection instanceof WebSocket) || !this.isOpen()) {
            let ip = DEFAULT_IP;
            if (serverIp) {
                ip = serverIp;
            }
            this.connection = new WebSocket('ws://' + ip + ':' + PORT, 'rust-websocket');
            this.connection.addEventListener('message', this.onMessageCallback);

            if (this.onErrorCallback) {
                this.connection.addEventListener('error', this.onErrorCallback);
            }
            if (this.onCloseCallback) {
                this.connection.addEventListener('close', this.onCloseCallback);
            }
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

    setOnError(callback)
    {
        this.onErrorCallback = callback;
        return this;
    }

    setOnClose(callback)
    {
        this.onCloseCallback = callback;
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

    /**
     * Returns the reason of for closing the connection
     *
     * @param closeEvent
     * @returns {string}
     */
    static getCloseReason(closeEvent)
    {
        let reason = '';
        switch (closeEvent.code) {
            case 1000:
                reason = 'Normal closure, meaning that the purpose for which the connection was established has been fulfilled.';
                break;
            case 1001:
                reason = 'An endpoint is \'going away\', such as a server going down or a browser having navigated away from a page.';
                break;
            case 1002:
                reason = 'An endpoint is terminating the connection due to a protocol error';
                break;
            case 1003:
                reason = 'An endpoint is terminating the connection because it has received a type of data it cannot accept (e.g., an endpoint that understands only text data MAY send this if it receives a binary message).';
                break;
            case 1004:
                reason = 'Reserved. The specific meaning might be defined in the future.';
                break;
            case 1005:
                reason = 'No status code was actually present.';
                break;
            case 1006:
                reason = 'The connection was closed abnormally, e.g., without sending or receiving a Close control frame';
                break;
            case 1007:
                reason = 'An endpoint is terminating the connection because it has received data within a message that was not consistent with the type of the message (e.g., non-UTF-8 [http://tools.ietf.org/html/rfc3629] data within a text message).';
                break;
            case 1008:
                reason = 'An endpoint is terminating the connection because it has received a message that \'violates its policy\'. This reason is given either if there is no other sutible reason, or if there is a need to hide specific details about the policy.';
                break;
            case 1009:
                reason = 'An endpoint is terminating the connection because it has received a message that is too big for it to process.';
                break;
            case 1010:
                reason = 'An endpoint (client) is terminating the connection because it has expected the server to negotiate one or more extension, but the server didn\'t return them in the response message of the WebSocket handshake. <br /> Specifically, the extensions that are needed are: ' + closeEvent.reason;
                break;
            case 1011:
                reason = 'A server is terminating the connection because it encountered an unexpected condition that prevented it from fulfilling the request.';
                break;
            case 1015:
                reason = 'The connection was closed due to a failure to perform a TLS handshake (e.g., the server certificate can\'t be verified).';
                break;
            default:
                reason = 'Unknown reason';
        }
        return reason;
    }
}