use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::{self, prelude::*};
//TODO: Мб реализовать конфигурационный файл и UI?

fn main() -> Result<()> {
    let path_db = "./data/test.db";
    let conn_db = Connection::open(path_db)?;
    
    println!("");
    let mode = get_input("\
        Выберете нужный запрос/режим:\n\
        \t1. Ведомость\n\
        \t2. Посещаемость\n\
        \t3. Замена\n\
        \t4. Вставка\n\
    ");

    match mode.as_str() {
        "1" => vedom(&conn_db)?,
        "2" => posesh(&conn_db)?,
        "3" => refer(&conn_db)?,
        "4" => insert_db(&conn_db)?,
        _ => println!("Error! Not found mode: {}.", mode),
    }
    Ok(())
}


fn vedom(conn_db: &Connection) -> Result<()> {
    struct Ved {
        last_name: String,
        name:      String,
        otch:      String,
        subjects:  String,
        semester:  u8,
        mark:      u8,
    }

    let mut stmt = conn_db.prepare("
        SELECT s.Фамилия, s.Имя, s.Отчество, 
          p.Наименование, v.Семестр, v.Оценка
        FROM 'Ведомость' v
        INNER JOIN 'Cтуденты' s ON s.Номер_зачетки=v.Номер_студент
        INNER JOIN 'Предметы' p ON p.id=v.Номер_предмет",
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
        SELECT pr.Наименование, pl.Тема_занятия, s.Фамилия, 
          s.Имя, s.Отчество, po.Дата, po.Присутствие, po.Оценка
        FROM 'Посещаемость' po
        INNER JOIN 'Cтуденты' s ON s.Номер_зачетки=po.Студент
        INNER JOIN 'Предметы' pr ON pr.id=po.Предмет
        INNER JOIN 'План_обучения' pl ON pl.id=po.Тема_занятия",
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
        ); //TODO: Возможно дописать для данных которые = NULL?
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

    let file_path = get_input("Введи путь до файла");
    let mut file = File::open(&file_path)
        .expect("Filed to open src_file");

    let id_stud = get_input("Введи ID студента ");

    let mut f_contents = String::new();
    file.read_to_string(&mut f_contents)
        .expect("Filed read file");
    
    let mut stmt = conn_db.prepare("
        SELECT s.'Фамилия', s.'Имя', s.'Отчество', n.'Дата_начала', 
          n.'Описание', f.'Наименование'
        FROM 'Cтуденты' s
        INNER JOIN 'Направления' n ON s.'Группа' = n.'Название'
        INNER JOIN 'Факультет' f ON n.'Факультет' = f.'id'
        WHERE s.'Номер_зачетки' = ?1
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


fn get_input(question: &str) -> String {
    print!("{}>> ", question);
    io::stdout().flush().unwrap();
    let mut data = String::new();
    io::stdin().read_line(&mut data)
        .expect("Filed to read data in get_input");
    data = data.trim().to_string();
     
    return data
}


fn insert_db(conn_db: &Connection) -> Result<()> {
    let mode = get_input("\
        Выберете цель вставки:\n\
        \t1. Ведомость\n\
        \t2. Посещаемость\n\
        \t3. Тема занятия\n\
    ");

    match mode.as_str() {
        "1" => in_vedom(&conn_db)?,
        "2" => in_posesh(&conn_db)?,
        "3" => in_themes_learn(&conn_db)?,
        _ => println!("Error! Not found mode: {}.", mode),
    }

    fn in_vedom(conn_db: &Connection) -> Result<()> {
        let num_subj = get_subject(conn_db)?;
        let semester: u32 = get_input("Введи семестр ").parse()
            .expect("Filed to convert semester in in_vedom");
        
        let mut count: u8 = get_input("Введи кол-во новых записей ").parse()
            .expect("Filed to convert count in in_vedom");

        while count > 0 {
            let num_stud = get_student(conn_db)?;
            let marks: u32 = get_input("Введи оценку (0,2..5) ").parse()
                .expect("Filed to convert marks in in_vedom");
            
            conn_db.execute("
                INSERT INTO 'Ведомость' (Номер_студент, Номер_предмет, Оценка, Семестр)
                VALUES (?1,?2,?3,?4)",
                [&num_stud, &num_subj, &marks, &semester],
            )?;
            count -= 1;
        }
        Ok(())
    }
    
    fn in_posesh(conn_db: &Connection) -> Result<()> {
        let num_stud = get_student(conn_db)?;
        let num_subj = get_subject(conn_db)?;
        let num_th_learn = get_themes_learn(conn_db, num_subj)?;
        let date = get_input("Введи дату (ГГГГ-ММ-ДД) ");
        let posesh = get_input("Введи присутствие (+,-) ");
        let marks = get_input("Введи оценку (0,2..5 ) ");

        conn_db.execute("
            INSERT INTO 'Посещаемость' (Дата, Предмет, Студент, 
              Присутствие, Оценка, Тема_занятия)
            VALUES (?1,?2,?3,?4,?5,?6)", 
            [
                &date, 
                &num_subj.to_string(), 
                &num_stud.to_string(), 
                &posesh, 
                &marks, 
                &num_th_learn.to_string(),
            ],
        )?;
        Ok(())
    }

    fn in_themes_learn(conn_db: &Connection) -> Result<()> {
        let num_subj = get_subject(conn_db)?;
        let th_learn = get_input("Введи тему занятия ");

        conn_db.execute("
            INSERT INTO 'План_обучения' (Предмет, Тема_занятия)
            VALUES (?1,?2)", 
            [&num_subj.to_string(), &th_learn],
        )?;
        Ok(())
    }

    fn get_student(conn_db: &Connection) -> Result<u32> {
        // Часть1: Достаю группу
        let mut stmt = conn_db.prepare("SELECT Название FROM 'Направления'")?; 
        let rows = stmt.query_map(
            [], 
            |row| {
                Ok(row.get::<_,String>(0)?)
        })?;
        
        println!();
        println!("Группы");
        println!("{:-<6}", "");
        for row in rows {
            let row = row?;
            println!("{}", row);
        }

        println!();
        let groups = get_input("Введи нужную группу ");

        // Часть2: Достаем студента
        let mut stmt = conn_db.prepare("
            SELECT Номер_зачетки, Фамилия, Имя, Отчество 
            FROM 'Cтуденты'
            WHERE Группа = ?1
        ")?;
        let rows = stmt.query_map(
            [&groups],
            |row| {
                Ok((
                    row.get::<_,u32>(0)?,
                    row.get::<_,String>(1)?,
                    row.get::<_,String>(2)?,
                    row.get::<_,String>(3)?,
                ))
        })?;
        
        println!();
        println!(
            "{:<7}|{:<15}|{:<15}|{:<15}",
            "Номер","Фамилия", "Имя", "Отчество"
        );
        println!("{:-<46}", "");

        for row in rows {
            let row = row?;
            println!("{:<7}|{:15}|{:15}|{:15}", row.0, row.1, row.2, row.3);
        }

        println!();
        let num_stud: u32 = get_input("Введи номер студента ").parse()
            .expect("Filed to convert num_stud");

        Ok(num_stud)
    }
    
    fn get_subject(conn_db: &Connection) -> Result<u32> {
        // Часть1: Достаю препода
        let mut stmt = conn_db.prepare("
            SELECT k.id, k.Фамилия, k.Имя, k.Отчество
            FROM 'Кадры' k
            INNER JOIN 'Должность' d ON k.Должность = d.id
            WHERE d.Наименование = 'Преподаватель';
        ")?;
        let rows = stmt.query_map(
            [],
            |row| {
                Ok((
                    row.get::<_,u32>(0)?,
                    row.get::<_,String>(1)?,
                    row.get::<_,String>(2)?,
                    row.get::<_,String>(3)?,
                ))
        })?;

        println!();
        println!(
            "{:4}|{:15}|{:15}|{:15}", 
            "ID", "Фамилия", "Имя", "Отчество"
        );
        println!("{:-<50}", "");

        for row in rows {
            let row = row?;
            println!("{:<4}|{:15}|{:15}|{:15}", row.0, row.1, row.2, row.3);
        }

        println!();
        let num_techer: u32 = get_input("Введи ID ").parse()
            .expect("Filed to convert num_techer in get_subject");
        
        // Часть2: Достаю предмет
        stmt = conn_db.prepare("
            SELECT p.id, p.Наименование 
            FROM 'Предметы' p
            INNER JOIN 'Кадры' k ON k.id = p.Преподаватель
            WHERE k.id = ?1
        ")?;
        let rows = stmt.query_map(
            [&num_techer],
            |row| {
                Ok((
                    row.get::<_,u32>(0)?,
                    row.get::<_,String>(1)?,
                ))
        })?;
        let rows = rows.collect::<Vec<Result<_>>>();

        let rows_count = rows.len();
        let mut num_learn = String::new();
        if rows_count > 1 {
            println!();
            println!("{:4}|{:8}", "ID", "Наименование");
            println!("{:-<13}", "");

            for row in rows {
                let row = row?;
                println!("{:<4}|{:8}", row.0, row.1);
            }
            num_learn = get_input("Введи ID предмета ");
        } else if rows_count == 1 {
            num_learn = rows[0].as_ref().unwrap().0.to_string();
        } else {
            eprintln!("Error: Что то пошло не так");
        }

        let num_learn: u32 = num_learn.parse()
            .expect("Filed to convert num_learn");
        Ok(num_learn)
    }

    fn get_themes_learn(conn_db: &Connection, num_subj: u32) -> Result<u32> {
        let mut stmt = conn_db.prepare("
            SELECT pl.id, pl.Тема_занятия 
            FROM 'План_обучения' pl
            INNER JOIN 'Предметы' pr ON pr.id = ?1
        ")?;
        let rows = stmt.query_map(
            [&num_subj],
            |row| { 
                Ok((
                    row.get::<_,u32>(0)?,
                    row.get::<_,String>(1)?,
                ))
        })?;

        println!();
        println!("{:4}|{:15}", "ID", "Тема");
        println!("{:-<20}", "");

        for row in rows {
            let row = row?;
            println!("{:<4}|{:15}", row.0, row.1);
        }

        let num_th_learn: u32 = get_input("Введи ID ").parse()
            .expect("Filed to convert num_th_learn in get_themes_learn");

        Ok(num_th_learn)
    }
    Ok(())
}
