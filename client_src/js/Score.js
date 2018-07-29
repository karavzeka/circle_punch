'use strict';

class Score
{
    constructor(arena)
    {
        this.arena = arena;
        this.table = document.getElementById('score');
        this.tbody = this.table.getElementsByTagName('tbody').item(0);
    }

    update(scoreList)
    {
        let rows = this.tbody.getElementsByTagName('tr');
        let len = rows.length;
        for (let i = 1; i < len; i++) {
            rows.item(1).remove();
        }

        for (let scoreItem of scoreList) {
            let tr = document.createElement('tr');
            let youStr = this.arena.getMainPlayer().nickname === scoreItem.nickname ? ' (you)' : '';
            tr.innerHTML = '<td>' + scoreItem.nickname + youStr + '</td><td>' + scoreItem.score + '</td>';
            this.tbody.appendChild(tr);
        }
    }
}