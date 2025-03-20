# DARK WITCH

    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
    MMMMMMMMMMMMds+:--------:+sdNMMMMMMMMMMM
    MMMMMMMMms:-+sdNMMMMMMMMNdy+--omMMMMMMMM
    MMMMMMh:` /mMMMMMMMMMMMMMMMMm+ `-yMMMMMM
    MMMMd--hN``--sNMMMMMMMMMMNy:..`md:.hMMMM
    MMM+`yMMMy hd+./hMMMMMMh/.+dd sMMMh`/MMM
    MM:.mMMMMM:.NMMh/.+dd+./hMMM--MMMMMm--NM
    M+`mMMMMMMN`+MMMMm-  .dMMMMo mMMMMMMN.:M
    d yMMMMMMMMy dNy:.omNs--sNm oMMMMMMMMh h
    /`MMMMMMMMMM.`.+dMMMMMMm+.``NMMMMMMMMM-:
    .:MMMMMMMd+./`oMMMMMMMMMMs /.+dMMMMMMM/`
    .:MMMMmo.:yNMs dMMMMMMMMm`oMNy:.omMMMM/`
    /`MNy:.omMMMMM--MMMMMMMM:.MMMMMNs--sNM.:
    d -` :++++++++: /++++++/ :++++++++:  : h
    M+ yddddddddddd+ yddddy /dddddddddddy`/M
    MM/.mMMMMMMMMMMM.-MMMM/.NMMMMMMMMMMm.:NM
    MMMo`sMMMMMMMMMMd sMMy hMMMMMMMMMMy`+MMM
    MMMMd--hMMMMMMMMM+`mN`/MMMMMMMMMh--hMMMM
    MMMMMMh:.omMMMMMMN.:/`NMMMMMMms.:hMMMMMM
    MMMMMMMMNs:./shmMMh  yMMNds/.:smMMMMMMMM
    MMMMMMMMMMMMdy+/---``---:+sdMMMMMMMMMMMM
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM

In memory key value database that supports SQL Queries

## Getting Started

These instructions will help you get a copy of the project up and running on your local machine.

```bash
git clone https://github.com/N1ghtStorm/dark-witch
cd dark-witch
cargo run -r --features local

curl -X GET http://localhost:3000/sql -H "Content-Type: text/plain" -d "SELECT * from main"
```

## SQL

Supports simple SELECT statements

```sql
SELECT * from main;
```

```sql
SELECT * FROM main WHERE name = 'John' AND age < 30;
```

```sql
SELECT name, age FROM main WHERE name = 'John' AND age < 30;
```