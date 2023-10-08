use std::fs::File;

use csv::{ReaderBuilder, Reader};
use prettytable::{Table, Row, Cell};

use crate::parser::{Node, NodeType};

pub struct Runtime {
    tables_path: String,
    columns: Vec<String>,
    table: String,
}

#[allow(dead_code)]
impl Runtime {
    pub fn new(tables_path: String) -> Self {
        Self { tables_path, columns: Vec::new(), table: String::from("") }
    }

    fn get_reader(&self, table: &str) -> Reader<File> {
        let table_path = format!("{}/{}.csv", self.tables_path, table);

        let file = File::open(table_path)
            .expect("Cannot open csv file.");

        ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file)
    }

    fn print_table(&self, data: Vec<Vec<String>>) {
        let mut table = Table::new();

        for row in &data {
            let mut cells: Vec<Cell> = Vec::new();
            for cell in row {
                cells.push(Cell::new(cell));
            }
            table.add_row(Row::new(cells));
        }

        table.printstd();
    }

    pub fn select(&self) {
        let mut reader = self.get_reader(&self.table[..]);
        let mut data: Vec<Vec<String>> = Vec::new();
        let mut accepted_indexes: Vec<usize> = Vec::new();
        let mut found_columns: Vec<String> = Vec::new();

        let headers = reader.headers().unwrap().to_owned();

        for header in headers.iter() {
            let fields: Vec<String> = header.split(';').map(|x| x.to_string()).collect();
            let mut row: Vec<String> = Vec::new();

            for (index, field) in fields.iter().enumerate() {
                if self.columns.contains(&field) {
                    accepted_indexes.push(index);
                    found_columns.push(field.clone());
                    row.push(field.clone());
                }
            }

            data.push(row);
        }

        if accepted_indexes.len() != self.columns.len() {
            let mut diff = vec![];

            for c in &self.columns {
                if ! found_columns.contains(&c) {
                    diff.push(c);
                }
            }

            if diff.len() != 0 {
                let message = diff.iter().map(|x| x.to_string() + ",").collect::<String>();
                eprintln!("ERROR: Columns [{message}] not found");
                std::process::exit(1);
            }
        }

        for result in reader.records() {
            for record in result.unwrap().iter() {
                let fields: Vec<String> = record.split(';').map(|x| x.to_string()).collect();
                let mut row: Vec<String> = Vec::new();

                for (index, field) in fields.iter().enumerate() {
                    if accepted_indexes.contains(&index) {
                        row.push(field.clone());
                    }
                }
    
                data.push(row);
            }
        }

        self.print_table(data);
    }

    pub fn query(&mut self, node: Node) {
        match node.kind {
            NodeType::ROOT => {
                for children in node.childrens {
                    self.query(children.clone())
                }
            },
            NodeType::SELECT => {
                for children in node.childrens {
                    self.columns.push(children.value.clone());
                }
            },
            NodeType::FROM => {
                self.table = node.childrens[0].value.clone();
            },
            _ => {},
        }
    }

    pub fn execute(&mut self) {
        self.select();
    }
}