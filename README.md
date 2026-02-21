# Парсер транзакций YPBank

Библиотека и CLI-утилиты для работы с транзакциями YPBank.

Поддерживает три формата файлов: бинарный, CSV и текстовый.

## Структура

```
src/
├── lib.rs           — точка входа библиотеки
├── parser.rs        — трейт Parser (чтение/запись)
├── storage.rs       — структуры данных (YPBankStorage, YPBankRecord)
├── error.rs         — типы ошибок (CliError, ParserError)
├── cli.rs           — разбор аргументов командной строки
└── format/
    ├── format_bin.rs — бинарный формат
    ├── format_csv.rs — CSV формат
    └── format_txt.rs — текстовый формат

bin/
├── ypbank_converter — конвертация между форматами
└── ypbank_compare  — сравнение файлов в разных форматах
```

## Форматы

Подробнее о форматах — в папке [`docs/`](docs/).

## Запуск

### Конвертация между форматами

```bash
cargo run --bin ypbank_converter -- \
  --input files/records_example.bin \
  --input-format bin \
  --output-format csv \
  [--output out.csv]
```

### Сравнение файлов разных форматов

```bash
cargo run --bin ypbank_compare -- \
  --file1 files/records_example.bin --format1 bin \
  --file2 files/records_example.csv --format2 csv
```

## Тесты

```bash
cargo test
```
