'use strict';

class Score
{
    constructor()
    {
        this.table = document.getElementById('score');
        this.tbody = this.table.getElementsByTagName('tbody').item(0);
        this.headRow = this.table.getElementsByTagName('tr').item(0);
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
            tr.innerHTML = '<td>' + scoreItem.nickname + '</td><td>' + scoreItem.score + '</td>';
            this.tbody.appendChild(tr);
        }
    }
}