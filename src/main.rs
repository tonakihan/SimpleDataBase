use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::{self, prelude::*};
//Мб реализовать конфигурационный файл и UI?

fn main() -> Result<()> {
    let path_db = "./data/test.db";
    let conn_db = Connection::open(path_db)?;
    
    println!("Выберете нужный запрос/режим:");
    println!("\t1. Ведомость\n\t2. Посещаемость\n\t3. Замена");
    print!(">> ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode)
        .expect("Filed to read line");
    mode = mode.trim().to_string();

    match mode.as_str() {
        "1" => vedom(&conn_db)?,
        "2" => posesh(&conn_db)?,
        "3" => refer(&conn_db)?,
        _ => println!("Error! Not found mode: {}.", mode),
    }

    Ok(())
}


fn vedom(conn_db: &Connection) -> Result<()> {
    #[derive(Debug)]
    struct Ved {
        last_name: String,
        name:      String,
        otch:      String,
        subjects:  String,
        semester:  u8,
        mark:      u8,
    }

    let mut stmt = conn_db.prepare("
        SELECT s.'Фамилия', s.'Имя', s.'Отчество', 
          p.'Наименование', v.'Симестр', v.'Оценка'
        FROM 'Ведомость' v
        INNER JOIN 'Cтуденты' s ON s.'Номер_зачетки'=v.'Номер_студент'
        INNER JOIN 'Предметы' p ON p.'id'=v.'Номер_предмет';",
    )?;

    let data = stmt.query_map([], |row| {
        Ok(Ved {
            last_name: row.get(0)?,
            name:      row.get(1)?,
            otch:      row.get(2)?,
            subjects:  row.get(3)?,
            semester:  row.get(4)?,
            mark:      row.get(5)?,
        })
    })?;
    
    println!();
    println!("\tData from DB:");
    println!(
        "{:20}|{:20}|{:20}|{:28}|{:8}|{:7}",
        "Фамилия", "Имя", "Отчество", 
        "Предмет", "Симестр", "Оценка"
    ); 
    println!("{:-<108}","");
    for part in data {
        let part = part?;
        println!(
            "{:20}|{:20}|{:20}|{:28}|{:<8}|{:<7}", 
            part.last_name,
            part.name,
            part.otch,
            part.subjects,
            part.semester,
            part.mark,
        );
    }

    Ok(())      
}


fn posesh(conn_db: &Connection) -> Result<()> {
    struct Pos {
        subjects:     String,
        themes_learn: String,
        last_name:    String,
        name:         String,
        otch:         String,
        date:         String,
        presence:     String,
        mark:         u8,
    }

    let mut stmt = conn_db.prepare("
        SELECT pr.'Наименование', pl.'Тема_занятия', s.'Фамилия', 
          s.'Имя', s.'Отчество', po.'Дата', po.'Присутствие', po.'Оценка'
        FROM 'Посещаемость' po
        INNER JOIN 'Cтуденты' s ON s.'Номер_зачетки'=po.'Студент'
        INNER JOIN 'Предметы' pr ON pr.'id'=po.'Предмет'
        INNER JOIN 'План_обучения' pl ON pl.'id'=po.'Тема_занятия';",
    )?;

    let data = stmt.query_map([], |row| {
        Ok(Pos {
            subjects:     row.get(0)?,
            themes_learn: row.get(1)?,
            last_name:    row.get(2)?,
            name:         row.get(3)?,
            otch:         row.get(4)?,
            date:         row.get(5)?,
            presence:     row.get(6)?,
            mark:         row.get(7)?,
        })
    })?;
    
    println!();
    println!("\tData from BD:");
    println!(
        "{:28}|{:20}|{:20}|{:20}|{:20}|{:11}|{:12}|{:7}",
        "Предмет", "Тема занятия", "Фамилия",
        "Имя", "Отчество", "Дата", "Присутствие",
        "Оценка"
    );
    println!("{:-<145}", "");
    for part in data {
        let part = part?;
        println!(
            "{:28}|{:20}|{:20}|{:20}|{:20}|{:11}|{:12}|{:<7}",
            part.subjects,
            part.themes_learn,
            part.last_name,
            part.name,
            part.otch,
            part.date,
            part.presence,
            part.mark,
        ); // Возможно дописать для данных которые = NULL?
    }

    Ok(())
}


fn refer(conn_db: &Connection) -> Result<()> {
    struct Reference {
        last_name:  String,
        name:       String,
        otch:       String,
        date_begin: String,
        direction:  String,
        faculty:    String,
    }

    print!("Enter file path >> "); 
    io::stdout().flush().unwrap();
    let mut file_path = String::new();   
    io::stdin().read_line(&mut file_path)
        .expect("Filed to read line (input)"); 
    file_path = file_path.trim().to_string(); 

    let mut file = File::open(&file_path)
        .expect("Filed to open src_file");

    print!("Input ID student >> ");
    io::stdout().flush().unwrap();
    let mut id_stud = String::new();
    io::stdin().read_line(&mut id_stud)
        .expect("Filed read id_stud");

    let mut f_contents = String::new();
    file.read_to_string(&mut f_contents)
        .expect("Filed read file");
    
    
    let mut stmt = conn_db.prepare("
        SELECT s.'Фамилия', s.'Имя', s.'Отчество', n.'Дата_начала', n.'Описание', f.'Наименование'
        FROM 'Cтуденты' s
        INNER JOIN 'Направления' n ON s.'Группа' = n.'Название'
        INNER JOIN 'Факультет' f ON n.'Факультет' = f.'id'
        WHERE s.'Номер_зачетки' = ?1;
    ")?;
    let data = stmt.query_row([id_stud], |row| {
        Ok(Reference {
            last_name:  row.get(0)?,
            name:       row.get(1)?,
            otch:       row.get(2)?,
            date_begin: row.get(3)?,
            direction:  row.get(4)?,
            faculty:    row.get(5)?,
        })
    })?;
    f_contents = f_contents.replace(
        "BD.Студенты.Фамилия",
        data.last_name.as_str()
    );
    f_contents = f_contents.replace(
        "BD.Студенты.Имя", 
        data.name.as_str()
    );
    f_contents = f_contents.replace(
        "BD.Студенты.Отчество",
        data.otch.as_str()
    );
    f_contents = f_contents.replace(
        "BD.Направления.Дата_начала",
        data.date_begin.as_str()
    );
    f_contents = f_contents.replace(
        "BD.Направления.Описание",
        data.direction.as_str()
    );
    f_contents = f_contents.replace(
        "BD.Факультет.Наименование",
        data.faculty.as_str()
    );
    
    let mut new_file = File::create("./result.txt")
        .expect("Filed create file \"result.txt\"");
    new_file.write_all(f_contents.as_bytes())
        .expect("Filed write data!");

    Ok(())
}
