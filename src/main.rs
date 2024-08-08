use std::io::{BufReader, BufWriter, Read, Write};
use std::num::{IntErrorKind};

// Функция получения ввода от пользователя
fn get_input(query: &str) -> String {       // в параметре получаем строку которая ввыводится перед вводом
    print!("{query}");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();       // считываем строку в переменную

    buffer.trim().to_owned()                                // обрезаем лишнее
}

// Функция шифровки и расшифровки
fn process_file_data(data: &Vec<u8>, key: u8) -> Vec<u8> {       // первый параметр данные в векторе байтов, второй ключ шифровки, рашифровки
    let mut processed_data = Vec::with_capacity(data.len());        // вектора с данными, с определнной вместимостью.
    for byte in data {                                              // проходимся по вектору
        processed_data.push(byte ^ key);                            // вставляем данные в новый вектор из ветора с XOR
    }
    processed_data                                                        // возвращаем новый вектор
}

fn main() {
    loop {          // бесконечный цикл
        println!("# # # # # # # # # #");

        let input_file_name = get_input("Enter file name to process: ");        // получаем имя файла
        let input_file = match std::fs::File::open(&input_file_name) {                  // открываем файл
            Ok(file) => file,
            Err(err) => {
                println!("Can't open file \"{input_file_name}\": {err}\n");
                continue                                                                     // продолжаем работу
            }
        };

        let key = match get_input("Enter a key for file encryption/decryption: ")   // получаем ключ шифрования и дешифрования
            .parse::<u8>() {                                                                  // парсим строку в u8 (0..255)
            Ok(key) => key,
            Err(err) => {
                match err.kind() {                                                            // обрабатываем ошибки
                    IntErrorKind::Empty => println!("Key mustn't be empty"),
                    IntErrorKind::InvalidDigit => println!("Enter correct number"),
                    IntErrorKind::PosOverflow => println!("Number must be in range (0; 255]"),
                    _ => println!("Error getting key")
                }
                println!();
                continue                                                                      // продолжаем работу
            }
        };

        // Если ключ равен нулю
        if key == 0 {
            println!("0 is unless key\n");
            continue
        }

        let mut reader = BufReader::new(input_file);        // читаем байты из файла
        let mut input_data = Vec::new();                         // данные для записи в файл, вектор байтов

        if let Err(err) = reader.read_to_end(&mut input_data) {     // ошибка чтения из файла
            println!("Error reading file: {err}\n");
            continue
        }

        let processed_data = process_file_data(&input_data, key);       // получаем шифрованные или дешифрованные данные

        let output_file_name = get_input("Enter file name to output: ");    // получаем имя файла для вывода ш/д файла
        let output_file = match std::fs::File::create(&output_file_name) {          // создаем новый фалй для ш/ф файла
            Ok(file) => file,
            Err(err) => {
                println!("Can't create file \"{output_file_name}\": {err}\n");
                continue
            }
        };

        let mut writer = BufWriter::new(output_file);                       // данные для записи в файл

        if let Err(err) = writer.write_all(&processed_data) {                      // записываем данные в файл
            println!("Error writing to output file: {err}\n");
            continue
        }

        println!("\n");
    }
}